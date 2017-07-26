extern crate image;
extern crate time;


use std::path::Path;
use std::vec::Vec;
use std::string::String;
use std::option::Option;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::AddAssign;

use std::f32;

use image::Pixel;

#[derive(Debug, Copy, Clone)]
struct Vec3{
    x : f32,
    y : f32,
    z : f32,
}

fn clamp(x : f32, min : f32, max : f32) -> f32{
    x.max(min).min(max)
}

impl Vec3{

    fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn dot(&self, other : &Vec3) -> f32 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    fn mult(&self, other : f32) -> Vec3 {
        Vec3{x: self.x * other, y: self.y * other, z: self.z * other}
    }

    fn normed(&self) -> Vec3{
        let l = self.length();
        Vec3{ x: self.x/l, y: self.y/l, z:self.z/l}
    }

    fn clamp(&self, min : &Vec3, max : &Vec3) -> Vec3{
        Vec3{
            x: clamp(self.x, min.x, max.x),
            y: clamp(self.y, min.y, max.y),
            z: clamp(self.z, min.z, max.z),
        }
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

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other : Vec3) -> Vec3 {
        Vec3{x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other : Vec3) -> Vec3 {
        Vec3{x: self.x / other.x, y: self.y / other.y, z: self.z / other.z}
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
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
    incident_dir : Vec3,
}

struct Light{
    color : Vec3,
    position : Vec3,
}

struct Scene{
    objects : Vec<Sphere>,
    lights : Vec<Light>,
}

impl CollisionObject for Sphere{
    fn get_first_collision(&self, ray : &Ray) -> Option<Hit> {

        let difference = ray.origin - self.origin;
        let dir = ray.direction.normed();

        let radicand = (dir.dot(&difference)).powi(2) - difference.dot(&difference) + self.radius.powi(2);

        if radicand < 0.0 {
            return None
        }

        let exp1 = -1.0 * ray.direction.dot(&difference);

        let dist = (exp1+radicand.sqrt()).min(exp1-radicand.sqrt());

        if dist < 0.0 {
            // The hit is before the start of the ray
            return None
        }

        let hitpoint = ray.origin + ray.direction.mult(dist);
        let normal = (hitpoint - self.origin).normed();

        Some(Hit{position: hitpoint, normal: normal, incident_dir: ray.direction})
    }
}

fn main() {

    let imgx : u32 = 800;
    let imgy : u32 = 800;

    let mut spheres : Vec<Sphere> = Vec::new();
    spheres.push(Sphere{ origin: Vec3 {x: 0.0, y: 0.0, z: 2.0}, radius: 0.2});
    spheres.push(Sphere{ origin: Vec3 {x: 0.5, y: 0.0, z: 2.0}, radius: 0.2});

    let mut lights : Vec<Light> = Vec::new();
    lights.push(Light{ color: Vec3 {x: 1.0, y: 0.0, z: 0.0}, position : Vec3 { x: 0.0, y: 0.0, z: 0.0}});
    //lights.push(Light{ color: Vec3 {x: 0.0, y: 1.0, z: 0.0}, position : Vec3 { x: 1.0, y: 0.0, z: 1.8}});
    lights.push(Light{ color: Vec3 {x: 0.0, y: 0.0, z: 1.0}, position : Vec3 { x: 0.0, y: 1.0, z: 2.0}});
    lights.push(Light{ color: Vec3 {x: 0.0, y: 1.0, z: 0.0}, position : Vec3 { x: 0.5, y: -1.0, z: 2.0}});

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let scene = Scene{objects: spheres, lights: lights};

    // Iterate over the coordinates and pixels of the image

    // let val = generate_value(0.5, 0.5, 50.0, &spheres);
    // let val2 = generate_value(0.0, 0.0, 50.0, &spheres);

    let start = time::PreciseTime::now();

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        let color = generate_value(x as f32 / imgx as f32, y as f32 / imgy as f32, 50.0, &scene);


        let final_col = color.mult(255.0);

        *pixel = image::Rgb::from_channels(final_col.x as u8, final_col.y as u8, final_col.z as u8, 255);
    };

    let end = time::PreciseTime::now();

    let duration = start.to(end);

    println!("rendering took: {:?} milliseconds", duration.num_milliseconds());

    write_image(&imgbuf);
}

fn generate_value(x_pos : f32, y_pos: f32, field_of_view : f32, scene : &Scene) -> Vec3 {


    let radians_fov = field_of_view.to_radians();
    let x_angle = ((x_pos - 0.5) * radians_fov).sin();
    let y_angle = ((y_pos - 0.5) * radians_fov).sin();

    let direction = Vec3{x: x_angle, y: y_angle, z: 1.0}; //Ortho projection for now
    let origin = Vec3{x: 0.0, y: 0.0, z: 0.0};
    let ray = Ray{ origin: origin, direction: direction};

    // TODO: Secondary hits

    let mut color = Vec3{x: 0.0, y: 0.0, z: 0.0};

    let mut cur_ray = ray;

    for bounce in 0..4 {
        let cur_hit = get_first_hit(&cur_ray, &scene.objects);

        let hit_color : Vec3; // = Vec3{x: 0.0, y: 0.0, z: 0.0};

        match cur_hit {
            Some(hit) => {
                hit_color = calculate_shading(&hit, &scene);
                cur_ray = Ray{origin: hit.position, direction: hit.normal};
                },
            None => break,
        };

        let contrib_factor = match bounce {
            0 => 0.6 ,
            1 => 0.3 ,
            2 => 0.1 ,
            _ => 0.0 ,
        };

        color += hit_color.mult(contrib_factor);

    }

    color
}

fn get_first_hit(ray : &Ray, spheres : &Vec<Sphere>) -> Option<Hit>
{
    let mut first_hit : Option<Hit> = None;
    let mut distance_to_closest = f32::MAX;

    for sphere in spheres {
        let cur_hit = sphere.get_first_collision(&ray);

        match cur_hit {
            Some(Hit{position, ..}) => {
                let d = position.length();
                if  d < distance_to_closest {
                    first_hit = cur_hit;
                    distance_to_closest = d;
                }
            },
            None => (),

        };
    };
    first_hit
}

fn calculate_shading(hit : &Hit, scene : &Scene) -> Vec3 {
    let zeros = Vec3{x: 0.0, y: 0.0, z: 0.0};
    let ones = Vec3{x: 1.0, y: 1.0, z: 1.0};

    let view_vec = hit.incident_dir.mult(-1.0);

    let mut diffuse = Vec3{x: 0.0, y: 0.0, z: 0.0};

    for light in &scene.lights {

        let light_vec = light.position - hit.position;
        let epsilon = 0.000001;
        let offset_pos = hit.position + hit.normal.mult(epsilon);

        let light_vec_normed = light_vec.normed();

        let occluder = get_first_hit(&Ray{origin: offset_pos, direction: light_vec_normed}, &scene.objects);

        let lv = (light_vec + view_vec).normed();

        let mut lambert = hit.normal.dot(&lv);

        match occluder {
            None => (),
            Some(occluder_hit) => {
                if (occluder_hit.position - hit.position).length() < light_vec.length(){
                    continue;
                }
            }
        };

        let contrib = light.color.mult(lambert).clamp(&zeros, &ones);
        
        diffuse += contrib;
    };

    diffuse = diffuse.clamp(&zeros, &ones);

    let ambient = Vec3{x: 0.1, y: 0.1, z: 0.1};

    let final_col = (ambient + diffuse).clamp(&zeros, &ones);

    final_col
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
