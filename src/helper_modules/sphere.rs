/*
class HitResult {
public:
    HitResult() { m_is_hit = false; };
    bool m_is_hit;
    Vec3D m_hit_pos;
    Vec3D m_hit_normal;
    shared_ptr<Material> m_hit_material;
    float m_t;
};
*/

use std::rc::Rc;

use glam::Vec3;
use super::material::{Material, Diffuse};
use super::ray::{Ray};

pub struct HitResult {
    pub m_is_hit: bool,
    pub m_hit_pos: Vec3,
    pub m_hit_normal: Vec3,
    pub m_hit_material: Rc<dyn Material>,
    pub m_t: f32
}

impl Default for HitResult {
    fn default() -> Self {
        return Self {
            m_is_hit: false,
            m_hit_pos: Vec3::ZERO,
            m_hit_normal: Vec3::ZERO,
            m_hit_material: Rc::new(Diffuse::default()),
            m_t: 0.0
        };
    }
}


pub struct Sphere {
    pub m_center: Vec3,
    pub m_radius: f32,
    pub m_pmaterial: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, r: f32, m: Rc<dyn Material>) -> Self {
        return Self { 
            m_center: center,
            m_radius: r,
            m_pmaterial: m,
        };
    }

    pub fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> HitResult {
        let mut hit_result: HitResult = HitResult::default();

        let oc: Vec3 = ray.m_origin - self.m_center;
        let doc: f32 = ray.m_direction.dot(oc);
        let discriminant: f32 = (doc * doc) - ray.m_direction.dot(ray.m_direction) * (oc.dot(oc) - self.m_radius * self.m_radius);

        if discriminant > 0.0 {
            let sqrt_discriminant: f32 = discriminant.sqrt();
            
            let mut ray_time: f32 = -doc - sqrt_discriminant;
            if ray_time < min_t || ray_time > max_t {
                ray_time = -doc + sqrt_discriminant;
            }

            if ray_time >= min_t && ray_time <= max_t {
                hit_result.m_is_hit = true;
                hit_result.m_t = ray_time;
                hit_result.m_hit_pos = ray.at(ray_time);
                hit_result.m_hit_normal = (hit_result.m_hit_pos - self.m_center) / self.m_radius;
                hit_result.m_hit_material = self.m_pmaterial.clone();   // only clones the reference
                return hit_result;
            }
        }

        hit_result.m_is_hit = false;
        return hit_result;
    }
}

