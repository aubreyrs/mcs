use reqwest;
use image::{GenericImageView, ImageFormat};
use std::io::Cursor;

pub fn render_head_with_info(player_name: &str, uuid: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://minotar.net/helm/{}/8.png", player_name);
    let response = reqwest::blocking::get(&url)?;
    let bytes = response.bytes()?;
    let img = image::load(Cursor::new(bytes), ImageFormat::Png)?;
    let info_lines = vec![
        String::new(),
        format!("\x1b[38;2;245;224;220m UUID: {}\x1b[0m", uuid),
        format!("\x1b[38;2;242;205;205m NameMC: https://namemc.com/{}\x1b[0m", player_name),
        format!("\x1b[38;2;245;194;231m Laby: https://laby.net/{}\x1b[0m", player_name),
        format!("\x1b[38;2;203;166;247m Crafty: https://crafty.gg/@{}\x1b[0m", player_name),
        format!("\x1b[38;2;243;139;168m Skin: https://minotar.net/skin/{}\x1b[0m", player_name),
        format!("\x1b[38;2;235;160;172m Cape: https://capes.dev/{}\x1b[0m", player_name),
        String::new(),
    ];

    let content_width = info_lines.iter().map(|line| line.len()).max().unwrap_or(0).max(img.width() as usize * 2 + 4);
    println!("\n\n");
    for (y, info) in (0..img.height()).zip(info_lines.iter().chain(std::iter::repeat(&String::new()))) {
        print!("    ");
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let image::Rgba(data) = pixel;
            print!(
                "\x1b[48;2;{};{};{}m  ",
                data[0], data[1], data[2]
            );
        }
        println!("\x1b[0m  {:width$}", info, width = content_width - img.width() as usize * 2 - 4);
    }
    for info in info_lines.iter().skip(img.height() as usize) {
        println!("    {:width$}", format!("{}", info), width = content_width);
    }

    println!("\x1b[0m");

    Ok(())
}
