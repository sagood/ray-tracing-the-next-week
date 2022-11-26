use std::{
    io::{self, Write},
    sync::Arc,
};

use material::{diffuse_light::DiffuseLight, material::Material};
use model::{
    hit::{HitRecord, Hittable},
    moving_sphere::MovingSphere,
    r#box::Box,
    ray::Ray,
    vec3::Vec3,
    xy_rect::XyRect,
    xz_rect::XzRect,
    yz_rect::YzRect,
};
use Vec3 as Point3;

use texture::{checker::CheckerTexture, image::ImageTexture, noise::NoiseTexture};
use util::{
    rtweekend::INFINITY,
    rtweekend::{random_double, random_double_by_range},
};

use crate::{
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    model::{camera::Camera, color::Color, hit::HittableList, sphere::Sphere},
    util::rtweekend::PI,
};
mod material;
mod model;
mod texture;
mod util;

fn main() {
    // Image
    let mut ASPECT_RATIO: f64 = 16.0 / 9.0;
    let mut IMAGE_WIDTH: usize = 400;
    let mut SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::new();
    let mut lookfrom: Point3;
    let mut lookat: Point3;
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    let mut background = Vec3::new(0.0, 0.0, 0.0);

    let scene = 6;
    match scene {
        2 => {
            world = two_spheres();
            background = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        3 => {
            world = two_perlin_spheres();
            background = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        4 => {
            world = earth();
            background = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        5 => {
            world = simple_light();
            SAMPLES_PER_PIXEL = 400;
            background = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Point3::new(26.0, 3.0, 6.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        6 => {
            world = cornell_box();
            ASPECT_RATIO = 1.0;
            IMAGE_WIDTH = 600;
            SAMPLES_PER_PIXEL = 200;
            background = Point3::new(0.0, 0.0, 0.0);
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            world = random_scene();
            background = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Point3::new(13.0, 2.0, 3.0);
            lookat = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
    }

    let IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    // Camera
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH as f64 - 1.0);
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT as f64 - 1.0);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &background, &world, MAX_DEPTH);
            }

            let s = pixel_color.as_color_repr(SAMPLES_PER_PIXEL);
            print!("{}", s);
        }
    }
    eprintln!("\nDone.");
}

fn ray_color(r: &Ray, background: &Vec3, world: &dyn Hittable, depth: i32) -> Vec3 {
    let mut rec = HitRecord::default();

    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    // If the ray hits nothing, return the background color
    if !world.hit(r, 0.001, INFINITY, &mut rec) {
        return background.clone();
    }

    let mut scattered = Ray::new(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 0.0), 0.0);
    let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
    let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);

    if !rec
        .material
        .scatter(r, &rec, &mut attenuation, &mut scattered)
    {
        return emitted;
    }

    return emitted + attenuation * ray_color(&scattered, background, world, depth - 1);
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(&Vec3::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let checker = Arc::new(CheckerTexture::new_with_color(
        &Vec3::new(0.2, 0.3, 0.1),
        &Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_with_texture(checker)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    sphere_material = Arc::new(Lambertian::new(&albedo));
                    let center2 = center + Vec3::new(0.0, random_double_by_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_by_range(0.5, 1.0);
                    let fuzz = random_double_by_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(&albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(&Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(&Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn two_spheres() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_with_color(
        &Vec3::new(0.2, 0.3, 0.1),
        &Vec3::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_with_texture(checker.clone())),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new_with_texture(checker)),
    )));

    world
}

fn two_perlin_spheres() -> HittableList {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_with_texture(pertext.clone())),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_with_texture(pertext)),
    )));

    world
}

fn earth() -> HittableList {
    let mut world = HittableList::new();

    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg".to_owned()));
    let earth_surface = Arc::new(Lambertian::new_with_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    world.add(globe);

    world
}

fn simple_light() -> HittableList {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new_with_texture(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new_with_texture(pertext)),
    )));

    let difflight = Arc::new(DiffuseLight::new_with_color((Vec3::new(4.0, 4.0, 4.0))));
    world.add(Arc::new(XyRect::new(3.0, 5.0, 1.0, 4.0, -2.0, difflight)));

    world
}

fn cornell_box() -> HittableList {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(&Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(&Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(&Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_with_color(Vec3::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Arc::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    world.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    world.add(Arc::new(Box::new(
        &Point3::new(130.0, 0.0, 65.0),
        &Point3::new(295.0, 165.0, 230.0),
        white.clone(),
    )));
    world.add(Arc::new(Box::new(
        &Point3::new(265.0, 0.0, 295.0),
        &Point3::new(430.0, 330.0, 460.0),
        white.clone(),
    )));

    world
}
