use chrono::Local;
use std::fs::File;
use std::io::Write;
use sysinfo::{System, Disks};


pub fn export_report() {

    let filename = format!(
        "relatorio_{}.txt",
        Local::now().format("%Y%m%d_%H%M%S")
    );

    let mut file = File::create(&filename).unwrap();

    let mut sys = System::new_all();
    sys.refresh_all();

    writeln!(
        file,
        "RELATORIO DE DIAGNOSTICO"
    ).unwrap();

    writeln!(
        file,
        "Gerado em: {}",
        Local::now()
    ).unwrap();

    writeln!(file, "\n--- SISTEMA ---").unwrap();
    writeln!(file, "Sistema: {}", System::name().unwrap_or("Desconhecido".to_string())).unwrap();
    writeln!(file, "Kernel : {}", System::kernel_version().unwrap_or("Desconhecido".to_string())).unwrap();
    writeln!(file, "CPU    : {}", sys.cpus()[0].brand()).unwrap();

    let ram = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0 ;
    let ram_usada = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    writeln!(file,"RAM  : {:.1} GB total | {:.1} GB usada", ram, ram_usada).unwrap();

    writeln!(file, "\n--- DISCOS ---").unwrap();
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        writeln!(
            file,
            "Disco : {:?} | {} GB livres",
            disk.name(),
            disk.available_space() / 1024 / 1024 / 1024
        ).unwrap();
    }

    println!("Relatorio salvo: {}", filename);
}