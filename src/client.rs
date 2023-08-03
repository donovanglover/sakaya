use std::process::Command;
use std::fs;

pub fn make_icon(input_path: &str, output_icon: &str) {
    Command::new("icoextract")
        .arg(input_path)
        .arg(output_icon)
        .output()
        .expect("failed to execute process");
}

pub fn make_desktop_file(output_location: &str, file_name: &str, full_path: &str) {
    let mut output: String = "[Desktop Entry]".to_owned() + "\n";
    output.push_str("Type=Application\n");
    output.push_str(&("Name=".to_owned() + file_name + "\n"));
    output.push_str(&("Exec=sakaya \"".to_owned() + full_path + "\"\n"));

    let _ = fs::write(output_location, output);
}
