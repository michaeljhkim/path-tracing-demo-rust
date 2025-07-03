use std::sync::Arc;

use glam::Vec3;
use super::material::{Material, Diffuse};
use super::ray::{Ray};

pub struct HitResult {
    pub m_is_hit: bool,
    pub m_hit_pos: Vec3,
    pub m_hit_normal: Vec3,
    pub m_hit_material: Arc<dyn Material>,
    pub m_t: f32
}

impl Default for HitResult {
    fn default() -> Self {
        return Self {
            m_is_hit: false,
            m_hit_pos: Vec3::ZERO,
            m_hit_normal: Vec3::ZERO,
            m_hit_material: Arc::new(Diffuse::default()),
            m_t: 0.0
        };
    }
}


pub struct Sphere {
    pub m_center: Vec3,
    pub m_radius: f32,
    pub m_pmaterial: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, r: f32, m: Arc<dyn Material>) -> Self {
        return Self { 
            m_center: center,
            m_radius: r,
            m_pmaterial: m,
        };
    }

    pub fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> HitResult {
        let mut hit_result = HitResult::default();

        let oc = ray.m_origin - self.m_center;

        // Quadratic coefficients
        let a = ray.m_direction.dot(ray.m_direction);
        let b = 2.0 * ray.m_direction.dot(oc);
        let c = oc.dot(oc) - self.m_radius * self.m_radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return hit_result; // no hit
        }

        let sqrt_d = discriminant.sqrt();
        let t1 = (-b - sqrt_d) / (2.0 * a);
        let t2 = (-b + sqrt_d) / (2.0 * a);

        let t_hit = if t1 >= min_t && t1 <= max_t {
            t1
        } else if t2 >= min_t && t2 <= max_t {
            t2
        } else {
            return hit_result; // both outside valid range
        };

        let hit_pos = ray.at(t_hit);
        let normal = (hit_pos - self.m_center) / self.m_radius;

        hit_result.m_is_hit = true;
        hit_result.m_t = t_hit;
        hit_result.m_hit_pos = hit_pos;
        hit_result.m_hit_normal = normal;
        hit_result.m_hit_material = self.m_pmaterial.clone(); // clone Rc ref

        return hit_result;
    }
}

