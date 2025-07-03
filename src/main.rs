use std::sync::Arc;
use rand::{random, random_range};
use rayon::prelude::*;

use glam::Vec3;
use image::{ImageFormat, Rgb, RgbImage};

mod helper_modules;
use helper_modules::ray::Ray;

use crate::helper_modules::camera::Camera;
use crate::helper_modules::material::{Diffuse, ReflectResult, Specular};
use crate::helper_modules::sphere::HitResult;
use crate::helper_modules::world::World;


fn ray_hit_color(r: &Ray, world: &World, max_light_bounce_num: i32) -> Vec3 {
    if max_light_bounce_num <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let hit: HitResult = world.hit(r, 0.001, f32::INFINITY);
    
    if hit.m_is_hit {
        let material = hit.m_hit_material.clone();
        let res: ReflectResult = material.reflect(r, &hit); // now you can pass hit by value
        return res.m_color * ray_hit_color(&res.m_ray, world, max_light_bounce_num - 1);
    }

    return Vec3::new(1.0, 1.0, 1.0);
}

fn generate_results(world: &World, result_ppm_path : String) {
    let width = 640;
    let height = 480;
    let aspect_ratio = width as f32 / height as f32;
    let rays_per_pixel = 100;
    let max_light_bounce_num = 5;

    let eye: Vec3 = Vec3::new(20.0, 3.0, 3.0);
    let target: Vec3 = Vec3::ZERO;
    let up: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let fov = 20.0;

    let camera: Camera = Camera::new(eye, target, up, fov, aspect_ratio);

    // Create a new RGB image buffer
    let mut pixels = vec![Vec3::ZERO; (width * height) as usize];

    // Fill the image with pixel data
    pixels.par_iter_mut().enumerate().for_each(|(idx, pixel_color)| {
        // Just borrow camera immutably here
        let x = idx % (width as usize);
        let y = idx / (width as usize);

        let mut color_accum = Vec3::ZERO;

        for _ in 0..rays_per_pixel {
            let u = (x as f32 + random::<f32>()) / (width as f32 - 1.0);
            let v = (y as f32 + random::<f32>()) / (height as f32 - 1.0);

            // pass reference to camera
            let ray = camera.generate_ray(u, v);
            color_accum += ray_hit_color(&ray, world, max_light_bounce_num);
        }

        *pixel_color = color_accum;
    });


    // Create image buffer and write pixel data
    let mut img = RgbImage::new(width as u32, height as u32);
    for (idx, color) in pixels.iter().enumerate() {
        let x = idx % (width as usize);
        let y = idx / (width as usize);
        let scale = 1.0 / rays_per_pixel as f32;

        let gamma_corrected = Vec3::new(
            (color.x * scale).sqrt(),
            (color.y * scale).sqrt(),
            (color.z * scale).sqrt(),
        );

        let to_u8 = |v: f32| -> u8 {
            let clamped = v.clamp(0.0, 0.999);
            (clamped * 256.0) as u8
        };

        // Flip Y for image coordinates
        let flipped_y = height as u32 - y as u32 - 1;

        img.put_pixel(x as u32, flipped_y, Rgb([
            to_u8(gamma_corrected.x),
            to_u8(gamma_corrected.y),
            to_u8(gamma_corrected.z),
        ]));
    }

    img.save_with_format(result_ppm_path, ImageFormat::Png).unwrap();
}


fn main() {
    let mut world: World = World::default();
    
    /*
    world.generate_scene_one( Arc::new(Diffuse::new(Vec3::new(0.3, 0.4, 0.5))) );
    generate_results(&world, "1mdiffuse.png".to_string());

    world.generate_scene_one( Arc::new(Specular::new(Vec3::new(1.0, 1.0, 1.0))) );
    generate_results(&world, "1specular.png".to_string());
    
    let diffuse_color = Vec3::new(random::<f32>(), random::<f32>(), random::<f32>()) * Vec3::new(random::<f32>(), random::<f32>(), random::<f32>());
    world.generate_scene_multi(Arc::new(Diffuse::new(diffuse_color)));
    generate_results(&world, "mdiffuse.png".to_string());

    world.generate_scene_multi(Arc::new(Specular::new(Vec3::new(random_range(0.3..1.0), random_range(0.3..1.0), random_range(0.3..1.0)))));
    generate_results(&world, "mspecular.png".to_string());
    */

    world.generate_scene_all();
    generate_results(&world, "all.png".to_string());
}   
