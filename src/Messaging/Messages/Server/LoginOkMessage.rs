use crate::Messaging::PiranhaMessage::PiranhaMessage;
use crate::Logic::ClientInstance::ClientInstance;
use crate::Messaging::LogicLaserMessageFactory::LogicLaserMessageFactory;

pub struct LoginOkMessage<'a> 
{
    pub Message: PiranhaMessage<'a>,
}

impl<'a> LoginOkMessage<'a> 
{
    pub fn new(MsgPayload: &[u8], Client: &'a mut ClientInstance) -> Self {
        let mut MessageInstance = PiranhaMessage::new(MsgPayload, Client);

        MessageInstance.setMessageTypeName("LoginOkMessage");
        MessageInstance.setMessageType(20104);
        MessageInstance.setMessageVersion(0);

        Self { Message: MessageInstance }
    }

    pub fn encode(&mut self) -> &mut Self {
        let ClientInst = self.Message.getClientInstance();

        // self.Message.stream.writeLong(&ClientInst.GetHighID(), &ClientInst.GetLowID());
        return self
    }

    pub fn process(&mut self) {
        self.Message.send();
    }
}
