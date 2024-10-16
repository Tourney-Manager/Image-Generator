use image::{GenericImageView, Rgba, RgbaImage};
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

    // Function to map coordinates
    let map_coordinates = |x: u32, y: u32, source_width: u32, source_height: u32| -> (u32, u32) {
        let target_x = x * source_width / width;
        let target_y = y * source_height / height;
        (target_x, target_y)
    };

    // Merge images with proper scaling
    for (x, y, pixel) in result.enumerate_pixels_mut() {
        if y < height - x * height / width {
            let (sx, sy) = map_coordinates(x, y, img1.width(), img1.height());
            *pixel = img1.get_pixel(sx, sy);
        } else {
            let (sx, sy) = map_coordinates(x, y, img2.width(), img2.height());
            *pixel = img2.get_pixel(sx, sy);
        }
    }

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
