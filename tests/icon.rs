// use sakaya::convert_largest_square_image_in_ico_to_png;
// use sakaya::get_first_ico_file;
//
// #[test]
// fn gets_ico_from_osu_exe() {
//     // https://m1.ppy.sh/r/osu!install.exe
//     assert!(
//         get_first_ico_file("./in-out/osu!install.exe").is_some(),
//         "osu!install.exe returns an ico group"
//     );
// }
//
// #[test]
// fn gets_largest_osu_icon() {
//     if let Some(ico) = get_first_ico_file("./in-out/osu!install.exe") {
//         assert!(
//             convert_largest_square_image_in_ico_to_png(ico, "./in-out/osu!install.png").is_ok()
//         );
//     }
// }
//
// #[test]
// fn gets_the_first_icon_if_there_is_only_one() {
//     if let Some(ico) = get_first_ico_file("./in-out/test.exe") {
//         assert!(convert_largest_square_image_in_ico_to_png(ico, "./in-out/test.png").is_ok());
//     }
// }
