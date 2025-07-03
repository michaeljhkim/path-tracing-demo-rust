use std::f32::consts::PI;

use glam::Vec3;
use rand::random;

use super::ray::Ray;
use super::sphere::{HitResult};



pub struct ReflectResult {
    pub m_ray: Ray,
    pub m_color: Vec3
}

impl Default for ReflectResult {
    fn default() -> Self {
        return Self { 
            m_ray: Ray::default(),
            m_color: Vec3::ZERO,
        };
    }
}

/*
pub struct Material {
    m_color: Vec3,
}
 */

pub trait Material: Send + Sync {
    fn reflect(&self, ray: &Ray, hit: &HitResult) -> ReflectResult;
}

pub struct Diffuse {
    m_color: Vec3
}

impl Diffuse {
    pub fn new(color: Vec3) -> Self {
        return Self {
            m_color: color
        };
    }
}

impl Default for Diffuse {
    fn default() -> Self {
        return Self {
            m_color: Vec3::ZERO
        };
    }
}

impl Material for Diffuse {
    fn reflect(&self, ray: &Ray, hit: &HitResult) -> ReflectResult {
        let mut res = ReflectResult::default();

        // Cosine-weighted hemisphere sampling
        let u1 = random::<f32>();
        let u2 = random::<f32>();

        let r = u1.sqrt();
        let theta = 2.0 * PI * u2;

        // Sample direction in local tangent space (z-up hemisphere)
        let local_dir = Vec3::new(
            r * theta.cos(),
            r * theta.sin(),
            (1.0 - u1).sqrt()
        );

        // Build orthonormal basis (Tangent, Bitangent, Normal)
        let normal = hit.m_hit_normal.normalize();
        let tangent = if normal.x.abs() > 0.1 {
            Vec3::new(0.0, 1.0, 0.0).cross(normal).normalize()
        } else {
            Vec3::new(1.0, 0.0, 0.0).cross(normal).normalize()
        };
        let bitangent = normal.cross(tangent);

        // Transform local_dir to world space
        let world_dir = tangent * local_dir.x + bitangent * local_dir.y + normal * local_dir.z;

        // Offset ray origin to avoid self-intersection
        let epsilon = 1e-4;
        let offset_pos = hit.m_hit_pos + normal * epsilon;

        res.m_ray = Ray::new(&offset_pos, &world_dir.normalize());
        res.m_color = self.m_color;

        return res;
    }
}



pub struct Specular {
    m_color: Vec3
}

impl Specular {
    pub fn new(color: Vec3) -> Self {
        return Self {
            m_color: color
        };
    }
}

impl Default for Specular {
    fn default() -> Self {
        return Self {
            m_color: Vec3::ZERO
        };
    }
}

impl Material for Specular {
    fn reflect(&self, ray: &Ray, hit: &HitResult) -> ReflectResult {
        let mut res = ReflectResult::default();

        // Normalize direction and normal
        let incoming_dir = -ray.direction().normalize(); // Negate because ray points outward
        let normal = hit.m_hit_normal.normalize();

        // Mirror reflection
        let reflected_dir = incoming_dir - 2.0 * incoming_dir.dot(normal) * normal;

        // Offset the hit position slightly along the normal to prevent self-intersection
        let epsilon = 1e-4;
        let offset_pos = hit.m_hit_pos + normal * epsilon;

        res.m_ray = Ray::new(&offset_pos, &reflected_dir);
        res.m_color = self.m_color;

        return res;
    }
}