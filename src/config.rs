use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "default_port")]
    pub server_port: u16,   
}

fn default_port()->u16 {8000}
