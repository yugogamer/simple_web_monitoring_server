use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
/// System data
/// - hostname: hostname of the server
/// - os: operating system
/// - cpu_name: name of the cpu
/// - cpu_usage_global_usage: global cpu usage
/// - cpu_global_frequency: global cpu frequency
/// - max_memory: max memory of the server
/// - used_memory: used memory of the server
/// - server_uptime: uptime of the server in seconds
/// - cpu_cores: list of cores
/// - temps: list of components
/// - last_update: last update of data
pub struct SystemData {
    pub hostname: Option<String>,
    pub os: Option<String>,
    pub cpu_name: String,
    pub cpu_usage_global_usage: f32,
    pub cpu_global_frequency: u64,
    pub max_memory: u64,
    pub used_memory: u64,
    pub server_uptime: u64,
    pub cpu_cores: Vec<Core>,
    pub temps: Vec<ComponentTemp>,
    pub last_update: u128,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
/// Core
/// - usage: core usage
/// - frequency: core frequency
pub struct Core {
    pub usage: f32,
    pub frequency: u64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
/// ComponentTemp
/// - label: component label
/// - temperature: component temperature
pub struct ComponentTemp {
    pub label: String,
    pub temperature: f32,
}

impl Default for SystemData {
    fn default() -> Self {
        SystemData {
            hostname: None,
            os: None,
            cpu_name: "".to_string(),
            cpu_usage_global_usage: 0.0,
            cpu_global_frequency: 0,
            max_memory: 0,
            used_memory: 0,
            server_uptime: 0,
            cpu_cores: vec![],
            temps: vec![],
            last_update: 0,
        }
    }
}
