use image::{GenericImage, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::env;
// Main function remains unchanged
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
    let img1 = image::open(image1_path).expect("Failed to open image 1").to_rgba8();
    let img2 = image::open(image2_path).expect("Failed to open image 2").to_rgba8();

    // Resize images
    let target_height = 300; // Desired height for both images
    let img1_resized = resize_image(&img1, target_height);
    let img2_resized = resize_image(&img2, target_height);

    // Create a new blank image with width of both images and height of the tallest one
    let (width1, _) = img1_resized.dimensions();
    let (width2, _) = img2_resized.dimensions();
    
    let total_width = width1 + width2 + 50; // Adding space for "vs"
    
    let mut merged_image: RgbaImage = RgbaImage::new(total_width, target_height);

    // Draw first image
    merged_image.copy_from(&img1_resized, 0, 0).unwrap();
    
    // Draw second image
    merged_image.copy_from(&img2_resized, width1 + 50, 0).unwrap(); // Offset for "vs"

    // Load font and draw "vs"
    let font_data = include_bytes!("../assets/Arial.ttf"); // Ensure you have this font file in assets folder
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error loading font");

    let scale = Scale { x: 50.0, y: 50.0 }; // Font size
    let color = Rgba([255, 255, 255, 255]); // White color

    draw_text_mut(&mut merged_image, color, (width1 + 10).try_into().unwrap(), (target_height / 4).try_into().unwrap(), scale, &font, "VS");

    // Apply some effects (optional)
    apply_effects(&mut merged_image);

    // Save the merged image
    merged_image.save(output_path).expect("Failed to save output image");
}

// Function to resize an image while maintaining aspect ratio
fn resize_image(img: &RgbaImage, target_height: u32) -> RgbaImage {
    let (width, height) = img.dimensions();
    
    if height <= target_height {
        return img.clone(); // No need to resize if already within target height
    }

    let aspect_ratio = width as f32 / height as f32;
    let new_width = (target_height as f32 * aspect_ratio) as u32;

    // Use the correct method to resize the image
    image::imageops::resize(img, new_width, target_height, image::imageops::FilterType::Lanczos3)
}

// Function to apply effects to the merged image (e.g., blur)
fn apply_effects(img: &mut RgbaImage) {
    use imageproc::filter::gaussian_blur_f32;

    _ = gaussian_blur_f32(img, 5.0); // Apply Gaussian blur with a radius of 5.0
}
