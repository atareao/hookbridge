use serde::{Serialize, Deserialize};
use tracing::info;

use super::Service;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Destination {
    name: String,
    service: Service,
}

impl Destination{
    pub fn get_name(&self) -> &str{
        info!("get_name");
        &self.name
    }
    pub fn get_service(&self) -> &Service{
        info!("get_service");
        &self.service
    }
}
