use std::{io, ops::Range};

use futures_util::StreamExt;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use derive_more::{Display, Error, From};

// TODO: In future a potentially better encoding system would be for every message to start with 4 bytes
// indicating the length of the message removing the needs for special separator characters

pub struct Stream<T: AsyncRead + AsyncWrite + Unpin> {
    stream: T,
    buf: [u8; 1024],
    buf_offset: usize,
    buf_end: usize,
    /// How much of the message has been read so far (not including bytes that are in the buffer
    /// after `buf_offset`.
    message_index: usize,
    /// The length of the message. This is sent before each message, if it is `None` then the
    /// system is yet to receive the message header.
    message_len: Option<usize>,
}

#[derive(Debug, Clone)]
enum BufData {
    /// There is a message part ready to be read
    Part(Range<usize>),
    /// The message has just started (we just received the length header)
    MessageStart(usize),
    /// The message has ended. The previous parts of the message have already been returned.
    MessageEnd,
    /// There is no message part in the buffer, there may or may not be a partial or complete header.
    /// No progress can be made until there is more data
    WaitingForData,
}

impl<T: AsyncRead + AsyncWrite + Unpin> Stream<T> {
    pub fn from_socket(stream: T) -> Self {
        Stream {
            stream,
            buf: [0; 1024],
            buf_offset: 0,
            buf_end: 0,
            message_index: 0,
            message_len: None,
        }
    }

    fn process_buf_data(&mut self) -> BufData {
        let unprocessed_bytes = self.buf_end - self.buf_offset;

        if Some(self.message_index) == self.message_len {
            self.message_len = None;
            self.message_index = 0;

            return BufData::MessageEnd;
        }

        if unprocessed_bytes == 0 {
            self.buf_offset = 0;
            self.buf_end = 0;
            return BufData::WaitingForData;
        }

        let message_len = match self.message_len {
            Some(len) => len,
            None => {
                // Need 4 bytes of data to get message len
                if self.message_len.is_none() && unprocessed_bytes < 4 {
                    // There needs to be enough room to read the 4 bytes of the length if there isn't
                    // enough room we copy the bytes we have received to the start of the buffer.
                    // If remaining space is less than the remaining bytes of the header we need more
                    // room.
                    if self.buf.len() - self.buf_end < 4 - unprocessed_bytes {
                        let (start, data) = self.buf.split_at_mut(self.buf_offset);
                        start[0..unprocessed_bytes].copy_from_slice(&data[0..unprocessed_bytes]);
                        self.buf_offset = 0;
                        self.buf_end = unprocessed_bytes;
                    }

                    return BufData::WaitingForData;
                }

                let mut len_bytes = [0; 4];
                len_bytes.copy_from_slice(&self.buf[self.buf_offset..(self.buf_offset + 4)]);
                let message_len = u32::from_be_bytes(len_bytes);
                self.buf_offset += 4;
                self.message_len = Some(message_len as usize);

                assert_eq!(self.message_index, 0);

                return BufData::MessageStart(message_len as usize);
            }
        };

        assert!(message_len >= self.message_index);
        // TODO: maybe not needed (replace with assert) see above
        if message_len == self.message_index {
            // We could only get here for 0 length messages, potentially we disallow those
            self.message_len = None;
            self.message_index = 0;

            assert_eq!(message_len, 0);
            return BufData::MessageEnd;
        }

        let remaining = message_len - self.message_index;

        let start = self.buf_offset;
        let end = self.buf_offset + unprocessed_bytes.min(remaining);
        self.buf_offset = end;

        self.message_index += end - start;

        BufData::Part(start..end)
    }

