use std::process::Command;
use std::fs;
use notify_rust::Notification;
use minreq;
use std::collections::HashMap;

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
    Notification::new()
        .summary("酒屋")
        .body(body)
        .icon(icon.get_or_insert(""))
        .timeout(3000)
        .show()
        .unwrap();
}

/// Saves a log file of the stderr/stdout of an application
pub fn log(application_name: &str, output: &str) {
    let log_file = &format!("/tmp/sakaya-{application_name}.log");

    fs::write(log_file, output).unwrap();

    println!("Log file available at {log_file}")
}

/// Sends a request to start an application inside a container
pub fn request(map: &HashMap<&str, &str>) -> Result<(), minreq::Error> {
    // http://192.168.100.49:39493
    let o = minreq::get("http://127.0.0.1:7878").send()?;
    let s = o.as_str()?;
    print!("{}", s);
    Ok(())
}
