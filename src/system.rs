use sysinfo::{System, Disks};

use crate::ui;

pub fn show_system_info() {

    let mut sys = System::new_all();

    sys.refresh_all();

    ui::section("INFORMAÇÕES DO SISTEMA");

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
        sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

    println!("RAM    : {:.1} GB", ram);

    let disks = Disks::new_with_refreshed_list();

    for disk in &disks {

        println!(
            "Disco  : {:?} | {} GB livres",
            disk.name(),
            disk.available_space() / 1024 / 1024 / 1024
        );
    }
}