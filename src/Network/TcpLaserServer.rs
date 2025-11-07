use std::net::{TcpListener, TcpStream};
use std::io::{Read, Error};
use std::thread;

use crate::Helpers::Logger::Logger as ServerLogger;
use crate::Logic::ClientInstance::ClientInstance;
use crate::Protocol::Messaging::Messaging;
use crate::Protocol::MessageManager::MessageManager;

pub struct TcpLaserServer;
impl TcpLaserServer {
    pub fn Listen(Ip: &str, Port: i32) -> Result<(), Error> 
    {
        let address = format!("{}:{}", Ip.to_string(), Port);
        let listener = TcpListener::bind(&address)?;
        ServerLogger::Debug("TcpLaserServer", "Listen", &format!("Server started on {}", address));

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    ServerLogger::Debug("TcpLaserServer", "Listen", &format!("New client connected: {}", stream.peer_addr().unwrap()));

                    thread::spawn(move || {
                        TcpLaserServer::HandleClient(stream)
                    });
                }
                Err(e) => {
                    ServerLogger::Error("TcpLaserServer", "Listen", &format!("Failed to accept client: {}", e));
                }
            }
        }

        Ok(())
    }

    fn HandleClient(mut stream: TcpStream) -> Result<(), Error> {
        let mut Client = ClientInstance::new(stream.try_clone().unwrap());
        let mut Header = [0u8; 7];
        let mut HeaderInfo: (i32, u32, i32);

        loop {
            stream.read_exact(&mut Header)?;

            HeaderInfo = Messaging::readHeader(&Header);

            let MsgType = HeaderInfo.0;
            let MsgLength = HeaderInfo.1;
            let MsgVersion = HeaderInfo.2;

            let mut Payload = vec![0u8; MsgLength as usize]; // very productive cast
            stream.read_exact(&mut Payload)?;

            MessageManager::receiveMessage(MsgType, &Payload, &mut Client);
        }
    }
}
