use pelite::{image::{IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_I386}, FileMap, PeFile};

/// Gets whether the exe is 32 or 64-bit
///
/// https://github.com/MicrosoftDocs/win32/blob/docs/desktop-src/Debug/pe-format.md#machine-types
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
