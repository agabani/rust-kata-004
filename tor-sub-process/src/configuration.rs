pub struct Configuration {
    pub hidden_services: Vec<HiddenService>,
}

pub struct HiddenService {
    pub service_directory: String,
    pub service_port: u16,
    pub host_address: String,
    pub host_port: u16,
}
