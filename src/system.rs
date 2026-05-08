use sysinfo::{System, Disks};
use colored::*;

pub fn show_system_info() {

    let mut sys = System::new_all();

    sys.refresh_all();

    println!("{}", "\n=== INFORMACOES DO SISTEMA ===".green());

    println!(
        "Sistema: {}",
        System::name().unwrap_or("Desconhecido".to_string())
    );

    println!(
        "Kernel : {}",
        System::kernel_version().unwrap_or("Desconhecido".to_string())
    );

    println!(
        "CPU    : {}",
        sys.cpus()[0].brand()
    );

    let ram =
        sys.total_memory() / 1024 / 1024;

    println!("RAM    : {} GB", ram);

    let disks = Disks::new_with_refreshed_list();

    for disk in &disks {

        println!(
            "Disco  : {:?} | {} GB livres",
            disk.name(),
            disk.available_space() / 1024 / 1024 / 1024
        );
    }
}