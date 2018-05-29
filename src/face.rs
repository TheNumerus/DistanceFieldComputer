use std::sync::Arc;
use vec3::Vec3;

#[derive(Debug)]
pub struct Face {
    pub verts: [Arc<Vec3>; 3],
    pub normal: Vec3,
}

impl Face {
    pub fn new(first: Arc<Vec3>, second: Arc<Vec3>, third: Arc<Vec3>) -> Face {
        let mut f = Face {
            verts: [first, second, third],
            normal: Vec3::new((0.0, 0.0, 1.0)),
        };
        f.compute_normal();
        f
    }

    pub fn compute_normal(&mut self) {
        let a = &self.verts[0];
        let b = &self.verts[1];
        let c = &self.verts[2];
        let ac = a.delta(&b);
        let ab = a.delta(&c);
        let norm = ac.cross(&ab).normalized();
        self.normal = norm;
    }
}

impl Clone for Face {
    fn clone(&self) -> Face {
        Face::new(Arc::clone(&self.verts[0]), Arc::clone(&self.verts[1]), Arc::clone(&self.verts[2]))
    }
}
