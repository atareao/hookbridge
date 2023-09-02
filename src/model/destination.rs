use serde::{Serialize, Deserialize};

use super::Service;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Destination {
    name: String,
    service: Service,
}

impl Destination{
    pub fn get_name(&self) -> &str{
        &self.name
    }
    pub fn get_service(&self) -> &Service{
        &self.service
    }
}
