use chrono::Local;
use std::fs::File;
use std::io::Write;

pub fn export_report() {

    let filename = format!(
        "relatorio_{}.txt",
        Local::now().format("%Y%m%d_%H%M%S")
    );

    let mut file = File::create(&filename)
        .unwrap();

    writeln!(
        file,
        "RELATORIO DE DIAGNOSTICO"
    ).unwrap();

    writeln!(
        file,
        "Gerado em: {}",
        Local::now()
    ).unwrap();

    println!("Relatorio salvo: {}", filename);
}