use pelite::{FileMap, PeFile};
use std::fs::File;
use std::io::Cursor;

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
    let icondir = ico::IconDir::read(buf)?;
    let mut largest_size = 0;
    let mut largest_index = 0;

    for (i, image) in icondir.entries().iter().enumerate() {
        let width = image.width();

        if width == image.height() && width > largest_size {
            largest_size = width;
            largest_index = i;
        }
    }

    let image = icondir.entries()[largest_index].decode()?;
    let out_file = File::create(output_path).unwrap();

    image.write_png(out_file)
}

/// Makes an icon for the application
pub fn make_icon(input_bin: &str, file_name: &str) -> String {
    let home = home::home_dir().unwrap();
    let home = home.to_str().unwrap();

    let output_path = &format!("{home}/.local/share/icons/{file_name}.png");

    if let Some(icon) = get_first_ico_file(input_bin) {
        let _ = convert_largest_square_image_in_ico_to_png(icon, output_path);
    }

    output_path.to_string()
}
