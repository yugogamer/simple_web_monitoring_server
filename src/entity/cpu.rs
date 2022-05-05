use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct SystemData {
    pub cpu_name: String,
    pub cpu_usage_global_usage: f32,
    pub cpu_global_frequency: u64,
    pub cpu_cores: Vec<Core>,
    pub temps: Vec<ComponentTemp>,
    pub last_update: u128
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Core{
    pub usage : f32,
    pub frequency : u64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ComponentTemp{
    pub label : String,
    pub temperature : f32,
}