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
    let size = 800;
    let width = size;
    let height = size;

    // Create a new image for the result
    let mut result = RgbaImage::new(width, height);

    // Function to fit image in triangle
    fn fit_image_in_triangle(img: &RgbaImage, result: &mut RgbaImage, is_top_left: bool) {
        let (width, height) = result.dimensions();
        
        // Calculate the scale to fit the image in half of the square
        let scale = f32::min(
            width as f32 / (2.0 * img.width() as f32),
            height as f32 / img.height() as f32
        );
        
        let new_width = (img.width() as f32 * scale) as u32;
        let new_height = (img.height() as f32 * scale) as u32;
        
        let resized = imageops::resize(img, new_width, new_height, imageops::FilterType::Lanczos3);
        
        for (x, y, &pixel) in resized.enumerate_pixels() {
            let (dest_x, dest_y) = if is_top_left {
                (x, y)
            } else {
                (width - new_width + x, height - new_height + y)
            };
            
            if dest_x < width && dest_y < height {
                let is_in_triangle = if is_top_left {
                    dest_y < height - dest_x
                } else {
                    dest_y > height - dest_x
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

    // Create a thin diagonal line
    let line_width = 3;
    let line_color = Rgba([255u8, 255u8, 255u8, 255u8]); // White color

    for x in 0..width {
        let y = height - x;
        for dy in 0..line_width {
            if y + dy < height {
                result.put_pixel(x, y + dy, line_color);
            }
        }
    }

    // Load a font
    let font = Vec::from(include_bytes!("../assets/Arial.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    // Draw "VS" text
    let text = "VS";
    let scale = Scale::uniform(80.0);
    
    // Calculate text position to fit in the center
    let text_width = 80;
    let text_height = 80;
    let text_x = (width - text_width) / 2;
    let text_y = (height - text_height) / 2;

    // Draw text with a slight shadow for better visibility
    draw_text_mut(
        &mut result,
        Rgba([0u8, 0u8, 0u8, 255u8]), // Shadow color
        text_x as i32 + 2,
        text_y as i32 + 2,
        scale,
        &font,
        text,
    );

    draw_text_mut(
        &mut result,
        Rgba([255u8, 0u8, 0u8, 255u8]), // Red color
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
