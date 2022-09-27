use crate::color::Color;
use crate::light::PointLight;
use crate::pattern::Pattern;
use crate::FP;
use crate::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Material {
    pub pattern: Pattern,
    pub ambient: FP,
    pub diffuse: FP,
    pub specular: FP,
    pub shininess: FP,
}

impl Material {
    pub fn new() -> Self {
        Self {
            pattern: Pattern::solid(Color::white()),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn lighting(
        &self,
        obj: &Shape,
        light: PointLight,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        let effective_color = self.pattern.color_at_object(obj, &point) * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;

        if in_shadow {
            return ambient;
        }

        let light_dot_normal = lightv.dot(&normalv);
        let diffuse: Color;
        let specular: Color;

        if light_dot_normal < 0.0 {
            diffuse = Color::black();
            specular = Color::black();
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = (-lightv).reflect(&normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);
            if reflect_dot_eye < 0.0 {
                specular = Color::black();
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        return ambient + diffuse + specular;
    }
}

#[cfg(test)]
mod tests {
    use crate::material::*;

    #[test]
    fn default_material() {
        let m = Material::new();

        assert_eq!(m.pattern, Pattern::solid(Color::white()));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::new();
        let p = point(0.0, 0.0, 0.0);

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        assert_eq!(
            Color::new(1.9, 1.9, 1.9),
            m.lighting(&Shape::sphere(), light, p, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_offset() {
        let m = Material::new();
        let p = point(0.0, 0.0, 0.0);

        let num = (2.0 as FP).sqrt() / 2.0;
        let eyev = vector(0.0, num, -num);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        assert_eq!(
            Color::new(1.0, 1.0, 1.0),
            m.lighting(&Shape::sphere(), light, p, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_with_eye_opposite_surface_offset_45() {
        let m = Material::new();
        let p = point(0.0, 0.0, 0.0);

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let num: FP = 0.736396;
        assert_eq!(
            Color::new(num, num, num),
            m.lighting(&Shape::sphere(), light, p, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_with_eye_inpath_of_reflection() {
        let m = Material::new();
        let p = point(0.0, 0.0, 0.0);

        let num = (2.0 as FP).sqrt() / 2.0;

        let eyev = vector(0.0, -num, -num);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 10.0, -10.0), Color::white());

        assert_eq!(
            Color::new(1.6364, 1.6364, 1.6364),
            m.lighting(&Shape::sphere(), light, p, eyev, normalv, false)
        );
    }

    #[test]
    fn lighting_with_subject_in_shadow() {
        let m = Material::new();
        let p = point(0.0, 0.0, 0.0);

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::white());

        assert_eq!(
            Color::new(0.1, 0.1, 0.1),
            m.lighting(&Shape::sphere(), light, p, eyev, normalv, true)
        );
    }

    #[test]
    fn lighting_with_pattern() {
        let mut m = Material::new();
        m.pattern = Pattern::stripe(Color::white(), Color::black());
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::white());

        assert_eq!(
            Color::white(),
            m.lighting(
                &Shape::sphere(),
                light,
                point(0.9, 0.0, 0.0),
                eyev,
                normalv,
                false
            )
        );
        assert_eq!(
            Color::black(),
            m.lighting(
                &Shape::sphere(),
                light,
                point(1.1, 0.0, 0.0),
                eyev,
                normalv,
                false
            )
        );
    }
}
