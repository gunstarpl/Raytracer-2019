use std::cmp;
use super::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray
{
    origin: Vec3,
    direction: Vec3,
    time: f32
}

impl Default for Ray
{
    #[inline]
    fn default() -> Self
    {
        Self
        {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 1.0, 0.0),
            time: 0.0
        }
    }
}

impl Ray
{
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Self
    {
        debug_assert!(direction.is_unit());

        Self
        {
            origin,
            direction,
            time
        }
    }

    #[inline]
    pub fn origin(&self) -> Vec3
    {
        self.origin
    }

    #[inline]
    pub fn direction(&self) -> Vec3
    {
        self.direction
    }

    #[inline]
    pub fn time(&self) -> f32
    {
        self.time
    }

    #[inline]
    pub fn point_at(&self, length: f32) -> Vec3
    {
        debug_assert!(length >= 0.0);
        debug_assert!(self.direction.is_unit());

        self.origin + self.direction * length
    }

    #[inline]
    pub fn is_valid(&self) -> bool
    {
        self.direction.is_unit()
    }
}

impl cmp::PartialEq for Ray
{
    #[inline]
    fn eq(&self, other: &Self) -> bool
    {
        self.origin == other.origin &&
        self.direction == other.direction
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn new()
    {
        let origin = Vec3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(0.57735, 0.57735, 0.57735);
        let ray = Ray::new(origin, direction, 0.0);
        
        assert_eq!(ray, ray);
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);

        assert_eq!(Ray::default(), Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 0.0));
    }

    #[test]
    #[should_panic]
    fn new_bad_direction()
    {
        Ray::new(Vec3::default(), Vec3::new(1.0, 1.0, 1.0), 0.0);
    }

    #[test]
    fn point_at()
    {
        let origin = Vec3::new(1.0, 1.0, 1.0);
        let direction = Vec3::new(0.57735, 0.57735, 0.57735);
        let ray = Ray::new(origin, direction, 0.0);

        assert_eq!(ray.point_at(2.0), Vec3::new(2.1547, 2.1547, 2.1547));
    }

    #[test]
    #[should_panic]
    fn point_at_bad_direction()
    {
        Ray::new(Vec3::default(), Vec3::new(1.0, 1.0, 1.0), 0.0).point_at(1.0);
    }
    
    #[test]
    #[should_panic]
    fn point_at_bad_length()
    {
        Ray::new(Vec3::default(), Vec3::new(0.0, 1.0, 0.0), 0.0).point_at(-1.0);
    }
    
    #[test]
    fn validate()
    {
        let mut ray = Ray::default();

        ray.direction = Vec3::new(0.57735, 0.57735, 0.57735);
        assert!(ray.is_valid() == true);

        ray.direction = Vec3::new(1.0, 1.0, 1.0);
        assert!(ray.is_valid() == false);
    }
}