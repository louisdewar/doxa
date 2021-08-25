use std::{io, ops::Range};

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use derive_more::{Display, Error, From};

pub(crate) struct Stream<T: AsyncRead + AsyncWrite + Unpin> {
    stream: T,
    buf: [u8; 1024],
    buf_offset: usize,
    buf_end: usize,
}

impl<T: AsyncRead + AsyncWrite + Unpin> Stream<T> {
    pub fn from_socket(stream: T) -> Self {
        Stream {
            stream,
            buf: [0; 1024],
            buf_offset: 0,
            buf_end: 0,
        }
    }

    // This code is a little messy because of some borrowing errors I encountered.

    /// Looks at the data from `buf_offset` to `buf_end` to see whether it needs to return bytes or
    /// EndMessage.
    /// If it needs to return one of these things to avoid borrowing errors it returns a range, if
    /// the range end equals the range start then it is indicating EndMessage otherwise it's the
    /// range of the bytes in the buffer.
    ///
    /// If it decides to do this it will update `buf_offset` accordingly.
    ///
    /// The only case where it returns None is if `buf_offset` equals `buf_end`, i.e. there is no
    /// data remaining in the buffer. In this case it will reset both `buf_offset` and `buf_end`
    /// back to 0.
    fn handle_buf_data<'a>(
        buf: &'a [u8],
        buf_end: &mut usize,
        buf_offset: &mut usize,
    ) -> Option<Range<usize>> {
        if buf_end != buf_offset {
            if let Some(position) = buf[*buf_offset..*buf_end]
                .iter()
                .copied()
                .position(|b| b == b'\0')
            {
                // If the NULL byte is the first char then we've sent the previous bytes of data
                // and now we need to send `EndMessage`
                // NOTE: position is relative to the buf_offset.
                if position == 0 {
                    *buf_offset += 1;
                    return Some(0..0);
                }

                let start = *buf_offset;
                *buf_offset += position;

                return Some(start..*buf_offset);
            }
        }

        *buf_offset = 0;
        *buf_end = 0;

        None
    }

    /// Get the next bytes that are all part of the same message or, if the message has ended then
    /// `EndMessage` will be returned.
    ///
    /// TODO: max_msg_len.
    /// Keep track of message len and if it exceeds value at any point return error and discard
    /// rest of message on subsequent next_part call. probably want to call self.next_part once to
    /// skip the next message then again to get the real message, then have a different public
    /// next_part method that wraps it.
    pub async fn next_part<'a>(
        &'a mut self,
        max_msg_len: usize,
    ) -> Result<MessagePart<'a>, ReadMessageError> {
        if let Some(range) =
            Self::handle_buf_data(&self.buf, &mut self.buf_end, &mut self.buf_offset)
        {
            if range.start == range.end {
                return Ok(MessagePart::EndMessage);
            }

            return Ok(MessagePart::Bytes(&self.buf[range]));
        }

        let n = AsyncReadExt::read(&mut self.stream, &mut self.buf).await?;

        if n == 0 {
            return Err(ReadMessageError::StreamClosed);
        }

        self.buf_end = n;

        if let Some(position) = self.buf[0..n].iter().copied().position(|b| b == b'\0') {
            if position == 0 {
                self.buf_offset = 1;
                return Ok(MessagePart::EndMessage);
            }
            self.buf_offset = position;

            return Ok(MessagePart::Bytes(&self.buf[0..position]));
        }

        self.buf_end = 0;

        Ok(MessagePart::Bytes(&self.buf[0..n]))
    }

    /// Fill the buffer exactly full with bytes
    pub async fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        let mut n_read = 0;

        while n_read < buf.len() {
            let n = AsyncReadExt::read(&mut self.stream, &mut buf[n_read..]).await?;
            n_read += n;
        }

        Ok(())
    }

    /// Reads data from the input stream up to but not including n bytes.
    /// This may read less than that amount but never more.
    /// This will ignore any message separators from the start up to but not beyond n bytes.
    /// If there is already data in the buffer it will be returned first.
    pub async fn read_until_n(&mut self, max: usize) -> io::Result<&[u8]> {
        if self.buf_offset != self.buf_end {
            let start = self.buf_offset;
            let end = self.buf_end;
            self.buf_offset = 0;
            self.buf_end = 0;
            return Ok(&self.buf[start..end]);
        }

        let n_read = AsyncReadExt::read(&mut self.stream, &mut self.buf).await?;

        if n_read == 0 {
            return Err(io::Error::new(
                io::ErrorKind::ConnectionReset,
                "the other socket disconnected",
            ));
        }

        self.buf_end = n_read;
        let end = max.min(self.buf_end);

        self.buf_offset = end;
        return Ok(&self.buf[0..end]);
    }

    pub async fn send_message(&mut self, msg: &[u8], add_terminator: bool) -> io::Result<()> {
        AsyncWriteExt::write_all(&mut self.stream, msg).await?;

        if add_terminator {
            self.stream.write_u8(b'\0').await?;
        }

        Ok(())
    }

    pub fn get_writer(&mut self) -> &mut impl AsyncWrite {
        &mut self.stream
    }

    /// If message parts have already been read from the current message then this will copy the
    /// remaining bytes into the `Vec` and the max_msg_len will be based on the entire message.
    pub async fn next_full_message(
        &mut self,
        buf: &mut Vec<u8>,
        max_msg_len: usize,
    ) -> Result<(), ReadMessageError> {
        loop {
            match self.next_part(max_msg_len).await? {
                MessagePart::Bytes(bytes) => buf.extend_from_slice(bytes),
                MessagePart::EndMessage => {
                    return Ok(());
                }
            }
        }
    }
}

pub enum MessagePart<'a> {
    Bytes(&'a [u8]),
    EndMessage,
}

#[derive(From, Error, Display, Debug)]
pub enum ReadMessageError {
    IO(io::Error),
    StreamClosed,
}
