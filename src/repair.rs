use std::process::Command;
use colored::*;

use crate::ui;

pub fn repair_windows() {

    ui::section("REPARO DO WINDOWS");

    println!("Executando SFC...");

    Command::new("cmd")
        .args(["/C", "sfc /scannow"])
        .status()
        .unwrap();

    println!("Executando DISM...");

    Command::new("cmd")
        .args([
            "/C",
            "DISM /Online /Cleanup-Image /RestoreHealth"
        ])
        .status()
        .unwrap();

    println!("Executando CHKDSK...");

    Command::new("cmd")
        .args([
            "/C",
            "chkdsk C: /scan"
        ])
        .status()
        .unwrap();

    println!("{}", "\nReparo concluido.".green());
}