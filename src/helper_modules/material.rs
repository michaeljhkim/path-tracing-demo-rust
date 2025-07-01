use std::f32::consts::PI;

use glam::Vec3;
use rand::random;

use super::ray::Ray;
use super::sphere::{HitResult};



pub struct ReflectResult {
    m_ray: Ray,
    m_color: Vec3
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

pub trait Material {
    fn reflect(&self, ray: Ray, hit: HitResult) -> ReflectResult;
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
    fn reflect(&self, ray: Ray, hit: HitResult) -> ReflectResult {
        let mut res: ReflectResult = ReflectResult::default();
        let u1: f32 = random::<f32>();
        let u2: f32 = random::<f32>();
        
        let theta: f32 = (1.0-u1).sqrt().acos();
        let phi: f32 = 2.0 * PI * u2;

        let mut random_direction: Vec3 = Vec3::new(
            theta.sin() * phi.cos(), 
            theta.sin() * phi.sin(), 
            theta.cos()
        ); 

        if random_direction.dot(hit.m_hit_normal) < 0.0 {
            random_direction *= -1.0;
        }

        res.m_ray = Ray::new(hit.m_hit_pos, random_direction);
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
    fn reflect(&self, ray: Ray, hit: HitResult) -> ReflectResult {
        let mut res: ReflectResult = ReflectResult::default();
        let mut incoming_dir: Vec3 = ray.direction();
        let magnitude: f32 = incoming_dir.length();

        if magnitude > 0.0 {
            incoming_dir /= magnitude;
        }

        let reflected_dir: Vec3 = incoming_dir - 2.0 * incoming_dir.dot(hit.m_hit_normal) * hit.m_hit_normal;

        res.m_ray = Ray::new(hit.m_hit_pos, reflected_dir);
        res.m_color = self.m_color;

        return res;
    }
}