use image::{GenericImageView, Rgba, RgbaImage, imageops::FilterType};
use imageproc::drawing::{draw_text_mut, draw_filled_rect_mut};
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
    let mut img1 = image::open(image1_path)?;
    let mut img2 = image::open(image2_path)?;

    // Resize images to a common size
    let width = 800;
    let height = 600;
    img1 = img1.resize_exact(width, height, FilterType::Lanczos3);
    img2 = img2.resize_exact(width, height, FilterType::Lanczos3);

    // Create a new image for the result
    let mut result = RgbaImage::new(width, height);

    // Merge images with a diagonal split
    for (x, y, pixel) in result.enumerate_pixels_mut() {
        if y < x * height / width {
            *pixel = img1.get_pixel(x, y);
        } else {
            *pixel = img2.get_pixel(x, y);
        }
    }

    // Create a grey slash in the middle
    let slash_width = 100;
    let slash_height = height;
    let slash_x = (width - slash_width) / 2;
    draw_filled_rect_mut(
        &mut result,
        imageproc::rect::Rect::at(slash_x as i32, 0).of_size(slash_width, slash_height),
        Rgba([128u8, 128u8, 128u8, 200u8]), // Semi-transparent grey
    );

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

    // Draw base text
    draw_text_mut(
        &mut result,
        Rgba([255u8, 165u8, 0u8, 255u8]), // Orange color
        text_x as i32,
        text_y as i32,
        scale,
        &font,
        text,
    );

    // Add fire effect
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let fx = text_x + rng.gen_range(0..text_width);
        let fy = text_y + rng.gen_range(0..text_height);
        let intensity = rng.gen_range(100..255) as u8;
        result.put_pixel(fx as u32, fy as u32, Rgba([255, intensity, 0, 255]));
    }

    // Add glitter effect
    for _ in 0..200 {
        let gx = rng.gen_range(0..width);
        let gy = rng.gen_range(0..height);
        result.put_pixel(gx, gy, Rgba([255u8, 255u8, 255u8, 200u8]));
    }

    // Save the result
    result.save(output_path)?;

    println!("Image processing complete. Check {output_path}");

    Ok(())
}
