use glam::Vec3;


pub struct Ray {
    pub m_origin: Vec3,
    pub m_direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Self {
        return Self { 
            m_origin: *origin, 
            m_direction: *direction
        };
    }

    /*
    pub fn origin(&self) -> Vec3 {
        return self.m_origin;
    }
    */

    pub fn direction(&self) -> Vec3 {
        return self.m_direction;
    } 

    pub fn at(&self, t: f32) -> Vec3 {
        return self.m_origin + t * self.m_direction;
    }
}

impl Default for Ray {
    fn default() -> Self {
        return Self { 
            m_origin: Vec3::ZERO,
            m_direction: Vec3::ZERO
        };
    }
}