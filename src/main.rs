extern crate image;


use std::path::Path;
use std::vec::Vec;
use std::string::String;
use std::option::Option;

use std::ops::Add;
use std::ops::Sub;
use std::cmp::min;

use image::Pixel;

#[derive(Debug, Copy, Clone)]
struct Vec3{
    x : f32,
    y : f32,
    z : f32,
}

impl Vec3{

    fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn dot(&self, other : &Vec3) -> f32 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    fn mult(self, other : f32) -> Vec3 {
        Vec3{x: self.x * other, y: self.y * other, z: self.z * other}
    }

    fn normed(&self) -> Vec3{
        let l = self.length();
        Vec3{ x: self.x/l, y: self.y/l, z:self.z/l}
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other : Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

#[derive(Debug)]
struct Ray{
    origin : Vec3,
    direction : Vec3,
}

trait CollisionObject{
    fn get_first_collision(&self, r : &Ray) -> Option<Hit>;
}

#[derive(Debug)]
struct Sphere{
    origin : Vec3,
    radius : f32,
}

struct Hit{
    position : Vec3,
    normal : Vec3,
}

impl CollisionObject for Sphere{
    fn get_first_collision(&self, ray : &Ray) -> Option<Hit> {
        // println!("Intersecting ray: {:?} with sphere {:?}", &ray, self);

        let difference = ray.origin - self.origin;
        let dir = ray.direction.normed();

        let radicand = (dir.dot(&difference)).powi(2) - difference.dot(&difference) + self.radius.powi(2);

        // println!("difference: {:?}", difference);
        // println!("dot: {:?}", dir.dot(&difference));
        // println!("dot sq: {:?}", dir.dot(&difference).powi(2));
        // println!("length: {:?}", difference.length());
        // println!("length sq: {:?}", difference.length().powi(2));
        // println!("radius: {:?}", self.radius);
        // println!("radicand : {:?}", radicand);

        if radicand < 0.0 {
            return None
        }

        let exp1 = -1.0 * ray.direction.dot(&difference);

        let dist = (exp1+radicand.sqrt()).min(exp1-radicand.sqrt());

        // println!("distance: {:?}", dist);

        let hitpoint = ray.origin + ray.direction.mult(dist);
        let normal = (hitpoint - self.origin).normed();

        Some(Hit{position: hitpoint, normal: normal})
    }
}

fn main() {

    println!("Hello World");

    let imgx : u32 = 800;
    let imgy : u32 = 800;

    let mut spheres : Vec<Sphere> = Vec::new();

    spheres.push(Sphere{ origin: Vec3 {x: 0.0, y: 0.0, z: 2.0}, radius: 0.1});

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
        // Iterate over the coordinates and pixels of the image

    //let val = generate_value(0.5, 0.5, 50.0, &spheres);
    // let val2 = generate_value(0.0, 0.0, 50.0, &spheres);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        *pixel = generate_value(x as f32 / imgx as f32, y as f32 / imgy as f32, 50.0, &spheres);

    };

    write_image(&imgbuf);
}

fn generate_value(x_pos : f32, y_pos: f32, field_of_view : f32, spheres : &Vec<Sphere>) -> image::Rgb<u8> {


    let radians_FoV = field_of_view.to_radians();
    let x_angle = ((x_pos - 0.5) * radians_FoV).sin();
    let y_angle = ((y_pos - 0.5) * radians_FoV).sin();
    let direction = Vec3{x: x_angle, y: y_angle, z: 1.0}; //Ortho projection for now
    let position = Vec3{x: 0.0, y: 0.0, z: 0.0};
    let ray = Ray{ origin: position, direction: direction};

    let mut color = image::Rgb::from_channels(0, 0, 0, 255);

    for sphere in spheres {
        let cur_hit = sphere.get_first_collision(&ray);
        

        match cur_hit {
            Some(hit) => {

                let shade = 1.0 - hit.normal.dot(&Vec3{x: 0.0, y: 0.0, z: 1.0});
                let c = (shade * 255.0) as u8;
                // println!("d: {:?}, c: {:?}", d, c);
                color = image::Rgb::from_channels(c, c, c, 255) },
            None => color = image::Rgb::from_channels(0, 0, 0, 255),
        };
    };

   color 
}

fn get_filename() -> String {

    let base_name  = "images/output";
    let ending = ".png";

    let mut i = 0;

    let mut complete_name : String;

    loop{
        complete_name = format!("{}{:04}{}", base_name, i, ending);
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
