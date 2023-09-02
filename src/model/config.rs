use serde::{Serialize, Deserialize};
use serde_yaml::Error;
use tracing::{info, debug};

use super::{Destination, Service};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    url: String,
    port: u16,
    log_level: String,
    destinations: Vec<Destination>,
}


impl Configuration {
    pub fn new(content: &str) -> Result<Configuration, Error>{
        serde_yaml::from_str(content)
    }
    pub fn get_port(&self) -> u16{
        self.port
    }

    pub fn get_log_level(&self) -> &str{
        &self.log_level
    }

    pub fn get_destination(&self, name: &str) -> Option<Service>{
        for destination in self.destinations.as_slice(){
            debug!("Destination: {}", destination.get_name());
            if destination.get_name() == name{
                return Some(destination.get_service().clone());
            }
        }
        None
    }
}

