use crate::cli::Cli;
use clap::Parser;
use home::home_dir;
use pelite::{FileMap, PeFile};
use sakaya::notify;
use std::fs;
use std::io::Cursor;
use std::net::SocketAddrV4;
use std::path::Path;
use urlencoding::encode;

/// https://github.com/MicrosoftDocs/win32/blob/docs/desktop-src/Debug/pe-format.md#machine-types
const IMAGE_FILE_MACHINE_I386: u16 = 0x14C;
const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;

/// Run an executable inside the container from the host by requesting
/// the server on a given socket address
pub fn exec(address: SocketAddrV4, path: &Path, directory: &str) {
    if !path.exists() {
        notify("Exiting since not a valid file.", None);
        return;
    }

    let file_name = path.file_name().unwrap().to_str().unwrap();
    let path = path.canonicalize().unwrap();
    let path = path.to_str().unwrap();

    if path.contains(directory) {
        let container_path = path.replace(directory, "mnt");
        let icon = make_icon(path, file_name);

        let Cli { wine32, wine64, .. } = Cli::parse();
        let wine_prefix = match get_target_machine(path) {
            32 => &wine32,
            64 => &wine64,
            _ => "",
        };

        if wine_prefix.is_empty() {
            notify(
                "Exiting since 32/64-bit could not be determined. Please report this issue.",
                Some(&icon),
            );
            return;
        }

        make_desktop_file(file_name, path);

        notify(&format!("Starting {file_name}..."), Some(&icon));

        if request(address, &container_path, wine_prefix).is_ok() {
            notify(&format!("Closed {file_name}."), Some(&icon));
        } else {
            notify(&format!("Error: sakaya server is not accessible."), Some(&icon));
        }
    }
}

/// Sends a request to start an application inside a container
pub fn request(address: SocketAddrV4, path: &str, wine_prefix: &str) -> Result<(), minreq::Error> {
    let path = encode(path);
    let wine_prefix = encode(wine_prefix);
    let response = minreq::get(format!("http://{address}/{path}//{wine_prefix}")).send()?;
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

/// Gets whether the exe is 32 or 64-bit
pub fn get_target_machine(input_bin: &str) -> u8 {
    let map = FileMap::open(input_bin).expect("Error opening the binary");
    let file = PeFile::from_bytes(&map).expect("Error parsing the binary");
    let target_machine = file.file_header().Machine;

    match target_machine {
        IMAGE_FILE_MACHINE_I386 => 32,
        IMAGE_FILE_MACHINE_AMD64 => 64,
        _ => 0,
    }
}

/// Given an .ico with multiple images, return the largest one that's a square
pub fn convert_largest_square_image_in_ico_to_png(
    buf: Cursor<Vec<u8>>,
    output_path: &str,
) -> Result<(), std::io::Error> {
    let icondir = ico::IconDir::read(buf).unwrap();
    let mut largest_size = 0;
    let mut largest_index = 0;

    for (i, image) in icondir.entries().iter().enumerate() {
        let width = image.width();

        if width == image.height() && width > largest_size {
            largest_size = width;
            largest_index = i;
        }
    }

    let image = icondir.entries()[largest_index].decode().unwrap();
    let out_file = fs::File::create(output_path).unwrap();

    image.write_png(out_file)
}

/// Makes an icon for the application
pub fn make_icon(input_bin: &str, file_name: &str) -> String {
    let home = home_dir().unwrap();
    let home = home.to_str().unwrap();

    let output_path = &format!("{home}/.local/share/icons/{file_name}.png");

    if let Some(icon) = get_first_ico_file(input_bin) {
        let _ = convert_largest_square_image_in_ico_to_png(icon, output_path);
    }

    output_path.to_string()
}

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

    let _ = fs::write(output_location, output);
}
