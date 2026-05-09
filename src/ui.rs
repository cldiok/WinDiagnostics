use colored::*;

pub fn banner() {

    println!("{}", "========================================".cyan());

    println!("{}", "               PC TOOLKIT               ".bright_green());

    println!("{}", "========================================".cyan());
}

pub fn section(title: &str) {

    println!(
        "\n{}",
        format!("=== {} ===", title)
            .yellow()
    );
}