use std::{io, path::PathBuf, str::FromStr};

use tokio::{
    self,
    fs::{File, OpenOptions},
    io::AsyncWriteExt,
    process::{self, Child},
    task::{self, JoinHandle},
};

use tokio_vsock::VsockStream;

use crate::stream::{MessagePart, ReadMessageError, Stream};

use derive_more::{Display, Error, From};

// /// Allow a 50 mb agent
// pub const MAX_MSG_LEN: usize = 50_000_000;

/// An upper bound on the agent size for sanity reasons, measured in bytes
pub const MAX_AGENT_SIZE: usize = 50_000_000;

pub const MAX_FILE_NAME_LEN: usize = 300;

#[derive(Debug, Error, From, Display)]
enum ReceieveAgentError {
    IO(io::Error),
    InvalidFormatting,
    ExtractError,
    ReadMessagePartError(ReadMessageError),
}

/// This is the server that runs inside of the VM.
pub struct VMExecutor {
    child_process: Child,
}

impl VMExecutor {
    pub fn start(cid: u32, port: u32) -> JoinHandle<()> {
        task::spawn(async move {
            let stream = VsockStream::connect(cid, port)
                .await
                .expect("Couldn't connect to stream");

            println!("VM executor connected");
            let mut stream = Stream::from_socket(stream);
            let output_dir = PathBuf::from_str("/tmp/doxa_executor").unwrap();
            tokio::fs::create_dir_all(&output_dir).await.unwrap();

            Self::receive_agent(&mut stream, output_dir)
                .await
                .expect("Failed to receive agent");
        })
    }

    /// Download the agent to `{output_dir}/download/agent_name.tar[.gz]`
    /// Then extract the tar file to `{output_dir}/agent` and delete the downloaded tar.
    async fn receive_agent(
        stream: &mut Stream<VsockStream>,
        output_dir: PathBuf,
    ) -> Result<(), ReceieveAgentError> {
        // == Length message
        let mut len_msg = [0; 9];

        stream.read_exact(&mut len_msg).await?;

        // F for File
        if len_msg[0] != b'F' {
            println!("Invalid message char {} for file len", len_msg[0]);
            return Err(ReceieveAgentError::InvalidFormatting);
        }

        let mut length_bytes = [0_u8; 8];
        length_bytes.copy_from_slice(&len_msg[1..]);

        let file_len = u64::from_be_bytes(length_bytes) as usize;

        // == Name message
        let mut name_msg = Vec::with_capacity(100);
        stream
            .next_full_message(&mut name_msg, MAX_FILE_NAME_LEN)
            .await?;

        // N for Name
        if name_msg[0] != b'N' {
            println!("Invalid message char {} for name", name_msg[0]);
            return Err(ReceieveAgentError::InvalidFormatting);
        }

        let name = String::from_utf8_lossy(&name_msg[1..]);

        let download_location = output_dir.join("download").join(name.as_ref());

        tokio::fs::create_dir_all(&output_dir.join("download")).await?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&download_location)
            .await?;

        println!(
            "Beginning download of agent {} ({} bytes total)",
            name, file_len
        );
        // == File data
        let mut current_len = 0;
        while current_len < file_len {
            let buf = stream.read_until_n(file_len - current_len).await?;
            file.write_all(&buf).await?;

            println!(
                "receieved {} bytes ({:.2}%)...",
                buf.len(),
                (100 * current_len) as f64 / file_len as f64
            );

            current_len += buf.len();
        }

        println!("Downloaded {} bytes", file_len);

        // Reuse name_msg to avoid extra allocations
        name_msg.truncate(0);
        let mut file_end_msg = name_msg;

        stream.next_full_message(&mut file_end_msg, 9).await?;

        if &file_end_msg != "FILE ENDS".as_bytes() {
            return Err(ReceieveAgentError::InvalidFormatting);
        }

        let agent_output = output_dir.join("agent");
        tokio::fs::create_dir_all(&agent_output).await?;

        let mut tar_process = process::Command::new("tar")
            .args(&[
                "xf",
                download_location.to_str().unwrap(),
                &format!("--directory={}", agent_output.to_str().unwrap()),
            ])
            .spawn()
            .expect("Couldn't spawn tar");

        let status = tar_process.wait().await;
        let status = status?;
        if !status.success() {
            return Err(ReceieveAgentError::ExtractError);
        }

        stream.send_message(b"RECEIVED", false).await?;

        Ok(())
    }
}
