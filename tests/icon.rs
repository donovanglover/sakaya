use sakaya::get_ico_files;

#[test]
fn gets_icon_files_from_osu_exe() {
    // https://m1.ppy.sh/r/osu!install.exe
    get_ico_files("osu!install.exe");
}
