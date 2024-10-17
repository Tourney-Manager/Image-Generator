use image::{Rgba, RgbaImage, ColorType, ImageEncoder};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use rand::Rng;
use std::env;
use std::io::Cursor;
use base64::{engine::general_purpose, Engine};

/// Main function that processes two input images and generates a combined image
/// with special effects, returning the result as a Base64-encoded PNG.
///
/// # Arguments
///
/// * `<image1>` - The path to the first input image.
/// * `<image2>` - The path to the second input image.
///
/// # Example
///
/// To run the program from the command line:
///
/// ```bash
/// cargo run path/to/image1.png path/to/image2.png
/// ```
///
/// The output will be a Base64 string representing the processed image.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <image1> <image2>", args[0]);
        return Ok(());
    }

    let image1_path = &args[1];
    let image2_path = &args[2];

    // Load the input images and convert them to RGBA format.
    let img1 = image::open(image1_path)?.to_rgba8();
    let img2 = image::open(image2_path)?.to_rgba8();

    // Set dimensions for the output image.
    let size = 1000;
    let width = size;
    let height = size;

    // Create a new image for the result.
    let mut result = RgbaImage::new(width, height);

    /// Fits and stretches an image into a triangle within the result image.
    fn fit_image_in_triangle(img: &RgbaImage, result: &mut RgbaImage, is_top_left: bool) {
        let (width, height) = result.dimensions();
        
        for (dest_x, dest_y, pixel) in result.enumerate_pixels_mut() {
            let is_in_triangle = if is_top_left {
                dest_y < height - dest_x
            } else {
                dest_y > height - dest_x
            };
            
            if is_in_triangle {
                let (source_x, source_y) = if is_top_left {
                    (dest_x * img.width() / width, dest_y * img.height() / height)
                } else {
                    ((width - dest_x) * img.width() / width, (height - dest_y) * img.height() / height)
                };
                
                *pixel = *img.get_pixel(source_x, source_y);
            }
        }
    }

    // Fit images into triangles.
    fit_image_in_triangle(&img1, &mut result, true);
    fit_image_in_triangle(&img2, &mut result, false);

    /// Adds a fire effect at a specific pixel location in the image.
    fn add_fire_effect(image: &mut RgbaImage, x: u32, y: u32, intensity: u8) {
        for dx in -5..=5 {
            for dy in -5..=5 {
                let fx = x as i32 + dx;
                let fy = y as i32 + dy;
                if fx >= 0 && fx < image.width() as i32 && fy >= 0 && fy < image.height() as i32 {
                    let distance = (dx * dx + dy * dy) as f32;
                    let factor = (-distance / 10.0).exp();
                    let fire_intensity = (intensity as f32 * factor) as u8;
                    let pixel = image.get_pixel_mut(fx as u32, fy as u32);
                    pixel[0] = pixel[0].saturating_add(fire_intensity);
                    pixel[1] = pixel[1].saturating_add(fire_intensity / 2);
                }
            }
        }
    }

    // Create a fiery diagonal line effect.
    let line_width = 5;
    for x in 0..width {
        let y = height - x;
        for dy in 0..line_width {
            if y + dy < height {
                add_fire_effect(&mut result, x, y + dy, 200);
            }
        }
    }

    // Load a font for drawing text.
    let font_data = include_bytes!("../assets/Arial.ttf");
    let font = Font::try_from_vec(font_data.to_vec()).unwrap();

    // Draw "VS" text with fire effect.
    let text = "VS";
    let scale = Scale::uniform(120.0);
    
    let text_width = 120;
    let text_height = 120;
    let text_x = (width - text_width) / 2;
    let text_y = (height - text_height) / 2;

    draw_text_mut(
        &mut result,
        Rgba([255u8, 255u8, 255u8, 255u8]), // White color
        text_x as i32,
        text_y as i32,
        scale,
        &font,
        text,
    );

    // Add fire effect to the drawn text.
    for x in text_x..text_x + text_width {
        for y in text_y..text_y + text_height {
            if result.get_pixel(x, y)[0] == 255 {
                add_fire_effect(&mut result, x, y, 150);
            }
        }
    }

    // Add glitter effect randomly across the image.
    let mut rng = rand::thread_rng();
    for _ in 0..500 {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let intensity = rng.gen_range(150..255) as u8;
        result.put_pixel(x, y, Rgba([intensity, intensity, intensity, 255]));
    }

    // Encode the resulting image to PNG format and then to Base64.
    let mut buffer: Vec<u8> = Vec::new();
    
    {
        let writer = Cursor::new(&mut buffer);
        image::codecs::png::PngEncoder::new(writer).write_image(
            &result,
            width,
            height,
            ColorType::Rgba8,
        )?;
        
        // Encode the buffer to Base64 using general-purpose engine.
        let res_base64 = general_purpose::STANDARD.encode(&buffer);
        
        println!("data:image/png;base64,{res_base64}");
        
        Ok(())
    }
}
