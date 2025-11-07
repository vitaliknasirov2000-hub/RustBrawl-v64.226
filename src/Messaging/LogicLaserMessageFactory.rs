use crate::Helpers::Logger::Logger as ServerLogger;
use crate::Logic::ClientInstance::ClientInstance;

use crate::Messaging::Messages::Client::LoginMessage::LoginMessage;
use crate::Messaging::Messages::Server::LoginOkMessage;
use crate::Messaging::Messages::Server::OwnHomeDataMessage;

pub struct LogicLaserMessageFactory;
impl LogicLaserMessageFactory 
{
    pub fn createMessageByType(MsgType: i32, MsgPayload: &[u8], Client: &mut ClientInstance) 
    {
        match MsgType 
        {
            10101 =>
            {
                let mut msg = LoginMessage::new(MsgPayload, Client);
                msg.decode();
                msg.process();
                // ServerLogger::Warn("LogicLaserMessageFactory", "createMessageByType", "msg decoded");
            }

            20104 =>
            {
                // LoginOkMessage::new(MsgPayload, Client).encode().process();
            }

            24101 =>
            {
                // OwnHomeDataMessage::new(MsgPayload, Client).encode().process();
            }

            _ => 
            {
                ServerLogger::Warn("LogicLaserMessageFactory", "createMessageByType", &format!("Unhandled Message with Type: {}", MsgType));
            }
        }
    }
}