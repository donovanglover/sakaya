use sakaya::state::Options;

#[test]
#[ignore]
fn options_default() {
    let options: Options = Default::default();

    assert_eq!(options.wine_prefix, "/mnt/wine32");
}
