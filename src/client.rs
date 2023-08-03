use std::process::Command;
use std::fs;
use notify_rust::Notification;
use reqwest::blocking::ClientBuilder;
use std::collections::HashMap;

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

pub fn notify(body: &str, mut icon: Option<&str>) {
    Notification::new()
        .summary("酒屋")
        .body(body)
        .icon(icon.get_or_insert(""))
        .timeout(3000)
        .show()
        .unwrap();
}

pub fn log(application_name: &str, output: &str) {
    let log_file = &format!("/tmp/sakaya-{application_name}.log");

    fs::write(log_file, output).unwrap();

    println!("Log file available at {log_file}")
}

pub fn request(map: &HashMap<&str, &str>) -> String {
    ClientBuilder::new()
        .timeout(None)
        .build()
        .unwrap()
        .post("http://192.168.100.49:39493")
        .json(&map)
        .send()
        .expect("Couldn't request sakaya-server")
        .text()
        .unwrap()
}
