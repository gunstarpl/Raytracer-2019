#![allow(dead_code)]

mod math;
mod render;
mod image;

use math::Vec3;
use math::Color;
use image::Image;
use render::primitive;
use render::material;

fn main() 
{
    let mut image = Image::new(1024, 576);

    let camera = render::Camera::new()
        .set_source_size(2.0, 2.0)
        .set_target_size(image.get_width(), image.get_height())
        .build();

    let scene = render::Scene::new()
        .add_primitive(primitive::Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material::Lambertian::from(Color::new(0.8, 0.3, 0.3, 1.0))))
        .add_primitive(primitive::Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material::Metalic::from(Color::new(0.8, 0.8, 0.8, 1.0))))
        .add_primitive(primitive::Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material::Metalic::from(Color::new(0.8, 0.8, 0.8, 1.0))))
        .add_primitive(primitive::Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material::Lambertian::from(Color::new(0.8, 0.8, 0.0, 1.0))));

    render::Renderer::new()
        .set_camera(&camera)
        .set_scene(&scene)
        .set_antialias_samples(8)
        .set_bounce_limit(32)
        .render(&mut image)
        .print_stats();

    image::Writer::new(image::FormatPNG::new())
        .input(&image).output("output/render.png").save()
        .expect("Failed to save rendered image!");
}
