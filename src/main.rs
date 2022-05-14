#![allow(dead_code)]
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

extern crate png;

extern crate rand;

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

mod hitable;
mod sphere;
mod material;
mod camera;
mod texture;
mod aabb;

extern crate clap;
use clap::{Arg, App};

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;

    discriminant > 0.0
}
 
fn color(ray: &Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction = Vec3::unit_vector(ray.direction());

    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    //Setup args
    let matches = App::new("Pathtracer")
                        .arg(Arg::with_name("samples_per_pixel")
                                    .short("s")
                                    .long("spp")
                                    .help("Number of samples per pixel")
                                    .takes_value(true))
                        .arg(Arg::with_name("width")
                                    .short("w")
                                    .long("width")
                                    .help("Rendered image width")
                                    .takes_value(true))
                        .arg(Arg::with_name("height")
                                    .short("h")
                                    .long("height")
                                    .help("Rendered image height")
                                    .takes_value(true))
                        .arg(Arg::with_name("output")
                                    .short("o")
                                    .long("output")
                                    .help("Output image path")
                                    .takes_value(true))
                        .get_matches();

    let samples_per_pixel = matches.value_of("samples_per_pixel").unwrap_or("100");
    let image_width = matches.value_of("width").unwrap_or("480");
    let image_height = matches.value_of("height").unwrap_or("270");
    let output_filename = matches.value_of("output").unwrap_or("output.png");

    let samples_per_pixel = samples_per_pixel.parse::<usize>().unwrap();
    let image_width = image_width.parse::<u32>().unwrap();
    let image_height = image_height.parse::<u32>().unwrap();

    println!("Generating a {}x{}@{}spp render, saving to {}", image_width, image_height, samples_per_pixel, output_filename);

    // Setup camera rays
    let aspect_ratio = image_width as f32 / image_height  as f32;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::zero_vector();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    //Generate image
    let mut data = Vec::new();

    //Save start time
    let start_time = std::time::Instant::now();

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let u = x as f32 / (image_width - 1) as f32;
            let v = y as f32 / (image_height - 1) as f32;

            let ray = Ray::new(
                origin,
                lower_left_corner + (u * horizontal) + (v * vertical) - origin
            );

            let color = color(&ray);

            let ir = (255.99*color.x()) as u8;
            let ig = (255.99*color.y()) as u8;
            let ib = (255.99*color.z()) as u8;

            data.push(ir);
            data.push(ig);
            data.push(ib);
            data.push(255);
        }
        print!("{} / {} scanlines rendered \r", (image_height - y), image_height)
    }

    //Save end time
    let end_time = std::time::Instant::now();

    let render_duration = end_time.duration_since(start_time);
    let render_time_sec = render_duration.as_secs();
    let render_time_ms = render_duration.subsec_millis();

    println!("Render took {}.{} seconds", render_time_sec, render_time_ms);

    //Store image to file
    let path = Path::new(output_filename);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image_width, image_height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();


    writer.write_image_data(&data).unwrap();

    println!("Done");
}
