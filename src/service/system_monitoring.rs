use std::{sync::Arc, time::SystemTime};

use crate::entity::system::{ComponentTemp, Core, DiskUsageData, ProcessData, SystemData};
use sysinfo::{ComponentExt, ProcessExt, ProcessorExt, System, SystemExt};
use tokio::{select, sync::RwLock};

/// Get system information
/// # Arguments
/// * `system_data` - System
/// # Returns
/// * `SystemData` - System data
pub fn get_current_value(sys: &mut System) -> SystemData {
    sys.refresh_cpu();
    sys.refresh_memory();
    sys.refresh_components();
    sys.refresh_components_list();
    sys.refresh_processes();

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

    let mut processes = Vec::new();
    for (_pid, process) in sys.processes() {
        let disk_usage = process.disk_usage();
        processes.push(ProcessData {
            name: process.name().to_string(),
            disk_usage: DiskUsageData {
                total_written_bytes: disk_usage.total_written_bytes,
                written_bytes: disk_usage.written_bytes,
                total_read_bytes: disk_usage.total_read_bytes,
                read_bytes: disk_usage.read_bytes,
            },
            memory: process.memory(),
            cpu_usage: process.cpu_usage(),
        });
        //println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    }

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
        processes,
    }
}

/// auto update system information
/// # Arguments
/// * `system_data` - System
pub async fn update_value(
    value: Arc<RwLock<SystemData>>,
    mut recv: tokio::sync::oneshot::Receiver<bool>,
) {
    let mut sys = System::new_all();
    sys.refresh_all();
    loop {
        select! {
            _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
                let mut value_write = value.write().await;
                *value_write = get_current_value(&mut sys);
            }
            _ = &mut recv=>{
                break;
            }
        }
    }
    println!("sysinfo stopped");
}
