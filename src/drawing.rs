// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

/*
this is old code to generate png from scratch:

use imageproc::drawing::draw_text_mut;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use rusttype::{Font, Scale};
use image::Rgba;

const IMAGE: &[u8] = include_bytes!("../assets/left.png");
const FONT: &[u8] = include_bytes!("../assets/ALatsi-Regular.ttf");
const BLACK: Rgba<u8> = Rgba([0u8, 0u8, 0u8, 255u8]);
const WHITE: Rgba<u8> = Rgba([255u8, 255u8, 255u8, 255u8]);

pub fn draw_file(count: usize) {
    let to_text = &count.to_string();
    let letter_size = 12 + (8 * (to_text.len() + 1)) as u32;
    let mut image = image::ImageBuffer::new(100 + letter_size, 20);
    let background = image::load_from_memory(IMAGE).unwrap().to_rgba8();

    image::imageops::overlay(&mut image, &background, 0, 0);

    draw_filled_rect_mut(&mut image, Rect::at(100, 0).of_size(letter_size, 20), WHITE);
    draw_text_mut(&mut image, BLACK, 102, 0, Scale{
        x: 18f32,
        y: 20f32,
    }, &Font::try_from_bytes(FONT).unwrap(), to_text);

    image.save(format!("{}.png", count));
}
*/

pub fn draw_file(text: &str) -> String {
    let width = 12 + (text.len() * 8);

    format!(
        r#"<!--
    Copyright (c) 2022 aiocat
    
    This software is released under the MIT License.
    https://opensource.org/licenses/MIT
    -->
    
    <svg height="20" xmlns="http://www.w3.org/2000/svg">
       <rect width="80" height="20" style="fill:#2a2a2a;" rx="5" ry="5" />
       <text x="6" font-size="12px" font-family="monospace,sans-serif" y="15" fill="black">View Count</text>
       <text x="5" font-size="12px" font-family="monospace,sans-serif" y="14" fill="white">View Count</text>
       
        <g>
            <rect x="76" width="{0}" height="20" style="fill:#52d36a;" />
            <svg x="76" width="{0}" height="20">
                <text x="50%" y="11" font-size="12px" alignment-baseline="middle" text-anchor="middle" font-family="monospace,sans-serif" font-weight="bold" fill="black">{1}</text>      
            </svg>
        </g>
    </svg>
    "#,
        width, text
    )
}
