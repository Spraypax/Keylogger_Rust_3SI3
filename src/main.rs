mod modules;
use modules::key_manager::get_or_create_key;
use std::io::{self, Write};

fn main() {
    print!(" Entrez votre passphrase : ");
    io::stdout().flush().unwrap();
    
    let mut passphrase = String::new();
    io::stdin().read_line(&mut passphrase).unwrap();
    let passphrase = passphrase.trim();

    let key = get_or_create_key(passphrase);
    println!("✔ Clé chargée avec succès !");
}