    /// Get the next bytes that are all part of the same message or, if the message has ended then
    /// `EndMessage` will be returned.
    ///
    /// This method is cancel safe, if this is cancelled then the next call will yield the data
    /// that the cancelled one would have.
    pub async fn next_part(&mut self) -> Result<MessagePart<'_>, ReadPartError> {
        loop {
            match self.process_buf_data() {
                BufData::WaitingForData => {}
                BufData::MessageStart(len) => return Ok(MessagePart::StartMessage { length: len }),
                BufData::MessageEnd => return Ok(MessagePart::EndMessage),
                BufData::Part(range) => return Ok(MessagePart::Bytes(&self.buf[range])),
            }

            let n = AsyncReadExt::read(&mut self.stream, &mut self.buf[self.buf_offset..]).await?;

            if n == 0 {
                return Err(ReadPartError::StreamClosed);
            }

            self.buf_end += n;
        }
    }

    /// Fill the buffer exactly full with bytes from the next message.
    /// If the message has an incorrect length (including if the message has already partly been
    /// read) then this will return an error.
    pub async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), ReadMessageError> {
        let mut n = 0;
        let expected_len = buf.len();

        {
            let part = self.next_part().await?;

            match part {
                MessagePart::StartMessage {
                    length: message_len,
                } => {
                    if message_len != expected_len {
                        return Err(ReadMessageError::IncorrectLength { message_len });
                    }
                }
                MessagePart::Bytes(_bytes) => {
                    return Err(ReadMessageError::MessagePartlyRead);
                }
                MessagePart::EndMessage if expected_len == 0 => return Ok(()),
                MessagePart::EndMessage => {
                    return Err(ReadMessageError::IncorrectLength { message_len: 0 })
                }
            }
        }

        loop {
            let part = self.next_part().await?;
            match part {
                MessagePart::StartMessage { .. } => {
                    panic!("already received start message")
                }
                MessagePart::Bytes(bytes) => {
                    buf[n..bytes.len()].copy_from_slice(bytes);
                    n += bytes.len();
                }
                MessagePart::EndMessage => return Ok(()),
            }
        }
    }

    async fn send_msg_header(&mut self, len: usize) -> io::Result<()> {
        let len: u32 = len as u32;
        self.stream.write_all(&len.to_be_bytes()).await?;

        Ok(())
    }

    /// Sends a full message from a single slice.
    pub async fn send_full_message(&mut self, msg: &[u8]) -> io::Result<()> {
        self.send_msg_header(msg.len()).await?;
        AsyncWriteExt::write_all(&mut self.stream, msg).await?;

        Ok(())
    }

    /// Sends a full message from a single slice and a prefix (sends the prefix first then the
    /// message).
    ///
    /// TODO: change the error type to a stream specific error.
    pub async fn send_prefixed_full_message(
        &mut self,
        prefix: &[u8],
        msg: &[u8],
    ) -> io::Result<()> {
        self.send_msg_header(msg.len() + prefix.len()).await?;

        AsyncWriteExt::write_all(&mut self.stream, prefix).await?;
        AsyncWriteExt::write_all(&mut self.stream, msg).await?;

        Ok(())
    }

    // /// Sends data from an object that implements `AsyncBufRead`.
    // /// You must prespecify the length of the data.
    // /// This will only read `length` bytes from the reader, if there are more they will be ignored.
    // /// If the reader contains too few bytes this will return an error.
    // ///
    // /// This will correctly send the length header before the message so it can be decoded at the
    // /// other end.
    // pub async fn send_reader<R: AsyncBufRead + Unpin>(
    //     &mut self,
    //     reader: &mut R,
    //     length: usize,
    // ) -> Result<(), SendReaderError> {
    //     self.send_msg_header(length).await?;
    //     let n = tokio::io::copy_buf(&mut reader.take(length as u64), &mut self.stream).await?;

    //     if n < length as u64 {
    //         return Err(SendReaderError::SourceTooShort);
    //     }

    //     Ok(())
    // }

    /// Sends data from a stream of bytes with a specified error type.
    /// You must prespecify the length of the data.
    /// This will only read `length` bytes from the stream, if there are more they will be ignored.
    /// If the stream contains too few bytes this will return an error.
    ///
    /// This will correctly send the length header before the message so it can be decoded at the
    /// other end.
    pub async fn send_stream<S: futures_util::Stream<Item = Result<bytes::Bytes, E>> + Unpin, E>(
        &mut self,
        stream: &mut S,
        length: usize,
    ) -> Result<(), SendStreamError<E>> {
        self.send_msg_header(length).await?;

        let mut n = 0;
        while let Some(bytes) = stream.next().await {
            let bytes = bytes.map_err(SendStreamError::Stream)?;
            n += bytes.len();
            self.stream.write_all(&bytes).await?;
        }

        if n < length {
            return Err(SendStreamError::StreamTooShort);
        }

        Ok(())
    }
    /// If necessary it will reallocate the vec.
    /// This will first truncate the Vec to 0 overwriting any previous data.
    ///
    /// If the message is too long `ReadMessageError::IncorrectLength` is returned.
    ///
    /// TODO: consider a cancel safe version of this code
    pub async fn next_full_message(
        &mut self,
        buf: &mut Vec<u8>,
        max_msg_len: usize,
    ) -> Result<(), ReadMessageError> {
        // TODO: use MessageReader - need to figure out a way of having an `OwnedMessageReader` and
        // a `MessageReader` that takes in a reference (maybe never store the reference just take
        // it in on each call)
        buf.truncate(0);

        {
            let part = self.next_part().await?;

            match part {
                MessagePart::StartMessage {
                    length: message_len,
                } => {
                    if message_len > max_msg_len {
                        return Err(ReadMessageError::IncorrectLength { message_len });
                    }

                    if buf.capacity() < message_len {
                        // buf.len() should equal 0 due to the above truncate
                        buf.reserve_exact(message_len);
                    }
                }
                MessagePart::Bytes(_bytes) => {
                    return Err(ReadMessageError::MessagePartlyRead);
                }
                MessagePart::EndMessage => {
                    return Err(ReadMessageError::MessagePartlyRead);
                }
            }
        }

        loop {
            let part = self.next_part().await?;
            match part {
                MessagePart::StartMessage { .. } => {
                    unreachable!("already received start message")
                }
                MessagePart::Bytes(bytes) => buf.extend_from_slice(bytes),
                MessagePart::EndMessage => return Ok(()),
            }
        }
    }

    pub async fn next_message_to_writer<W: AsyncWrite + Unpin>(
        &mut self,
        writer: &mut W,
        max_msg_len: usize,
    ) -> Result<usize, ReadMessageError> {
        let mut n = 0;

        {
            let part = self.next_part().await?;

            match part {
                MessagePart::StartMessage {
                    length: message_len,
                } => {
                    if message_len > max_msg_len {
                        return Err(ReadMessageError::IncorrectLength { message_len });
                    }
                }
                MessagePart::Bytes(_bytes) => {
                    return Err(ReadMessageError::MessagePartlyRead);
                }
                MessagePart::EndMessage => {
                    return Err(ReadMessageError::IncorrectLength { message_len: 0 })
                }
            }
        }

        loop {
            let part = self.next_part().await?;
            match part {
                MessagePart::StartMessage { .. } => {
                    unreachable!("already received start message")
                }
                MessagePart::Bytes(bytes) => {
                    n += bytes.len();
                    writer.write_all(bytes).await?;
                }
                MessagePart::EndMessage => return Ok(n),
            }
        }
    }

    /// Reads the next message checking to see if it matches the provided message.
    /// If it does Ok(()) is returned else an error.
    pub async fn expect_exact_msg(
        &mut self,
        expected_msg: &[u8],
    ) -> Result<(), ExpectMessageError> {
        // TODO: In future this could be optimised to avoid allocation but it probably isn't worth it.
        let mut msg_buf = vec![0; expected_msg.len()];
        self.read_exact(&mut msg_buf).await?;

        if expected_msg != msg_buf {
            return Err(ExpectMessageError::IncorrectMessage {
                received_msg: String::from_utf8_lossy(&msg_buf).to_string(),
                expected: String::from_utf8_lossy(expected_msg).to_string(),
            });
        }

        Ok(())
    }
}

