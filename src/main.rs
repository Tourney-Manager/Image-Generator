use image::{GenericImage, GenericImageView, Rgba, RgbaImage};
use rusttype::{Font, Scale};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <image1> <image2> <output>", args[0]);
        return;
    }

    let image1_path = &args[1];
    let image2_path = &args[2];
    let output_path = &args[3];

    // Load images
    let img1 = image::open(image1_path).expect("Failed to open image 1");
    let img2 = image::open(image2_path).expect("Failed to open image 2");

    // Create a new blank image with width of both images and height of the tallest one
    let (width1, height1) = img1.dimensions();
    let (width2, height2) = img2.dimensions();
    let total_width = width1 + width2 + 50; // Adding space for "vs"
    let max_height = height1.max(height2);
    
    let mut merged_image: RgbaImage = RgbaImage::new(total_width, max_height);

    // Draw first image
    merged_image.copy_from(&img1.to_rgba8(), 0, 0).unwrap();
    // Draw second image
    merged_image.copy_from(&img2.to_rgba8(), width1 + 50, 0).unwrap(); // Offset for "vs"

    // Load font and draw "vs"
    let font_data = include_bytes!("../assets/Arial.ttf"); // Ensure you have this font file in assets folder
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error loading font");

    let scale = Scale { x: 50.0, y: 50.0 }; // Font size
    let color = Rgba([255, 255, 255, 255]); // White color

    let text = "vs";
    
    for glyph in font.layout(text, scale, rusttype::point(width1 as f32 + 10.0, max_height as f32 / 2.0)) {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                if v > 0.0 {
                    let px = x + bounding_box.min.x as u32;
                    let py = y + bounding_box.min.y as u32;
                    if px < total_width && py < max_height {
                        merged_image.put_pixel(px, py, Rgba([
                            (color[0] as f32 * v) as u8,
                            (color[1] as f32 * v) as u8,
                            (color[2] as f32 * v) as u8,
                            color[3],
                        ]));
                    }
                }
            });
        }
    }

    // Save the merged image
    merged_image.save(output_path).expect("Failed to save output image");
}
