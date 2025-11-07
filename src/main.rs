pub mod LogicConfiguration;
pub mod Helpers 
{
    pub mod Logger;
}
pub mod Network
{
    pub mod TcpLaserServer;
}
pub mod Logic 
{
    pub mod ClientInstance;
}
pub mod Protocol 
{
    pub mod Messaging;
    pub mod MessageManager;
}
pub mod Messaging 
{
    pub mod PiranhaMessage;
    pub mod LogicLaserMessageFactory;
    pub mod Messages 
    {
        pub mod Client 
        {
            pub mod LoginMessage;
        }

        pub mod Server 
        {
            pub mod LoginOkMessage;
            pub mod OwnHomeDataMessage;
        }
    }
}

pub mod Datastream 
{
    pub mod ByteStream;
}

use Helpers::Logger::Logger as ServerLogger;
use Network::TcpLaserServer::TcpLaserServer;
use Logic::ClientInstance::ClientInstance;

struct RustBrawl;
impl RustBrawl 
{
    fn main() 
    {
        ServerLogger::Debug("RustBrawl", "main", "Starting Server..");
        ServerLogger::Debug("RustBrawl", "main", "Loading Config..");
        let (Ip, Port) = LogicConfiguration::LoadConfig(LogicConfiguration::Environments::Development);
        ServerLogger::Debug("RustBrawl", "main", format!("Loaded Config at {}:{}", Ip, Port).as_str());
        TcpLaserServer::Listen(Ip, Port);
    }
}

fn main() 
{
    RustBrawl::main()
}