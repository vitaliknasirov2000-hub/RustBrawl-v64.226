use crate::Helpers::Logger::Logger as ServerLogger;
use crate::Logic::ClientInstance::ClientInstance;

use crate::Messaging::Messages::Client::LoginMessage::LoginMessage;
use crate::Messaging::Messages::Server::
{
    LoginOkMessage::LoginOkMessage, OwnHomeDataMessage::OwnHomeDataMessage
};

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
                let mut msg = LoginOkMessage::new(MsgPayload, Client);
                msg.encode();
                msg.process();
            }

            24101 =>
            {
                let mut msg = OwnHomeDataMessage::new(MsgPayload, Client);
                msg.encode();
                msg.process();
            }

            _ => 
            {
                ServerLogger::Warn("LogicLaserMessageFactory", "createMessageByType", &format!("Unhandled Message with Type: {}", MsgType));
            }
        }
    }
}