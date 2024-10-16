use image::{Rgba, RgbaImage, imageops};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <image1> <image2> <output>", args[0]);
        return Ok(());
    }

    let image1_path = &args[1];
    let image2_path = &args[2];
    let output_path = &args[3];

    // Load the input images
    let img1 = image::open(image1_path)?;
    let img2 = image::open(image2_path)?;

    // Set dimensions
    let width = 800;
    let height = 600;

    // Create a new image for the result
    let mut result = RgbaImage::new(width, height);

    // Function to fit image in triangle
    fn fit_image_in_triangle(img: &RgbaImage, result: &mut RgbaImage, top_left: bool) {
        let (width, height) = result.dimensions();
        let diagonal_slope = height as f32 / width as f32;
        
        let triangle_height = if top_left { height as f32 } else { width as f32 * diagonal_slope } as u32;
        let triangle_width = if top_left { height as f32 / diagonal_slope } else { width as f32 } as u32;
        
        let scale = f32::min(
            triangle_width as f32 / img.width() as f32,
            triangle_height as f32 / img.height() as f32
        ) * 0.9; // 0.9 to leave a small margin
        
        let new_width = (img.width() as f32 * scale) as u32;
        let new_height = (img.height() as f32 * scale) as u32;
        
        let resized = imageops::resize(img, new_width, new_height, imageops::FilterType::Lanczos3);
        
        let (offset_x, offset_y) = if top_left {
            ((triangle_width - new_width) / 2, (triangle_height - new_height) / 2)
        } else {
            (width - triangle_width + (triangle_width - new_width) / 2, 
             height - triangle_height + (triangle_height - new_height) / 2)
        };
        
        for (x, y, &pixel) in resized.enumerate_pixels() {
            let dest_x = x + offset_x;
            let dest_y = y + offset_y;
            if dest_x < width && dest_y < height {
                let is_in_triangle = if top_left {
                    dest_y < height - (dest_x as f32 * diagonal_slope) as u32
                } else {
                    dest_y > height - (dest_x as f32 * diagonal_slope) as u32
                };
                if is_in_triangle {
                    result.put_pixel(dest_x, dest_y, pixel);
                }
            }
        }
    }

    // Fit images in triangles
    fit_image_in_triangle(&img1.to_rgba8(), &mut result, true);
    fit_image_in_triangle(&img2.to_rgba8(), &mut result, false);

    // Create a thin slanted slash
    let slash_width = 5;
    let slash_color = Rgba([128u8, 128u8, 128u8, 200u8]); // Semi-transparent grey

    for x in 0..width {
        let y_center = height.saturating_sub(x * height / width);
        let y_start = y_center.saturating_sub(slash_width / 2);
        let y_end = std::cmp::min(y_center + slash_width / 2, height);
        
        for y in y_start..y_end {
            result.put_pixel(x, y, slash_color);
        }
    }

    // Load a font
    let font = Vec::from(include_bytes!("../assets/Arial.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    // Draw "VS" text
    let text = "VS";
    let scale = Scale::uniform(60.0);
    
    // Calculate text position to fit in the slash
    let text_width = 60;
    let text_height = 60;
    let text_x = (width - text_width) / 2;
    let text_y = height / 2 - text_height / 2;

    // Draw text with a slight shadow for better visibility
    draw_text_mut(
        &mut result,
        Rgba([0u8, 0u8, 0u8, 200u8]), // Shadow color
        text_x as i32 + 2,
        text_y as i32 + 2,
        scale,
        &font,
        text,
    );

    draw_text_mut(
        &mut result,
        Rgba([255u8, 255u8, 255u8, 255u8]), // White color
        text_x as i32,
        text_y as i32,
        scale,
        &font,
        text,
    );

    // Save the result
    result.save(output_path)?;

    println!("Image processing complete. Check {output_path}");

    Ok(())
}
