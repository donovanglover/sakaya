use home::home_dir;
use pelite::{FileMap, PeFile};
use sakaya::notify;
use std::fs;
use std::io::Cursor;
use std::net::SocketAddrV4;
use std::path::PathBuf;

pub fn exec(address: SocketAddrV4, path: &PathBuf, directory: &str) {
    if !path.exists() {
        notify("Exiting since not a valid file.", None);
        return;
    }

    let file_name = path.file_name().unwrap().to_str().unwrap();
    let path = path.canonicalize().unwrap();
    let path = path.to_str().unwrap();

    if path.contains(directory) {
        let container_path = path.replace(directory, "mnt");

        let _home = home_dir().unwrap();
        let home = _home.to_str().unwrap();

        let icon = &format!("{home}/.local/share/icons/{file_name}.png");

        make_icon(path, icon);
        make_desktop_file(&format!("{home}/.local/share/applications/{file_name}.desktop"), file_name, path);
        notify(&format!("Starting {file_name}..."), Some(icon));
        request(address, &container_path).unwrap();
        notify(&format!("Closed {file_name}."), Some(icon));
    }
}

/// Sends a request to start an application inside a container
pub fn request(address: SocketAddrV4, path: &str) -> Result<(), minreq::Error> {
    let response = minreq::get(format!("http://{address}/{path}")).send()?;
    print!("{}", response.as_str()?);
    Ok(())
}

/// Given an .exe file, return the first .ico file inside it
pub fn get_first_ico_file(input_bin: &str) -> Option<Cursor<Vec<u8>>> {
    let map = FileMap::open(input_bin).expect("Error opening the binary");
    let file = PeFile::from_bytes(&map).expect("Error parsing the binary");
    let resources = file
        .resources()
        .expect("Error binary does not have resources");

    if let Some((_, group)) = resources.icons().find_map(Result::ok) {
        let mut contents = Vec::new();
        group.write(&mut contents).unwrap();

        return Some(Cursor::new(contents));
    }

    None
}

/// Given an .ico with multiple images, return the largest one that's a square
pub fn convert_largest_square_image_in_ico_to_png(
    buf: Cursor<Vec<u8>>,
    output_path: &str,
) -> Result<(), std::io::Error> {
    let icondir = ico::IconDir::read(buf).unwrap();
    let mut largest_size = 0;
    let mut i = 0;
    let mut largest_index = 0;

    for image in icondir.entries() {
        let width = image.width();

        if width == image.height() && width > largest_size {
            largest_size = width;
            largest_index = i;
        }

        i = i + 1;
    }

    let image = icondir.entries()[largest_index].decode().unwrap();
    let out_file = fs::File::create(output_path).unwrap();

    image.write_png(out_file)
}

/// Makes an icon for the application
pub fn make_icon(input_bin: &str, output_path: &str) {
    if let Some(icon) = get_first_ico_file(input_bin) {
        let _ = convert_largest_square_image_in_ico_to_png(icon, output_path);
    }
}

/// Makes a desktop file for the application
pub fn make_desktop_file(output_location: &str, file_name: &str, full_path: &str) {
    let mut output: String = "[Desktop Entry]".to_owned() + "\n";
    output.push_str("Type=Application\n");
    output.push_str(&("Name=".to_owned() + file_name + "\n"));
    output.push_str(&("Exec=sakaya \"".to_owned() + full_path + "\"\n"));

    let _ = fs::write(output_location, output);
}
