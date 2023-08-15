use sakaya::get_first_ico_file;

#[test]
fn gets_icon_files_from_osu_exe() {
    // https://m1.ppy.sh/r/osu!install.exe
    get_first_ico_file("osu!install.exe");
}
