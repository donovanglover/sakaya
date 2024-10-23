use std::fs::write;
use home::home_dir;

/// Makes a desktop file for the application
pub fn make_desktop_file(file_name: &str, full_path: &str) {
    let home = home_dir().unwrap();
    let home = home.to_str().unwrap();

    let output_location = &format!("{home}/.local/share/applications/{file_name}.desktop");

    let mut output: String = "[Desktop Entry]".to_owned() + "\n";

    output.push_str("Type=Application\n");
    output.push_str(&("Name=".to_owned() + file_name + "\n"));
    output.push_str(&("Icon=".to_owned() + file_name + "\n"));
    output.push_str(&("Exec=sakaya \"".to_owned() + full_path + "\"\n"));

    let _ = write(output_location, output);
}
