use clap::Parser;
use std::collections::HashMap;
use std::path::Path;
use reqwest::blocking::ClientBuilder;

#[derive(Parser)]
#[command(version)]
#[command(arg_required_else_help = true)]
struct Cli {
    /// Path to the executable to run.
    executable: String,
}

fn main() {
    let cli = Cli::parse();

    let path = Path::new(&cli.executable);

    if path.exists() {
        let full_path = path.canonicalize().unwrap();
        let full_path_str = full_path.to_str().expect("Couldn't convert to str");

        // TODO: Don't hardcode this?
        if full_path_str.contains("/home/user/containers/wine") {
            let container_path = full_path_str.replace("/home/user/containers/wine", "/mnt");
            let path_str = path.to_str().expect("Couldn't convert to str");

            println!("Running {} as {}...", path_str, container_path);

            let mut map = HashMap::new();
            map.insert("wine", "");
            map.insert("path", &container_path);

            let client = ClientBuilder::new().timeout(None).build().unwrap();
            let result = client.post("http://192.168.100.49:39493").json(&map).send().expect("Couldn't request sakaya-server").text();

            println!("{:?}", result)
        } else {
            // TODO: Get rid of else statements
            println!("File is NOT in path")
        }
    }
}
