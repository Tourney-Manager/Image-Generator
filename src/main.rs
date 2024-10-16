use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use rand::Rng;
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
    let size = 1000;
    let width = size;
    let height = size;

    // Create a new image for the result
    let mut result = RgbaImage::new(width, height);

    // Function to fit and stretch image in triangle
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

    // Fit images in triangles
    fit_image_in_triangle(&img1.to_rgba8(), &mut result, true);
    fit_image_in_triangle(&img2.to_rgba8(), &mut result, false);

    // Function to add fire effect
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

    // Create a fiery diagonal line
    let line_width = 5;
    for x in 0..width {
        let y = height - x;
        for dy in 0..line_width {
            if y + dy < height {
                add_fire_effect(&mut result, x, y + dy, 200);
            }
        }
    }

    // Load a font
    let font = Vec::from(include_bytes!("../assets/Arial.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    // Draw "VS" text with fire effect
    let text = "VS";
    let scale = Scale::uniform(120.0);
    
    let text_width = 120;
    let text_height = 120;
    let text_x = (width - text_width) / 2;
    let text_y = (height - text_height) / 2;

    // Draw text
    draw_text_mut(
        &mut result,
        Rgba([255u8, 255u8, 255u8, 255u8]), // White color
        text_x as i32,
        text_y as i32,
        scale,
        &font,
        text,
    );

    // Add fire effect to text
    for x in text_x..text_x+text_width {
        for y in text_y..text_y+text_height {
            if result.get_pixel(x, y)[0] == 255 {
                add_fire_effect(&mut result, x, y, 150);
            }
        }
    }

    // Add glitter effect
    let mut rng = rand::thread_rng();
    for _ in 0..500 {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let intensity = rng.gen_range(150..255) as u8;
        result.put_pixel(x, y, Rgba([intensity, intensity, intensity, 255]));
    }

    // Save the result
    result.save(output_path)?;

    println!("Image processing complete. Check {output_path}");

    Ok(())
}
