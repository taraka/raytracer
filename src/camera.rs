use crate::canvas::Canvas;
use crate::color::Color;
use crate::matrix::*;
use crate::ray::Ray;
use crate::tuple::*;
use crate::world::World;
use crate::EPSILON;
use crate::FP;
use std::f64::consts::PI;

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub half_width: FP,
    pub half_height: FP,
    pub fov: FP,
    pub transform: Matrix4,
    pub pixel_size: FP,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: FP) -> Self {
        let half_view = (fov / 2.0).tan();
        let aspect = (hsize as FP) / (vsize as FP);

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        Self {
            hsize,
            vsize,
            half_width,
            half_height,
            fov,
            pixel_size: (half_width * 2.0) / (hsize as FP),
            transform: Matrix4::identity(),
        }
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = ((px as FP) + 0.5) * self.pixel_size;
        let yoffset = ((py as FP) + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.transform.inverse() * point(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, w: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for x in 0..(self.hsize - 1) {
            for y in 0..(self.vsize - 1) {
                let ray = self.ray_for_pixel(x, y);
                let color = w.color_at(&ray);
                image.write_pixel(x, y, color);
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use crate::camera::*;

    #[test]
    fn create_camera() {
        let c = Camera::new(160, 120, PI / 2.0);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert!((c.fov - (PI / 2.0)).abs() < EPSILON);
    }

    #[test]
    fn camera_pixel_size_h() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!((c.pixel_size - 0.01).abs() < EPSILON);
    }

    #[test]
    fn camera_pixel_size_v() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!((c.pixel_size - 0.01).abs() < EPSILON);
    }

    #[test]
    fn ray_through_center() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r, Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, -1.0)));
    }

    #[test]
    fn ray_through_corner() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(
            r,
            Ray::new(point(0.0, 0.0, 0.0), vector(0.66519, 0.33259, -0.66851))
        );
    }

    #[test]
    fn ray_with_transform() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = rotation_y(PI / 4.0) * translation(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(
            r,
            Ray::new(
                point(0.0, 2.0, -5.0),
                vector((2.0_f64).sqrt() / 2.0, 0.0, -((2.0_f64).sqrt() / 2.0))
            )
        );
    }

    #[test]
    fn render_the_world() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        c.transform = Matrix4::view_transform(
            point(0.0, 0.0, -5.0),
            point(0.0, 0.0, 0.0),
            vector(0.0, 1.0, 0.0),
        );

        let image = c.render(&w);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
