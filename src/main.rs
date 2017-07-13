extern crate image;

use std::fs::File;
use std::path::Path;
use std::string::String;

use image::Pixel;

fn main() {

    println!("Hello World");

    let imgx : u32 = 800;
    let imgy : u32 = 800;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
        // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        // Create an 8bit pixel of type Luma and value i
        // and assign in to the pixel at position (x, y)
        *pixel = generate_value(x, y);

    }

    write_image(&imgbuf);
}

fn generate_value(x : u32, y:u32) -> image::Rgb<u8> {

    image::Rgb::from_channels(x as u8, y as u8, 127, 255)
}

fn get_filename() -> String {

    let base_name  = "images/output";
    let ending = ".png";

    let mut i = 0;

    let mut complete_name : String;

    loop{
        complete_name = format!("{}{}{}", base_name, i, ending);
        if !Path::new(&complete_name).exists() {
            break;
        }

        i += 1;

    }

    complete_name
    
}

fn write_image(imgbuf : &image::RgbImage) {


    // Create a new ImgBuf with width: imgx and height: imgy

    // We must indicate the image’s color type and what format to save as
    let _ = imgbuf.save(&Path::new(&get_filename()));
}
