use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};

mod system;
mod disk;
mod repair;
mod report;
mod benchmark;
mod ui;

fn banner() {
    println!("{}", "========================================".cyan());
    println!("{}", "      PC TOOLKIT - DIAGNOSTICO".cyan());
    println!("{}", "========================================".cyan());
}

fn main() {

    loop {

        ui::banner();

        let items = vec![
            "Informacoes do Sistema",
            "Saude do Disco",
            "Reparo do Windows",
            "Exportar Relatorio",
            "Sair"
        ];

        let escolha = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Escolha uma opcao")
            .items(&items)
            .default(0)
            .interact()
            .unwrap();

        match escolha {

            0 => {
                system::show_system_info();
            }

            1 => {
                disk::check_disk();
            }

            2 => {
                repair::repair_windows();
            }

            3 => {
                report::export_report();
            }

            4 => {
                println!("{}", "Saindo...".red());
                break;
            }

            _ => {}
        }

        println!();
    }
}