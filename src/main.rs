use image::{GenericImageView, ImageBuffer, Rgba, imageops::FilterType};
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
    let mut img1 = image::open(image1_path)?;
    let mut img2 = image::open(image2_path)?;

    // Resize images to a common size (e.g., 800x600)
    let width = 800;
    let height = 600;
    img1 = img1.resize_exact(width, height, FilterType::Lanczos3);
    img2 = img2.resize_exact(width, height, FilterType::Lanczos3);

    // Create a new image for the result
    let mut result = ImageBuffer::new(width, height);

    // Merge images
    for (x, y, pixel) in result.enumerate_pixels_mut() {
        if y < x * height / width {
            *pixel = img1.get_pixel(x, y);
        } else {
            *pixel = img2.get_pixel(x, y);
        }
    }

    // Load a font
    let font = Vec::from(include_bytes!("../assets/Arial.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    // Draw "VS" text
    let scale = Scale::uniform(120.0);
    let text = "VS";
    draw_text_mut(
        &mut result,
        Rgba([255u8, 0u8, 0u8, 255u8]), // Red color
        (width / 2 - 60).try_into().unwrap(),
        (height / 2 - 60).try_into().unwrap(),
        scale,
        &font,
        text,
    );

    // Save the result
    result.save(output_path)?;

    println!("Image processing complete. Check {output_path}");

    Ok(())
}