/// A cancellation safe way to a single message
pub struct MessageReader {
    len: Option<usize>,
    max_msg_len: usize,
    msg_buf: Vec<u8>,
    completed: bool,
}

impl MessageReader {
    pub fn new(mut msg_buf: Vec<u8>, max_msg_len: usize) -> Self {
        msg_buf.truncate(0);

        MessageReader {
            len: None,
            max_msg_len,
            msg_buf,
            completed: false,
        }
    }

    pub fn reset(&mut self) {
        let mut msg_buf = Vec::new();

        std::mem::swap(&mut self.msg_buf, &mut msg_buf);

        *self = Self::new(msg_buf, self.max_msg_len);
    }

    /// If this future is cancelled then any read data has been stored in the buf and it is safe to
    /// repeatedly call this method.
    ///
    /// `ReadMessageError::MessagePartlyRead` can only ever be returned on the first invocation of
    /// `read_full_message`.
    /// If data is read between invocations of this function then it may go unnoticed causing
    /// missing chunks of data and then when receiving EndMessage this method will panic when it
    /// detects that the length of the data read does not equal the supposed length of the message.
    ///
    /// If it receives two StartMessage events (without EndMessage) then it also will panic since that can only be
    /// caused by reading data in between invocations.
    ///
    /// Once a full message has been read successfully it will reset itself on the next invocation ready for the
    /// next message.
    pub async fn read_full_message<T: AsyncRead + AsyncWrite + Unpin>(
        &mut self,
        stream: &mut Stream<T>,
    ) -> Result<&[u8], ReadMessageError> {
        if self.completed {
            self.reset();
        }

        let message_len = match self.len {
            Some(len) => len,
            // We're still waiting for the header
            None => {
                let part = stream.next_part().await?;

                match part {
                    MessagePart::StartMessage {
                        length: message_len,
                    } => {
                        if message_len > self.max_msg_len {
                            return Err(ReadMessageError::IncorrectLength { message_len });
                        }

                        if self.msg_buf.capacity() < message_len {
                            // self.msg_buf.len() should equal 0 due to the truncate in Self::new
                            self.msg_buf.reserve_exact(message_len);
                        }

                        self.len = Some(message_len);

                        message_len
                    }
                    MessagePart::Bytes(_bytes) => {
                        return Err(ReadMessageError::MessagePartlyRead);
                    }
                    MessagePart::EndMessage => {
                        return Err(ReadMessageError::IncorrectLength { message_len: 0 })
                    }
                }
            }
        };

        loop {
            let part = stream.next_part().await?;
            match part {
                MessagePart::StartMessage { .. } => {
                    panic!("already received start message")
                }
                MessagePart::Bytes(bytes) => self.msg_buf.extend_from_slice(bytes),
                MessagePart::EndMessage => {
                    if self.msg_buf.len() != message_len {
                        panic!(
                            "final message buf len ({}) did not equal expected message len ({})",
                            self.msg_buf.len(),
                            message_len
                        );
                    }

                    self.completed = true;

                    return Ok(&self.msg_buf);
                }
            }
        }
    }

    pub fn take_buf(self) -> Vec<u8> {
        self.msg_buf
    }
}

pub enum MessagePart<'a> {
    StartMessage { length: usize },
    Bytes(&'a [u8]),
    EndMessage,
}

#[derive(From, Error, Display, Debug)]
pub enum ReadPartError {
    IO(io::Error),
    StreamClosed,
}

#[derive(From, Error, Display, Debug)]
pub enum SendReaderError {
    IO(io::Error),
    SourceTooShort,
}

#[derive(From, Error, Display, Debug)]
pub enum SendStreamError<E> {
    IO(io::Error),
    StreamTooShort,
    #[from(ignore)]
    Stream(E),
}

#[derive(From, Error, Display, Debug)]
pub enum ExpectMessageError {
    #[from(forward)]
    Message(ReadMessageError),
    #[from(ignore)]
    #[display(fmt = "received `{}`, expected `{}`", received_msg, expected)]
    IncorrectMessage {
        received_msg: String,
        expected: String,
    },
}

#[derive(From, Error, Display, Debug)]
pub enum ReadMessageError {
    #[from(forward)]
    Part(ReadPartError),
    IncorrectLength {
        message_len: usize,
    },
    /// Part of the current message has already been read
    MessagePartlyRead,
}
