use sysinfo::System;
use colored::*;
use std::{thread, time::Duration};

pub fn run_benchmark() {

    println!("{}", "\n=== BENCHMARK BASICO ===".blue());

    let mut sys = System::new_all();

    println!("Monitorando CPU por 5 segundos...\n");

    for i in 1..=5 {

        sys.refresh_cpu();

        let cpu =
            sys.global_cpu_info().cpu_usage();

        println!(
            "Segundo {} -> CPU: {:.2}%",
            i,
            cpu
        );

        thread::sleep(Duration::from_secs(1));
    }

    sys.refresh_memory();

    let total_ram =
        sys.total_memory() / 1024 / 1024;

    let used_ram =
        sys.used_memory() / 1024 / 1024;

    println!("\nRAM TOTAL : {} GB", total_ram);

    println!("RAM USADA : {} GB", used_ram);

    println!(
        "{}",
        "\nBenchmark concluido.".green()
    );
}