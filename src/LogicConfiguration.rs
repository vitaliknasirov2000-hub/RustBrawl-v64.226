use colored::Colorize;
use crate::Helpers::Logger::Logger as ServerLogger;

pub struct Production;
impl Production {
    pub const Ip: &'static str = "127.0.0.1";
    pub const Port: i32 = 9339;
}

pub struct Stage;
impl Stage {
    pub const Ip: &'static str = "127.0.0.1";
    pub const Port: i32 = 9339;
}

pub struct Integration;
impl Integration {
    pub const Ip: &'static str = "127.0.0.1";
    pub const Port: i32 = 9339;
}

pub struct Development;
impl Development {
    pub const Ip: &'static str = "127.0.0.1";
    pub const Port: i32 = 9339;
}

#[derive(Debug)]
pub enum Environments {
    Production,
    Stage,
    Integration,
    Development,
}

pub fn LoadConfig(Environment: Environments) -> (&'static str, i32) {
    let (Ip, Port) = match Environment {
        Environments::Production => (Production::Ip, Production::Port),
        Environments::Stage => (Stage::Ip, Stage::Port),
        Environments::Integration => (Integration::Ip, Integration::Port),
        Environments::Development => (Development::Ip, Development::Port),
    };

    return (Ip, Port);
}