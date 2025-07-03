use glam::Vec3;
use rand::{random, random_range};
use std::sync::Arc;

use crate::helper_modules::material::Specular;

use super::material::{Material, Diffuse};
use super::ray::{Ray};
use super::sphere::{HitResult, Sphere};

pub struct World {
    m_spheres: Vec<Arc<Sphere>>,
}

impl World {
    // remember to clone whatever value of max_t to input
    pub fn hit(&self, ray: &Ray, min_t: f32, mut max_t: f32) -> HitResult {
        let mut hit_result: HitResult = HitResult::default();

        for curr_sphere in self.m_spheres.iter() {
            let hit: HitResult = curr_sphere.hit(&ray, min_t, max_t);

            if hit.m_is_hit {
                hit_result = hit;
                max_t = hit_result.m_t;
            }
        }

        return hit_result;
    }

    pub fn add_scene_floor(&mut self) {
        let material_floor: Arc<Diffuse> = Arc::new(Diffuse::new(Vec3::new(0.5, 0.5, 0.5)));
        self.m_spheres.push(
            Arc::new(Sphere::new(Vec3::new(0.0, -2000.0, 0.0), 2000.0, material_floor))
        );
    }

    pub fn generate_spheres<F>(&mut self,
        row_start: i32, row_end: i32,
        col_start: i32, col_end: i32,
        spacing: f32, min_radius: f32, max_radius: f32,
        material_generator: F) 
    where
        F: Fn() -> Arc<dyn Material>,
    {
        for row in row_start..row_end {
            for col in col_start..col_end {
                let radius: f32 = random_range(min_radius..=max_radius);
                let center: Vec3 = Vec3::new(
                    (spacing as f32) * (row as f32) + 0.5 * random::<f32>(),
                    radius as f32,
                    (spacing as f32) * (col as f32) + 0.5 * random::<f32>(),
                );
                self.m_spheres.push(
                    Arc::new(Sphere::new(center, radius as f32, material_generator()))
                );
            }
        }
    }

    pub fn generate_scene_one(&mut self, sphere_material: Arc<dyn Material>) {
        self.m_spheres.clear();

        self.m_spheres.push(
            Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, sphere_material))
        );
        self.add_scene_floor();
    }

    pub fn generate_scene_multi(&mut self, sphere_material: Arc<dyn Material>) {
        self.m_spheres.clear();
        let closure_annotated = || -> Arc<dyn Material> {  
            sphere_material.clone()
        };

        self.generate_spheres(-3, 3, -3, 3, 3.0, 0.2, 0.8, closure_annotated);
        self.add_scene_floor();
    }

    pub fn generate_scene_all(&mut self) {
        self.m_spheres.clear();

        let closure_annotated = || -> Arc<dyn Material> { 
            let is_diffuse: bool = random::<f32>() <= 0.6;
            let color: Vec3 = if is_diffuse { 
                    Vec3::new(random::<f32>(), random::<f32>(), random::<f32>()) * Vec3::new(random::<f32>(), random::<f32>(), random::<f32>())
                } else {
                    Vec3::new(random_range(0.5..1.0), random_range(0.5..1.0), random_range(0.5..1.0)) 
                };
                
            if is_diffuse {
                Arc::new(Diffuse::new(color))
            }
            else {
                Arc::new(Specular::new(color))
            }
        };

        self.generate_spheres(-5, 10, -5, 5, 1.5, 0.2, 0.5, closure_annotated);
        self.add_scene_floor();
    }

}

impl Default for World {
    fn default() -> Self {
        return Self {
            m_spheres: Vec::default()
        };
    }
}