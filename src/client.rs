use std::process::Command;
use std::fs;
use minreq;

/// Makes an icon for the application with icoextract
pub fn make_icon(input_path: &str, output_icon: &str) {
    Command::new("icoextract")
        .arg(input_path)
        .arg(output_icon)
        .output()
        .unwrap();
}

/// Makes a desktop file for the application
pub fn make_desktop_file(output_location: &str, file_name: &str, full_path: &str) {
    let mut output: String = "[Desktop Entry]".to_owned() + "\n";
    output.push_str("Type=Application\n");
    output.push_str(&("Name=".to_owned() + file_name + "\n"));
    output.push_str(&("Exec=sakaya \"".to_owned() + full_path + "\"\n"));

    let _ = fs::write(output_location, output);
}

/// Notifies the user of an event
pub fn notify(body: &str, mut icon: Option<&str>) {
    println!("{body}");
    Command::new("dunstify")
        .args(["--icon", icon.get_or_insert("sakaya"), "--timeout", "3000", "酒屋", body])
        .output()
        .unwrap();
}

/// Sends a request to start an application inside a container
pub fn request(path: &str) -> Result<(), minreq::Error> {
    // http://192.168.100.49:39493
    let o = minreq::get(format!("http://127.0.0.1:7878/{path}")).send()?;
    let s = o.as_str()?;
    print!("{}", s);
    Ok(())
}
