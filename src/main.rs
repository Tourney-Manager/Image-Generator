use base64::{engine::general_purpose::STANDARD as base64_engine, Engine};
use image::{codecs::gif::GifDecoder, AnimationDecoder, Rgba, RgbaImage, ColorType, ImageEncoder, buffer::ConvertBuffer};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use rand::Rng;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};

/// Extracts a frame from a GIF, taking the first frame.
/// 
/// # Arguments
/// 
/// * `path` - The file path to the GIF image
/// 
/// # Returns
/// 
/// * `Result<RgbaImage, Box<dyn std::error::Error>>` - The extracted frame as an RgbaImage or an error
fn extract_frame_from_gif(path: &str) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let decoder = GifDecoder::new(reader)?;
    
    // Get the first frame directly
    let frame = decoder
        .into_frames()
        .next()
        .ok_or("No frames found in GIF")?
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    Ok(frame.into_buffer().convert())
}

/// Checks if a file is a GIF by reading its magic numbers.
/// 
/// # Arguments
/// 
/// * `path` - The file path to check
/// 
/// # Returns
/// 
/// * `Result<bool, std::io::Error>` - True if the file is a GIF, false otherwise
fn is_gif(path: &str) -> Result<bool, std::io::Error> {
    let mut file = File::open(path)?;
    let mut buffer = [0; 3];
    file.read_exact(&mut buffer)?;
    Ok(&buffer == b"GIF")
}

/// Fits and stretches an image into a triangle within the result image.
/// 
/// # Arguments
/// 
/// * `img` - The source image to fit into the triangle
/// * `result` - The destination image where the triangle will be drawn
/// * `is_top_left` - A boolean indicating whether to fit in the top-left or bottom-right triangle
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

/// Adds a fire effect at a specific pixel location in the image.
/// 
/// # Arguments
/// 
/// * `image` - The mutable reference to the target image where the effect will be applied
/// * `x` - The x-coordinate of the pixel where the effect starts
/// * `y` - The y-coordinate of the pixel where the effect starts
/// * `intensity` - The intensity of the fire effect
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

/// Main function that processes two input images and generates a combined image
/// with special effects, returning the result as a base64 encoded string.
/// 
/// # Arguments
/// 
/// * Command line arguments:
///   * First argument: Path to the first input image
///   * Second argument: Path to the second input image
/// 
/// # Returns
/// 
/// * `Result<(), Box<dyn std::error::Error>>` - Ok(()) if successful, Error if failed
/// 
/// # Example
/// 
/// ```bash
/// cargo run path/to/image1.png path/to/image2.gif
/// ```
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <image1_path> <image2_path>", args[0]);
        return Ok(());
    }

    let image1_path = &args[1];
    let image2_path = &args[2];

    // Load the input images, handling GIFs appropriately
    let img1 = if is_gif(image1_path)? {
        extract_frame_from_gif(image1_path)?
    } else {
        image::open(image1_path)?.to_rgba8()
    };

    let img2 = if is_gif(image2_path)? {
        extract_frame_from_gif(image2_path)?
    } else {
        image::open(image2_path)?.to_rgba8()
    };

    // Set dimensions for the output image
    let size = 1000;
    let width = size;
    let height = size;

    // Create a new image for the result
    let mut result = RgbaImage::new(width, height);

    // Fit images into triangles
    fit_image_in_triangle(&img1, &mut result, true);
    fit_image_in_triangle(&img2, &mut result, false);

    // Create a fiery diagonal line effect
    let line_width = 5;
    for x in 0..width {
        let y = height - x;
        for dy in 0..line_width {
            if y + dy < height {
                add_fire_effect(&mut result, x, y + dy, 200);
            }
        }
    }

    // Load a font for drawing text onto the image
    let font_data = include_bytes!("../assets/Arial.ttf");
    let font = Font::try_from_vec(font_data.to_vec()).unwrap();

    // Draw "VS" text with fire effect on the resulting image
    let text = "VS";
    let scale = Scale::uniform(120.0);
    
    // Calculate position for centering text on the canvas
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

    // Add fire effect to drawn text
    for x in text_x..text_x + text_width {
        for y in text_y..text_y + text_height {
            if result.get_pixel(x, y)[0] == 255 {
                add_fire_effect(&mut result, x, y, 150);
            }
        }
    }

    // Add glitter effect randomly across the resulting image
    let mut rng = rand::thread_rng();
    for _ in 0..500 {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let intensity = rng.gen_range(150..255) as u8;
        result.put_pixel(x, y, Rgba([intensity, intensity, intensity, 255]));
    }

    // Encode the resulting image to PNG format and convert to base64
    let mut buffer: Vec<u8> = Vec::new();
    {
        use std::io::Cursor;
        image::codecs::png::PngEncoder::new(Cursor::new(&mut buffer)).write_image(
            &result,
            width,
            height,
            ColorType::Rgba8,
        )?;
    }

    // Convert to base64 and print to stdout
    let base64_string = base64_engine.encode(&buffer);
    println!("{}", base64_string);

    // Open or create the file "base64.txt" and overwrite its contents
    let mut file = OpenOptions::new()
        .write(true)      // Enable write access
        .create(true)     // Create the file if it does not exist
        .truncate(true)   // Truncate the file if it already exists
        .open("base64.txt")?; // Open the specified file

    // Write the Base64 string to the file
    file.write_all(base64_string.as_bytes())?;

    Ok(())
}
