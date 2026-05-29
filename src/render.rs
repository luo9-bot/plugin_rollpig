//! 渲染模块 - 对应原始template_to_pic

use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, draw_filled_rect_mut};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};
use crate::models::Pigsonality;

/// 生成小猪卡片图片
pub fn generate_pig_card(pig: &Pigsonality) -> Vec<u8> {
    // 创建图片 (400x300)
    let mut img: RgbImage = ImageBuffer::new(400, 300);
    
    // 背景色 (浅粉色)
    let bg_color = Rgb([255, 228, 225]);
    for pixel in img.pixels_mut() {
        *pixel = bg_color;
    }
    
    // 绘制边框
    let border_color = Rgb([255, 182, 193]);
    draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(400, 5), border_color);
    draw_filled_rect_mut(&mut img, Rect::at(0, 295).of_size(400, 5), border_color);
    draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(5, 300), border_color);
    draw_filled_rect_mut(&mut img, Rect::at(395, 0).of_size(5, 300), border_color);
    
    // 加载字体
    let font_data = include_bytes!("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf");
    let font = Font::try_from_bytes(font_data).unwrap();
    
    let scale = Scale::uniform(24.0);
    let small_scale = Scale::uniform(18.0);
    
    // 绘制标题
    draw_text_mut(&mut img, Rgb([255, 105, 180]), 20, 20, scale, &font, "🐷 今天你是...");
    
    // 绘制小猪名称
    draw_text_mut(&mut img, Rgb([0, 0, 0]), 20, 60, scale, &font, &pig.name);
    
    // 绘制描述
    draw_text_mut(&mut img, Rgb([100, 100, 100]), 20, 100, small_scale, &font, &pig.description);
    
    // 绘制分析
    let analysis_lines = wrap_text(&pig.analysis, 30);
    for (i, line) in analysis_lines.iter().enumerate() {
        draw_text_mut(&mut img, Rgb([0, 0, 0]), 20, (150 + i * 30) as i32, small_scale, &font, line);
    }
    
    // 转换为PNG字节
    let mut buf = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut buf);
    encoder.encode(&img, 400, 300, image::ColorType::Rgb8).unwrap();
    buf
}

/// 文本换行
fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_line = String::new();
    
    for c in text.chars() {
        current_line.push(c);
        if current_line.chars().count() >= max_chars {
            result.push(current_line.clone());
            current_line.clear();
        }
    }
    if !current_line.is_empty() {
        result.push(current_line);
    }
    result
}
