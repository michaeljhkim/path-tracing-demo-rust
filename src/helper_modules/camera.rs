use std::f32::consts::PI;
use glam::Vec3;


use super::ray::Ray;


pub struct Camera {
    m_eye: Vec3,
    
    m_ndc_width: f32, 
    m_ndc_height: f32,

    m_u: Vec3, 
    m_v: Vec3, 
    m_w: Vec3
}


impl Camera {
    pub fn new(eye: Vec3, target: Vec3, up: Vec3, fov: f32, aspect_ratio: f32) -> Self {
        let theta: f32 = fov * PI / 180.0;
        let h: f32 = (theta / 2.0).tan();

        let mut new_instance: Self = Self::default();
        new_instance.m_ndc_height = 2.0 * h;
        new_instance.m_ndc_width = aspect_ratio * new_instance.m_ndc_height;
        new_instance.m_w = (eye - target).normalize();
        new_instance.m_u = (up.cross(new_instance.m_w)).normalize();
        new_instance.m_v = new_instance.m_w.cross(new_instance.m_u);
        new_instance.m_eye = eye;

        return new_instance;
    }

    pub fn generate_ray(&self, col: f32, row: f32) -> Ray {
        let direction: Vec3 = (col-0.5) * self.m_ndc_width * self.m_u + (row-0.5) * self.m_ndc_height * self.m_v - self.m_w;
        return Ray::new(&self.m_eye, &direction);
    }
}

impl Default for Camera {
    fn default() -> Self {
        return Self {
            m_eye: Vec3::ZERO,
            m_ndc_width: 0.0, 
            m_ndc_height: 0.0,
            m_u: Vec3::ZERO, 
            m_v: Vec3::ZERO, 
            m_w: Vec3::ZERO
        };
    }
}