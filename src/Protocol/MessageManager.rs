use std::io::{Write, Result};
use crate::Helpers::Logger::Logger as ServerLogger;
use crate::Logic::ClientInstance::ClientInstance;
use crate::Protocol::Messaging::Messaging;
use crate::Messaging::LogicLaserMessageFactory::LogicLaserMessageFactory;

pub struct MessageManager;
impl MessageManager 
{
    pub fn receiveMessage(MsgType: i32, MsgPayload: &[u8], Client: &mut ClientInstance) 
    {
        ServerLogger::Debug("MessageManager", "receiveMessage", &format!("Received Message with Type: {}", MsgType));
        LogicLaserMessageFactory::createMessageByType(MsgType, MsgPayload, Client);
    }

    pub fn sendMessage(MsgType: i32, MsgLength: i32, MsgVersion: i32, MsgPayload: &[u8], MsgTypeName: String, Client: &mut ClientInstance) 
    {
        let mut MessageBuffer = vec![0u8; MsgPayload.len() + 7];

        Messaging::writeHeader(&mut MessageBuffer, MsgType, MsgLength as u32, MsgVersion);
        MessageBuffer[7..].copy_from_slice(MsgPayload);

        let mut stream = Client.GetStream();
        stream.write_all(&MessageBuffer);
        stream.flush();

        ServerLogger::Debug("MessageManager", "sendMessage", &format!("Sent {}! (Type: {})", MsgTypeName, MsgType));
    }
}