use std::fs;
use pelite::{FileMap, PeFile};

/// Checks if we're inside a container
pub fn is_container() -> bool {
    fs::read("/run/systemd/container").is_ok()
}

/// Given an .exe file, return the ico files inside it
pub fn get_ico_files(input_bin: &str) {
    let map = FileMap::open(input_bin).expect("Error opening the binary");
    let file = PeFile::from_bytes(&map).expect("Error parsing the binary");
    let resources = file.resources().expect("Error binary does not have resources");

    for (name, group) in resources.icons().filter_map(Result::ok) {
        let mut contents = Vec::new();
        group.write(&mut contents).unwrap();

        fs::write(format!("{name}.ico"), contents).unwrap();

        let file = fs::File::open(format!("{name}.ico")).unwrap();
        let icondir = ico::IconDir::read(file).unwrap();
        let image = icondir.entries()[3].decode().unwrap();
        let out_file = fs::File::create(format!("{name}.png")).unwrap();

        image.write_png(out_file).unwrap();

        return;
    }
}
