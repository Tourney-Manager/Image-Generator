# Image-Generator
Source Code for the program which generates "VS" images for match embeds.

## Overview

This Rust application is designed to generate visually appealing "VS" images specifically for use as embed thumbnails in the **Tournament Manager** Discord bot. The application combines two input images and applies various effects to create a dynamic output image that enhances the visual appeal of tournament announcements.

## Features

- **Image Combination**: Combines two images into a single output image.
- **Visual Effects**: Applies fire effects and glitter to enhance the "VS" text.
- **Dynamic Text Rendering**: Draws "VS" text in the center of the image using a specified font.
- **PNG Output**: Outputs the final image as a PNG, which can be easily shared or used in Discord embeds.

## How It Works

1. **Input Images**: The program takes two image file paths as command-line arguments.
2. **Image Processing**:
   - Loads the input images and converts them to RGBA format.
   - Fits each image into one of two triangles within a larger canvas.
   - Applies fire effects along a diagonal line and to the "VS" text.
   - Adds random glitter effects across the resulting image.
3. **Output Generation**: The final processed image is encoded in PNG format and written directly to standard output, allowing other applications (like Discord bots) to capture it easily.

## Usage

### Running the Application

To use this Rust application, follow these steps:

1. **Install Rust**: Ensure that you have Rust installed on your machine. You can download it from [rust-lang.org](https://www.rust-lang.org/).

2. **Build the Application**:
   ```bash
   cargo build --release
   ```

3. **Run the Application**:
   Execute the application from the command line with two image paths as arguments:
   ```bash
   ./target/release/vs_image path/to/image1.png path/to/image2.png
   ```

### Integration with Other Languages

You can call this Rust executable from any programming language that supports executing system commands (like Python, JavaScript, etc.). For example, you can use the following command in your code:

```bash
./target/release/vs_image /path/to/image1.png /path/to/image2.png
```

### Output

The output will be a PNG image written directly to standard output, which can be redirected or captured by other applications.

## Purpose

This application was specifically developed for the premium feature of the **Tournament Manager** Discord bot, enhancing tournament announcements with visually striking "VS" images. This feature aims to improve user engagement and provide a more professional appearance for tournament promotions.

---

If you have any questions or need further assistance, feel free to reach out!
