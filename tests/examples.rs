mod examples
{
    use raytracer::math::Vec3;
    use raytracer::math::Vec4;
    use raytracer::image;
    use raytracer::render;
    use render::Object;
    use render::objects;
    use render::materials;

    fn save_and_test_example(name: &str, parameters: render::Parameters, scene: render::Scene)
    {
        let save_path = format!("examples/{}.json", name);
        let render_dir = format!("target/tests/examples/{}", name);
        let render_path = format!("{}/output.png", render_dir);

        let setup = render::Setup
        {
            parameters,
            scene
        };

        setup.save(save_path).expect("Saving setup file failed!");

        let _ = std::fs::remove_dir_all(render_dir);

        let test_parameters = render::Parameters
        {
            image_width: setup.parameters.image_width / 16,
            image_height: setup.parameters.image_height / 16,
            antialias_samples: 1,
            scatter_limit: 8,
            ..setup.parameters
        };

        let image = render::Renderer::new()
            .set_parameters(&test_parameters)
            .set_scene(&setup.scene)
            .render();

        image::Writer::new(image::FormatPNG::new())
            .input(&image).output(render_path)
            .save().expect("Failed to save rendered image!");
    }

    #[test]
    fn spheres()
    {
        let parameters = render::Parameters
        {
            image_width: 1024,
            image_height: 576,
            antialias_samples: 16,
            scatter_limit: 16,
            ..render::Parameters::default()
        };

        let camera = render::camera::Parameters::new()
            .set_origin(Vec3::new(0.0, -0.6, 0.0))
            .set_look_at(Some(Vec3::new(0.0, 1.0, -0.2)))
            .set_field_of_view(55.0);
        
        let scene = render::Scene::new()
            .set_camera(camera)
            .add_object(Object::new(objects::Sphere::new(Vec3::new(0.3, 0.5, -0.3), 0.2, materials::Refractive::new(Vec4::new(1.0, 1.0, 1.0, 1.0), 1.008))))
            .add_object(Object::new(objects::Sphere::new(Vec3::new(-0.3, 0.5, -0.3), -0.2, materials::Refractive::new(Vec4::new(1.0, 1.0, 1.0, 1.0), 1.3))))
            .add_object(Object::new(objects::Sphere::new(Vec3::new(0.0, 1.4, 0.0), 0.5, materials::Diffuse::new(Vec4::new(0.8, 0.3, 0.3, 1.0)))))
            .add_object(Object::new(objects::Sphere::new(Vec3::new(0.8, 1.0, -0.1), 0.4, materials::Metallic::new(Vec4::new(0.8, 0.8, 0.8, 1.0), 0.0))))
            .add_object(Object::new(objects::Sphere::new(Vec3::new(-0.8, 1.0, -0.1), 0.4, materials::Metallic::new(Vec4::new(0.8, 0.8, 0.8, 1.0), 0.8))))
            .add_object(Object::new(objects::Sphere::new(Vec3::new(0.0, 1.0, -100.5), 100.0, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.0, 1.0)))));

        save_and_test_example("spheres", parameters, scene);
    }

    #[test]
    fn metallic()
    {
        let parameters = render::Parameters
        {
            image_width: 1024,
            image_height: 200,
            antialias_samples: 16,
            scatter_limit: 16,
            ..render::Parameters::default()
        };

        let camera = render::camera::Parameters::new()
            .set_origin(Vec3::new(0.0, -5.5, 0.0))
            .set_look_at(Some(Vec3::new(0.0, 0.0, 0.0)))
            .set_field_of_view(20.0);
        
        let mut scene = render::Scene::new()
            .set_camera(camera)
            .add_object(Object::new(objects::Sphere::new(Vec3::new(0.0, 1.0, -600.5), 600.0, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.0, 1.0)))));

        for x in 0..=8
        {
            scene = scene.add_object(Object::new(
                objects::Sphere::new(Vec3::new(1.0 * (x as f32) - 4.0, 0.0, -0.002 * ((x - 4) as f32).abs()), 0.5,
                materials::Metallic::new(Vec4::new(0.9, 0.9, 0.9, 1.0), 1.0 / 8.0 * (x as f32)))
            ));
        }

        save_and_test_example("metallic", parameters, scene);
    }

    #[test]
    fn focus()
    {
        let parameters = render::Parameters
        {
            image_width: 1024,
            image_height: 576,
            antialias_samples: 16,
            scatter_limit: 16,
            ..render::Parameters::default()
        };

        let camera = render::camera::Parameters::new()
            .set_origin(Vec3::new(0.8, 1.2, 1.0))
            .set_look_at(Some(Vec3::new(0.0, 0.0, 0.0)))
            .set_field_of_view(55.0)
            .set_aperture_size(0.1)
            .set_focus_on_look_at(-0.25);
        
        let scene = render::Scene::new()
            .set_camera(camera)
            .add_object(Object::new(objects::Sphere::new(Vec3::new(0.0, 0.0, -100.5), 100.0, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.0, 1.0)))))
            .add_object(Object::new(objects::Sphere::new(Vec3::new(1.3, 0.0, 0.0), 0.5, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.3, 1.0)))))
            .add_object(Object::new(objects::Sphere::new(Vec3::new(-1.3, 0.0, 0.0), 0.5, materials::Diffuse::new(Vec4::new(0.3, 0.6, 0.3, 1.0)))))
            .add_object(Object::new(objects::Sphere::new(Vec3::new(0.0, 1.3, 0.0), 0.5, materials::Diffuse::new(Vec4::new(0.6, 0.2, 0.2, 1.0)))))
            .add_object(Object::new(objects::Sphere::new(Vec3::new(0.0, -1.3, 0.0), 0.5, materials::Diffuse::new(Vec4::new(0.3, 0.3, 0.6, 1.0)))))
            .add_object(Object::new(objects::Sphere::new(Vec3::new(1.0, 1.0, 0.0), 0.5, materials::Diffuse::new(Vec4::new(1.0, 0.3, 0.3, 1.0)))))
            .add_object(Object::new(objects::Sphere::new(Vec3::new(-1.0, -1.0, 0.0), 0.5, materials::Diffuse::new(Vec4::new(0.3, 1.0, 0.3, 1.0)))))
            .add_object(Object::new(objects::Sphere::new(Vec3::new(-1.0, 1.0, 0.0), 0.5, materials::Diffuse::new(Vec4::new(1.0, 0.6, 0.3, 1.0)))))
            .add_object(Object::new(objects::Sphere::new(Vec3::new(1.0, -1.0, 0.0), 0.5, materials::Diffuse::new(Vec4::new(0.3, 0.3, 1.0, 1.0)))))
            .add_object(Object::new(objects::Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.5, materials::Metallic::new(Vec4::new(0.8, 0.8, 0.8, 1.0), 0.0))));

        save_and_test_example("focus", parameters, scene);
    }

    #[test]
    fn velocity()
    {
        let parameters = render::Parameters
        {
            image_width: 1024,
            image_height: 200,
            antialias_samples: 16,
            scatter_limit: 16,
            ..render::Parameters::default()
        };

        let camera = render::camera::Parameters::new()
            .set_origin(Vec3::new(0.0, -5.5, 0.0))
            .set_look_at(Some(Vec3::new(0.0, 0.0, 0.0)))
            .set_field_of_view(20.0)
            .set_shutter_open_time(0.0)
            .set_shutter_close_time(1.0);
        
        let mut scene = render::Scene::new()
            .set_camera(camera)
            .add_object(Object::new(objects::Sphere::new(Vec3::new(0.0, 1.0, -600.5), 600.0, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.0, 1.0)))));

        for x in 0..=5
        {
            scene = scene.add_object(Object::new_moving(
                objects::Sphere::new(Vec3::new(1.0 * (x as f32) + 0.5 * (x as f32) - 4.0, 0.0, 0.0), 0.5,
                materials::Metallic::new(Vec4::new(0.9, 0.9, 0.9, 1.0), 0.5)),
                Vec3::new(0.1 * (x as f32), 0.0, 0.0)
            ));
        }

        save_and_test_example("velocity", parameters, scene);
    }

    #[test]
    fn diffuse()
    {
        let parameters = render::Parameters
        {
            image_width: 1024,
            image_height: 576,
            antialias_samples: 16,
            scatter_limit: 32,
            debug_mode: Some(render::DebugMode::Diffuse),
            ..render::Parameters::default()
        };

        let camera = render::camera::Parameters::new()
            .set_origin(Vec3::new(0.0, -0.6, 0.0))
            .set_look_at(Some(Vec3::new(0.0, 1.0, -0.2)))
            .set_field_of_view(55.0);
        
        let scene = render::Scene::new()
            .set_camera(camera)
            .add_object(Object::new(objects::Sphere::new(Vec3::new(0.0, 0.5, -0.1), 0.4, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.8, 1.0)))))
            .add_object(Object::new(objects::Sphere::new(Vec3::new(0.0, 1.0, -100.5), 100.0, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.8, 1.0)))));

        save_and_test_example("diffuse", parameters, scene);
    }

    #[test]
    fn normals()
    {
        let parameters = render::Parameters
        {
            image_width: 1024,
            image_height: 576,
            antialias_samples: 16,
            scatter_limit: 1,
            debug_mode: Some(render::DebugMode::Normals),
            ..render::Parameters::default()
        };

        let camera = render::camera::Parameters::new()
            .set_origin(Vec3::new(0.0, -0.6, 0.0))
            .set_look_at(Some(Vec3::new(0.0, 1.0, -0.2)))
            .set_field_of_view(55.0);
        
        let scene = render::Scene::new()
            .set_camera(camera)
            .add_object(Object::new(objects::Sphere::new(Vec3::new(0.0, 0.5, -0.1), 0.4, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.8, 1.0)))))
            .add_object(Object::new(objects::Sphere::new(Vec3::new(0.0, 1.0, -100.5), 100.0, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.8, 1.0)))));

        save_and_test_example("normals", parameters, scene);
    }

    #[test]
    fn benchmark()
    {
        let parameters = render::Parameters
        {
            image_width: 1024,
            image_height: 576,
            antialias_samples: 16,
            scatter_limit: 16,
            ..render::Parameters::default()
        };

        let camera = render::camera::Parameters::new()
            .set_origin(Vec3::new(0.0, -5.0, 0.8))
            .set_look_at(Some(Vec3::new(0.0, -3.0, 0.0)))
            .set_field_of_view(45.0);
        
        let mut scene = render::Scene::new()
            .set_camera(camera)
            .add_object(Object::new(objects::Sphere::new(Vec3::new(0.0, 1.0, -600.5), 600.0, materials::Diffuse::new(Vec4::new(0.8, 0.8, 0.0, 1.0)))));

        for x in 0..=10
        {
            for y in 0..=12
            {
                let offset = 0.5 * ((y % 2) as f32);

                scene = scene.add_object(Object::new(
                    objects::Sphere::new(Vec3::new(1.0 * (x as f32) - 5.0 + offset, 1.0 * (y as f32) - 6.0, 0.0), 0.5,
                    materials::Metallic::new(Vec4::new(0.8, 0.8, 0.8, 1.0), 0.6))
                ));
            }
        }

        save_and_test_example("benchmark", parameters, scene);
    }
}
