use image::{ImageBuffer, Rgba, RgbaImage, Pixel};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

pub fn create_image(
    size: &str,
    bg_color: Option<&str>,
    fg_color: Option<&str>,
    text: Option<&str>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // 解析尺寸，支持单个数字和 widthxheight 格式
    let (width, height) = parse_size(size)?;
    
    // 使用默认颜色或解析提供的颜色
    let bg = match bg_color {
        Some(color) => parse_color(color)?,
        None => Rgba([240, 240, 240, 255])  // 默认淡灰色
    };
    
    let fg = match fg_color {
        Some(color) => parse_color(color)?,
        None => Rgba([0, 0, 0, 255])  // 默认黑色
    };

    // 创建图片
    let mut img: RgbaImage = ImageBuffer::new(width, height);

    // 填充背景色
    for pixel in img.pixels_mut() {
        *pixel = bg;
    }

    // 如果有文本，则绘制文本
    if let Some(text) = text {
        draw_centered_text(&mut img, text, &fg)?;
    } else {
        // 如果没有文本，绘制尺寸信息
        let size_text = format!("{}x{}", width, height);
        draw_centered_text(&mut img, &size_text, &fg)?;
    }

    // 图片写入
    let mut buffer = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut buffer);
    img.write_to(&mut cursor, image::ImageFormat::Png)?;
    
    Ok(buffer)
}

fn draw_centered_text(img: &mut RgbaImage, text: &str, color: &Rgba<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = img.dimensions();
    let font_data = include_bytes!("/Library/Fonts/Arial Unicode.ttf") as &[u8];
    let font = Font::try_from_vec(Vec::from(font_data))
        .unwrap_or_else(|| panic!("Error loading font"));

    // 调整文本大小，使其适应图片
    let scale = Scale {
        x: height as f32 / 6.0,
        y: height as f32 / 6.0,
    };

    // 获取字体度量信息
    let v_metrics = font.v_metrics(scale);
    
    // 计算基线位置
    let baseline = v_metrics.ascent;

    // 获取文本布局信息
    let glyphs: Vec<_> = font
        .layout(text, scale, rusttype::point(0.0, baseline))
        .collect();
    
    // 计算文本边界
    let bounds = glyphs
        .iter()
        .filter_map(|g| g.pixel_bounding_box())
        .fold(None, |acc: Option<(i32, i32, i32, i32)>, bbox| {
            Some(match acc {
                None => (bbox.min.x, bbox.min.y, bbox.max.x, bbox.max.y),
                Some((x1, y1, x2, y2)) => (
                    x1.min(bbox.min.x),
                    y1.min(bbox.min.y),
                    x2.max(bbox.max.x),
                    y2.max(bbox.max.y),
                ),
            })
        })
        .unwrap_or((0, 0, 0, 0));

    // 计算文本尺寸
    let text_width = (bounds.2 - bounds.0) as u32;
    let text_height = (bounds.3 - bounds.1) as u32;

    // 计算水平居中位置
    let x = if text_width >= width {
        0i32
    } else {
        ((width - text_width) / 2) as i32
    };

    // 计算垂直居中位置
    let y = if text_height >= height {
        0i32
    } else {
        ((height - text_height) / 2) as i32
    };

    // 绘制文本，考虑基线和边界偏移
    draw_text_mut(
        img,
        *color,
        x - bounds.0,
        y - bounds.1,
        scale,
        &font,
        text,
    );

    Ok(())
}

fn parse_size(size: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    if let Ok(single_size) = size.parse::<u32>() {
        // 如果是单个数字，宽高相等
        return Ok((single_size, single_size));
    }
    
    // 否则尝试解析 widthxheight 格式
    let parts: Vec<&str> = size.split('x').collect();
    if parts.len() != 2 {
        return Err("Invalid size format".into());
    }
    
    Ok((
        parts[0].parse::<u32>()?,
        parts[1].parse::<u32>()?,
    ))
}

fn parse_color(color: &str) -> Result<Rgba<u8>, Box<dyn std::error::Error>> {
    if color.len() != 6 {
        return Err("Invalid color format".into());
    }

    let r = u8::from_str_radix(&color[0..2], 16)?;
    let g = u8::from_str_radix(&color[2..4], 16)?;
    let b = u8::from_str_radix(&color[4..6], 16)?;

    Ok(Rgba([r, g, b, 255]))
} 