use std::fs;
use std::io::Cursor;
use pelite::{FileMap, PeFile};

/// Checks if we're inside a container
pub fn is_container() -> bool {
    fs::read("/run/systemd/container").is_ok()
}

/// Given an .exe file, return the first .ico file inside it
pub fn get_first_ico_file(input_bin: &str) -> Option<Cursor<Vec<u8>>> {
    let map = FileMap::open(input_bin).expect("Error opening the binary");
    let file = PeFile::from_bytes(&map).expect("Error parsing the binary");
    let resources = file.resources().expect("Error binary does not have resources");

    for (_, group) in resources.icons().filter_map(Result::ok) {
        let mut contents = Vec::new();
        group.write(&mut contents).unwrap();

        return Some(Cursor::new(contents));
    }

    None
}

/// Given an .ico with multiple images, return the largest one that's a square
pub fn convert_largest_square_image_in_ico_to_png(buf: Cursor<Vec<u8>>) {
    let icondir = ico::IconDir::read(buf).unwrap();
    let image = icondir.entries()[3].decode().unwrap();
    let out_file = fs::File::create(format!("result.png")).unwrap();

    image.write_png(out_file).unwrap();
}
