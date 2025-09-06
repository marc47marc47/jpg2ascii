use jpg2ascii::{convert_path_to_ascii, Config};

#[test]
fn snapshot_8x3_multi_colors_width8_aspect1() {
    let cfg = Config {
        width: Some(8),
        aspect: 1.0,
        ..Default::default()
    };
    let out = convert_path_to_ascii(
        "image2ascii/convert/testdata/8x3_multi_colors.png",
        &cfg,
    )
    .unwrap();

    insta::assert_snapshot!(out, @r###"+@%#%=**
:%###=+-
-+*=:+.:"###);
}

