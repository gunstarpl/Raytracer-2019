use std::cmp;
use std::ops;

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec2
{
    pub x: f32,
    pub y: f32
}

impl Vec2
{
    pub fn new(x: f32, y: f32) -> Vec2
    {
        Vec2
        {
            x,
            y
        }
    }

    pub fn random_direction() -> Vec2
    {
        let azimuth = rand::random::<f32>() * 2.0 * std::f32::consts::PI;

        Vec2
        {
            x: azimuth.cos(),
            y: azimuth.sin()
        }
    }

    pub fn dot(self, other: Vec2) -> f32
    {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(self, other: Vec2) -> f32
    {
        self.x * other.y - self.y * other.x
    }

    pub fn length_sqr(self) -> f32
    {
        self.dot(self)
    }

    pub fn length(self) -> f32
    {
        self.length_sqr().sqrt()
    }

    pub fn normalized(self) -> Vec2
    {
        self / self.length()
    }

    pub fn is_unit(self) -> bool
    {
        (self.length_sqr() - 1.0).abs() < 0.0001
    }
}

impl cmp::PartialEq for Vec2
{
    fn eq(&self, other: &Self) -> bool
    {
        self.x == other.x &&
        self.y == other.y
    }
}

impl ops::Add<Vec2> for Vec2
{
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2
    {
        Vec2
        {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl ops::Sub<Vec2> for Vec2
{
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2
    {
        Vec2
        {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl ops::Mul<f32> for Vec2
{
    type Output = Vec2;

    fn mul(self, other: f32) -> Vec2
    {
        Vec2
        {
            x: self.x * other,
            y: self.y * other
        }
    }
}

impl ops::Div<f32> for Vec2
{
    type Output = Vec2;

    fn div(self, other: f32) -> Vec2
    {
        Vec2
        {
            x: self.x / other,
            y: self.y / other
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn new()
    {
        assert_eq!(Vec2::new(2.0, 1.0), Vec2::new(2.0, 1.0));
        assert_eq!(Vec2::default(), Vec2::new(0.0, 0.0));

        let vector = Vec2::new(1.0, 2.0);
        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 2.0);
    }

    #[test]
    fn random()
    {
        for _ in 0..100
        {
            let direction = Vec2::random_direction();
            assert!(direction.is_unit());
        }
    }

    #[test]
    fn calculate()
    {
        let vec_a = Vec2::new(1.0, 2.0);
        let vec_b = Vec2::new(2.0, 4.0);
  
        assert_eq!(vec_a.dot(vec_b), 10.0);
        assert_eq!(vec_a.cross(vec_b), 0.0);
        assert_eq!(vec_a.length(), 2.236068);
        assert_eq!(vec_a.length_sqr(), 5.0);
        assert_eq!(vec_b.length(), vec_b.length_sqr().sqrt());
        assert_eq!(vec_b.length_sqr(), vec_b.length() * vec_b.length());
        assert_eq!(vec_a.normalized(), Vec2::new(0.4472136, 0.8944272));
        assert_eq!(vec_a.normalized(), vec_b.normalized());
        
        assert!(vec_a.normalized().is_unit());
        assert!(vec_b.normalized().is_unit());

        assert_eq!(vec_a, vec_a);
        assert_eq!(vec_a + vec_b, Vec2::new(3.0, 6.0));
        assert_eq!(vec_a - vec_b, Vec2::new(-1.0, -2.0));
        assert_eq!(vec_a * 4.0, Vec2::new(4.0, 8.0));
        assert_eq!(vec_b / 2.0, Vec2::new(1.0, 2.0));
    }
}
