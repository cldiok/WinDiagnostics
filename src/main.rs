use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};

mod system;
mod disk;
mod repair;
mod report;
mod ui;

fn main() {

    #[cfg(target_os = "windows")]
    unsafe {
        winapi::um::wincon::SetConsoleOutputCP(65001);
    }


    let tema = ColorfulTheme {
        active_item_prefix: dialoguer::console::style(">".to_string()).for_stderr(),
        ..ColorfulTheme::default()
    };


    loop {

        ui::banner();

        let items = vec![
            "Informacoes do Sistema",
            "Saude do Disco",
            "Reparo do Windows",
            "Exportar Relatorio",
            "Sair"
        ];

        let escolha = Select::with_theme(&tema)
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