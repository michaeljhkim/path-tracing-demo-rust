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
    pub fn new(&self, eye: Vec3, target: Vec3, up: Vec3, fov: f32, aspect_ratio: f32) -> Self {
        let theta: f32 = fov * PI / 180.0;
        let h: f32 = (theta / 2.0).tan(); 
        return Self {
            m_ndc_height: 2.0 * h,
            m_ndc_width: aspect_ratio * self.m_ndc_height,
            m_w: (eye - target).normalize(),
            m_u: (up.cross(self.m_w)).normalize(),
            m_v: self.m_w.cross(self.m_u),
            m_eye: eye
        };
    }

    pub fn generate_ray(&self, col: f32, row: f32) -> Ray {
        let direction: Vec3 = (col-0.5) * self.m_ndc_width * self.m_u + (row-0.5) * self.m_ndc_height * self.m_v - self.m_w;
        return Ray::new(self.m_eye, direction);
    }
}