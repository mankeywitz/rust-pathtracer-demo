use vec3::Vec3;
use material::Material;
use ray::Ray;

extern crate rand;

pub struct Hit<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Box<(dyn Material + Sync)>
}

pub trait Hitable {
    fn hit(&self, t_min: f32, t_max: f32, r: &Ray) -> Option<Hit>;
}
