use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpStream, ToSocketAddrs},
};

use crate::{cmd::Command, response::{Response, StatusCode}};

pub struct DictClient {
    stream: TcpStream,
    reader: BufReader<TcpStream>,
    pub header: String,
}

impl DictClient {
    pub fn new(stream: TcpStream) -> Self {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut header = String::new();
        let _ = reader.read_line(&mut header);

        DictClient { stream, reader, header }
    }

    pub fn connect<T: ToSocketAddrs>(addr: T) -> Self {
        let stream = TcpStream::connect(addr).unwrap();
        DictClient::new(stream)
    }



    pub(crate) fn send_str(&mut self, data: &str) {
        self.send_bytes(data.as_bytes());
    }

    pub(crate) fn send_bytes(&mut self, data: &[u8]) {
        let _ = self.stream.write(data);
        let _ = self.stream.flush();
    }

    /// Write a command to the server.
    pub fn send_cmd(&mut self, command: Command) {
        let message = command.to_message();
        self.send_str(&message);
    }

    /// Read a response from the server.
    pub fn read_resp(&mut self) -> Option<Response> {
        let mut line = String::new();
        let mut resp: Option<Response> = if let Ok(v) = self.reader.read_line(&mut line) {
            if v != 0 {
                Some(Response::from_line(&line))
            } 
            else {
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
                        let _ = self.reader.read_line(&mut line);
                        if line.trim() == "." {
                            break;
                        }
                        data.push_str(&line);
                    }
                    resp.content.push(data.clone());
                }
            }
            else if resp.is_multple_data() {
                loop {
                    let mut line = String::new();
                    let _ = self.reader.read_line(&mut line);
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
    pub fn command(&mut self, command: Command) -> Option<Response> {
        self.send_cmd(command);
        self.read_resp()
    }
}
