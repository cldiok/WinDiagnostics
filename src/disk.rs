use std::process::Command;
use colored::*;

pub fn check_disk() {

    println!("{}", "\n=== VERIFICANDO DISCO ===".yellow());

    let output = Command::new("cmd")
        .args([
            "/C",
            "wmic diskdrive get model,status,size"
        ])
        .output();

    match output {

        Ok(result) => {

            println!(
                "{}",
                String::from_utf8_lossy(&result.stdout)
            );
        }

        Err(e) => {
            println!("Erro: {}", e);
        }
    }
}