use crate::Messaging::PiranhaMessage::PiranhaMessage;
use crate::Logic::ClientInstance::ClientInstance;
use crate::Messaging::LogicLaserMessageFactory::LogicLaserMessageFactory;

pub struct LoginMessage<'a> {
    pub Message: PiranhaMessage<'a>,
}

impl<'a> LoginMessage<'a> {
    pub fn new(MsgPayload: &[u8], Client: &'a mut ClientInstance) -> Self {
        let mut MessageInstance = PiranhaMessage::new(MsgPayload, Client);

        MessageInstance.setMessageTypeName("LoginMessage");
        MessageInstance.setMessageType(10101);
        MessageInstance.setMessageVersion(0);

        Self { Message: MessageInstance }
    }

    pub fn decode(&mut self) -> &mut Self {
        let HighID = self.Message.stream.readInt();
        let LowID = self.Message.stream.readInt();
        let PassToken = self.Message.stream.readString();

        let ClientMajor = self.Message.stream.readInt();
        let ClientMinor = self.Message.stream.readInt();
        let ClientBuild = self.Message.stream.readInt();
        let ResourceSha = self.Message.stream.readString();

        let Device = self.Message.stream.readString();
        let PreferredLanguage = self.Message.stream.readDataReference();
        let PreferredDeviceLanguage = self.Message.stream.readString();
        let OSVersion = self.Message.stream.readString();
        let Android = self.Message.stream.readBoolean();

        let ClientInstance = self.Message.getClientInstance();

        ClientInstance.SetHighID(HighID);
        ClientInstance.SetLowID(LowID);
        ClientInstance.SetPassToken(PassToken);

        ClientInstance.SetClientMajor(ClientMajor);
        ClientInstance.SetClientMinor(ClientMinor);
        ClientInstance.SetClientBuild(ClientBuild);
        ClientInstance.SetResourceSha(ResourceSha);

        ClientInstance.SetDevice(Device);
        ClientInstance.SetPreferredLanguage(PreferredLanguage.to_vec());
        ClientInstance.SetPreferredDeviceLanguage(PreferredDeviceLanguage);
        ClientInstance.SetOSVersion(OSVersion);
        ClientInstance.SetAndroid(Android);

        return self
    }

    pub fn process(&mut self) {
        LogicLaserMessageFactory::createMessageByType(24101, &[], self.Message.getClientInstance());
    }
}
