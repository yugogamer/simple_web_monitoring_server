use std::{sync::Arc, time::SystemTime};

use crate::entity::system::{ComponentTemp, Core, SystemData};
use sysinfo::{ComponentExt, ProcessorExt, System, SystemExt};
use tokio::sync::RwLock;

pub fn get_current_value() -> SystemData {
    let mut sys = System::new_all();
    sys.refresh_all();
    let cpu_global = sys.global_processor_info();

    let mut cores = Vec::new();

    for processor in sys.processors() {
        cores.push(Core {
            usage: processor.cpu_usage(),
            frequency: processor.frequency(),
        })
    }

    let mut temps = Vec::new();

    for component in sys.components() {
        temps.push(ComponentTemp {
            label: component.label().to_string(),
            temperature: component.temperature(),
        })
    }

    let max_memory = sys.total_memory();
    let used_memory = sys.used_memory();

    let hostname = sys.host_name();
    let os = sys.long_os_version();

    let uptime = sys.uptime();

    SystemData {
        hostname,
        os,
        cpu_name: cpu_global.brand().to_string().trim().to_string(),
        cpu_usage_global_usage: cpu_global.cpu_usage(),
        cpu_global_frequency: cpu_global.frequency(),
        max_memory: max_memory,
        used_memory: used_memory,
        server_uptime: uptime,
        cpu_cores: cores,
        temps,
        last_update: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    }
}

pub async fn update_value(value: Arc<RwLock<SystemData>>) {
    loop {
        let mut value_write = value.write().await;
        *value_write = get_current_value();
        drop(value_write);
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
    }
}
