use std::net::TcpStream;

pub struct ClientInstance {
    pub Stream: TcpStream,

    HighID: i32,
    LowID: i32,
    PassToken: String,

    ClientMajor: i32,
    ClientMinor: i32,
    ClientBuild: i32,
    ClientVersion: String,

    ResourceSha: String,
    Device: String,
    PreferredLanguage: Vec<i32>,
    PreferredDeviceLanguage: String,
    OSVersion: String,
    IsAndroid: bool,
}

impl ClientInstance {
    pub fn new(Stream: TcpStream) -> Self {
        ClientInstance {
            Stream,
            HighID: 0,
            LowID: 0,
            PassToken: String::new(),
            ClientMajor: 0,
            ClientMinor: 0,
            ClientBuild: 0,
            ClientVersion: String::new(),
            ResourceSha: String::new(),
            Device: String::new(),
            PreferredLanguage: Vec::new(),
            PreferredDeviceLanguage: String::new(),
            OSVersion: String::new(),
            IsAndroid: false,
        }
    }

    pub fn Save(&self) {
        // DatabaseManager::SaveClient(self);
    }

    pub fn GetStream(&self) -> &TcpStream { &self.Stream }
    pub fn SetStream(&mut self, Stream: TcpStream) { self.Stream = Stream; }

    pub fn GetHighID(&self) -> i32 { self.HighID }
    pub fn SetHighID(&mut self, HighID: i32) { self.HighID = HighID; self.Save(); }

    pub fn GetLowID(&self) -> i32 { self.LowID }
    pub fn SetLowID(&mut self, LowID: i32) { self.LowID = LowID; self.Save(); }

    pub fn GetPassToken(&self) -> &str { &self.PassToken }
    pub fn SetPassToken(&mut self, PassToken: String) { self.PassToken = PassToken; self.Save(); }

    pub fn GetClientMajor(&self) -> i32 { self.ClientMajor }
    pub fn SetClientMajor(&mut self, ClientMajor: i32) { self.ClientMajor = ClientMajor; self.Save(); }

    pub fn GetClientMinor(&self) -> i32 { self.ClientMinor }
    pub fn SetClientMinor(&mut self, ClientMinor: i32) { self.ClientMinor = ClientMinor; self.Save(); }

    pub fn GetClientBuild(&self) -> i32 { self.ClientBuild }
    pub fn SetClientBuild(&mut self, ClientBuild: i32) { self.ClientBuild = ClientBuild; self.Save(); }

    pub fn GetClientVersion(&self) -> &str { &self.ClientVersion }
    pub fn SetClientVersion(&mut self, ClientVersion: String) { self.ClientVersion = ClientVersion; self.Save(); }

    pub fn GetResourceSha(&self) -> &str { &self.ResourceSha }
    pub fn SetResourceSha(&mut self, ResourceSha: String) { self.ResourceSha = ResourceSha; self.Save(); }

    pub fn GetDevice(&self) -> &str { &self.Device }
    pub fn SetDevice(&mut self, Device: String) { self.Device = Device; self.Save(); }

    pub fn GetPreferredLanguage(&self) -> &Vec<i32> { &self.PreferredLanguage }
    pub fn SetPreferredLanguage(&mut self, PreferredLanguage: Vec<i32>) { self.PreferredLanguage = PreferredLanguage; self.Save(); }

    pub fn GetPreferredDeviceLanguage(&self) -> &str { &self.PreferredDeviceLanguage }
    pub fn SetPreferredDeviceLanguage(&mut self, PreferredDeviceLanguage: String) { self.PreferredDeviceLanguage = PreferredDeviceLanguage; self.Save(); }

    pub fn GetOSVersion(&self) -> &str { &self.OSVersion }
    pub fn SetOSVersion(&mut self, OSVersion: String) { self.OSVersion = OSVersion; self.Save(); }

    pub fn IsAndroid(&self) -> bool { self.IsAndroid }
    pub fn SetAndroid(&mut self, IsAndroid: bool) { self.IsAndroid = IsAndroid; self.Save(); }
}
