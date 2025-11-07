use crate::Helpers::Logger::Logger as ServerLogger;
use crate::Logic::ClientInstance::ClientInstance;
use crate::Protocol::MessageManager::MessageManager;
use crate::Datastream::ByteStream::ByteStream;

pub struct PiranhaMessage<'a> 
{
    pub stream: ByteStream,
    pub ClientInst: &'a mut ClientInstance,

    pub MsgTypeName: String,

    pub MsgType: i32,
    pub MsgLength: i32,
    pub MsgVersion: i32,
}

impl<'a> PiranhaMessage<'a>
{
    pub fn new(MsgPayload: &[u8], Client: &'a mut ClientInstance) -> Self
    {
        let mut byteStream = ByteStream::new();
        byteStream.replaceBuffer(MsgPayload.to_vec()); 

        Self {
            stream: byteStream,
            ClientInst: Client,
            MsgTypeName: String::new(),
            MsgType: 0,
            MsgLength: MsgPayload.len() as i32,
            MsgVersion: 0,
        }
    }

    pub fn isServerToClientMessage(MsgType: i32) -> bool
    {
        return MsgType >= 20000;
    }

    pub fn setMessageVersion(&mut self, MsgVersion: i32)
    {
        self.MsgVersion = MsgVersion;
    }

    pub fn setMessageType(&mut self, MsgType: i32) 
    {
        self.MsgType = MsgType;
    }

    pub fn setMessageTypeName(&mut self, MsgTypeName: &str) 
    {
        self.MsgTypeName = MsgTypeName.to_string();
    }

    pub fn getMessageTypeName(&self) -> String 
    {
        return self.MsgTypeName.clone();
    }

    pub fn getMessageType(&self) -> i32
    {
        return self.MsgType;
    }

    pub fn getMessageVersion(&self) -> i32
    {
        return self.MsgVersion;
    }

    pub fn getClientInstance(&mut self) -> &mut ClientInstance
    {
        return self.ClientInst;
    }

    pub fn encode(&mut self)
    {
        
    }

    pub fn decode(&mut self)
    {
        
    }

    pub fn process(&mut self)
    {
        
    }

    pub fn send(&mut self)
    {
        MessageManager::sendMessage(self.getMessageType(), self.stream.getBuffer().len() as i32, self.getMessageVersion(), self.stream.getBuffer(), self.getMessageTypeName(), self.ClientInst);
    }
}