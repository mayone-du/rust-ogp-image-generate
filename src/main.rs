use image::Rgba;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

fn main() {
    let mut image = match image::open("assets/images/220x220.jpg") {
        Ok(image) => image,
        Err(err) => panic!("画像を読み取れませんでした。{:?}", err),
    };
    let font = Vec::from(include_bytes!("../assets/fonts/azuki.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let size = 10.0;
    let scale = Scale { x: size, y: size };

    let text = "コマンドライン引数の文字列が入ります";
    draw_text_mut(
        &mut image,
        Rgba([0u8, 0u8, 0u8, 0u8]),
        10,
        0,
        scale,
        &font,
        text,
    );

    image.save("test.jpg").unwrap();
}
