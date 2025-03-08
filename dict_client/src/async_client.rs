#![cfg(feature = "async")]

use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::io::BufWriter;
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::net::ToSocketAddrs;

use crate::{
    cmd::Command,
    response::{Response, StatusCode},
};

pub struct AsyncDClient {
    pub reader: BufReader<OwnedReadHalf>,
    pub writer: BufWriter<OwnedWriteHalf>,
    pub header: String,
}

impl AsyncDClient {
    pub async fn new(stream: TcpStream) -> Self {
        let (read_half, write_half) = stream.into_split();
        let mut reader = BufReader::new(read_half);
        let writer = BufWriter::new(write_half);

        let mut header = String::new();
        let _ = reader.read_line(&mut header).await;

        AsyncDClient { reader, writer, header }
    }

    pub async fn connect<T: ToSocketAddrs>(addr: T) -> Self {
        let stream = TcpStream::connect(addr).await.unwrap();
        AsyncDClient::new(stream).await
    }

    pub(crate) async fn send_str(&mut self, data: &str) {
        self.send_bytes(data.as_bytes()).await;
    }

    pub(crate) async fn send_bytes(&mut self, data: &[u8]) {
        let _ = self.writer.write(data).await;
        let _ = self.writer.flush().await;
    }

    /// Write a command to the server.
    pub async fn send_cmd(&mut self, command: Command) {
        let message = command.to_message();
        self.send_str(&message).await;
    }

    /// Read a response from the server.
    pub async fn read_resp(&mut self) -> Option<Response> {
        let mut line = String::new();
        let mut resp: Option<Response> = if let Ok(v) = self.reader.read_line(&mut line).await {
            if v != 0 {
                Some(Response::from_line(&line))
            } else {
                None
            }
        } else {
            None
        };

        // If the response is None, return None
        if resp.is_none() {
            return None;
        }
        // If the response is Some, check if it is a multiple data response
        if let Some(ref mut resp) = resp {
            let mut data = String::new();
            if resp.code() == StatusCode::DefinitionsRetrieved {
                for _ in 0..resp.count() {
                    loop {
                        let mut line = String::new();
                        let _ = self.reader.read_line(&mut line).await;
                        if line.trim() == "." {
                            break;
                        }
                        data.push_str(&line);
                    }
                    resp.content.push(data.clone());
                }
            } else if resp.is_multple_data() {
                loop {
                    let mut line = String::new();
                    let _ = self.reader.read_line(&mut line).await;
                    if line.trim() == "." {
                        break;
                    }
                    data.push_str(&line);
                }
                if !data.is_empty() {
                    resp.content.push(data);
                }
            }
        }
        resp
    }

    /// Send a command to the server and read the response.
    pub async fn command(&mut self, command: Command) -> Option<Response> {
        self.send_cmd(command).await;
        self.read_resp().await
    }
}
