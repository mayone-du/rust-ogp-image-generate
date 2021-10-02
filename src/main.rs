use chrono::Local;
use image::Rgba;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::env::args;

fn main() {
    // 背景用画像を取得
    let mut image = match image::open("assets/images/220x220.jpg") {
        Ok(image) => image,
        Err(err) => panic!("画像を読み取れませんでした。{:?}", err),
    };
    // 描画するためのフォントを取得
    let font = Vec::from(include_bytes!("../assets/fonts/azuki.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    // 文字のサイズを決定
    let size = 20.0;
    let scale = Scale { x: size, y: size };

    // コマンド実行時の引数で文字列を受け取り、その文字を画像に描画する
    let args: Vec<String> = args().collect();
    let text = &args[1];

    // テキストを画像に描画
    draw_text_mut(
        &mut image,
        Rgba([0u8, 0u8, 0u8, 0u8]),
        20,
        0,
        scale,
        &font,
        text,
    );

    // 日付をファイル名にして画像を保存
    let now = Local::now().format("%Y-%m-%d.jpg");
    let save_path = format!("generated/{}", now);

    // 画像を保存
    image.save(save_path).unwrap();
}
