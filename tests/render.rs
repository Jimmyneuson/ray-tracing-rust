use math::ray::Ray;
use math::vector3::Vector3;
use ray_tracing::math;
use ray_tracing::utils;
use std::fs::read_to_string;
use utils::camera::Camera;
use utils::hittable::*;
use utils::ppm::RGBTriplet;
use utils::ppm::PPM;
use utils::sphere::Sphere;
use utils::world::World;

#[test]
fn sky() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u64;

    let origin = Vector3::new(0.0, 0.0, 0.0);

    let mut ppm = PPM::new(image_width, image_height);
    let cmp = read_to_string("./tests/sky.ppm").unwrap();

    let camera = Camera::new(
        2.0,
        aspect_ratio * 2.0,
        1.0,
        origin,
    );

    let lower_left_corner = camera.position
        - camera.horizontal / 2.0
        - camera.vertical / 2.0
        - Vector3::new(0.0, 0.0, camera.focal_length);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let mut r = Ray::new(
                camera.position,
                lower_left_corner
                    + u * camera.horizontal
                    + v * camera.vertical
                    - camera.position,
            );

            let unit_direction = r.direction.unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            let v = Vector3::lerp(
                Vector3::new(1.0, 1.0, 1.0),
                Vector3::new(0.5, 0.7, 1.0),
                t,
            )
            .color();

            ppm.set(
                i,
                image_height - j - 1,
                RGBTriplet::from_vector3(v),
            );
        }
    }

    assert_eq!(ppm.to_string(), cmp);
}

#[test]
fn sphere() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u64;

    let origin = Vector3::new(0.0, 0.0, 0.0);

    let mut ppm = PPM::new(image_width, image_height);
    let cmp = read_to_string("./tests/sphere.ppm").unwrap();

    let camera = Camera::new(
        2.0,
        aspect_ratio * 2.0,
        1.0,
        origin,
    );

    let lower_left_corner = camera.position
        - camera.horizontal / 2.0
        - camera.vertical / 2.0
        - Vector3::new(0.0, 0.0, camera.focal_length);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let mut r = Ray::new(
                camera.position,
                lower_left_corner
                    + u * camera.horizontal
                    + v * camera.vertical
                    - camera.position,
            );

            let mut world = World::default();
            world.add(Box::new(Sphere::new(
                Vector3::new(0.0, 0.0, -1.0),
                0.5,
            )));

            if let Some(_) = world.hit(&r, 0.0..=f64::MAX) {
                ppm.set(
                    i,
                    image_height - j - 1,
                    RGBTriplet::new(255, 0, 0),
                );
            } else {
                let unit_direction = r.direction.unit();
                let t = 0.5 * (unit_direction.y + 1.0);
                let v = Vector3::lerp(
                    Vector3::new(1.0, 1.0, 1.0),
                    Vector3::new(0.5, 0.7, 1.0),
                    t,
                )
                .color();

                ppm.set(
                    i,
                    image_height - j - 1,
                    RGBTriplet::from_vector3(v),
                );
            }
        }
    }

    assert_eq!(ppm.to_string(), cmp);
}

#[test]
fn normal_sphere() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u64;

    let origin = Vector3::new(0.0, 0.0, 0.0);

    let mut ppm = PPM::new(image_width, image_height);
    let cmp = read_to_string("./tests/normal_sphere.ppm").unwrap();

    let camera = Camera::new(
        2.0,
        aspect_ratio * 2.0,
        1.0,
        origin,
    );

    let lower_left_corner = camera.position
        - camera.horizontal / 2.0
        - camera.vertical / 2.0
        - Vector3::new(0.0, 0.0, camera.focal_length);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let mut r = Ray::new(
                camera.position,
                lower_left_corner
                    + u * camera.horizontal
                    + v * camera.vertical
                    - camera.position,
            );

            let mut world = World::default();
            world.add(Box::new(Sphere::new(
                Vector3::new(0.0, 0.0, -1.0),
                0.5,
            )));

            if let Some(hit) = world.hit(&r, 0.0..=f64::MAX) {
                let v =
                    0.5 * (hit.normal + Vector3::new(1.0, 1.0, 1.0)).color();
                ppm.set(
                    i,
                    image_height - j - 1,
                    RGBTriplet::from_vector3(v),
                );
            } else {
                let unit_direction = r.direction.unit();
                let t = 0.5 * (unit_direction.y + 1.0);
                let v = Vector3::lerp(
                    Vector3::new(1.0, 1.0, 1.0),
                    Vector3::new(0.5, 0.7, 1.0),
                    t,
                )
                .color();

                ppm.set(
                    i,
                    image_height - j - 1,
                    RGBTriplet::from_vector3(v),
                );
            }
        }
    }

    assert_eq!(ppm.to_string(), cmp);
}
