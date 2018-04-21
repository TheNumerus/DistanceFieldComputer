use vec3::Vec3;

#[derive(Debug)]
pub struct Face {
    pub verts: Vec<Vec3>,
    pub normal: Vec3,
    pub center: Vec3,
}

impl Face {
    pub fn new(first: Vec3, second: Vec3, third: Vec3) -> Face {
        let mut f = Face {
            verts: vec![first, second, third],
            normal: Vec3::new((0.0, 0.0, 1.0)),
            center: Vec3::new((0.0, 0.0, 0.0)),
        };
        f.compute_center();
        f.compute_normal();
        f
    }

    pub fn compute_normal(&mut self) {
        let ac = &self.verts[0] - &self.verts[1];
        let ab = &self.verts[0] - &self.verts[2];
        let norm = ac.cross(&ab).normalized();
        self.normal = norm;
    }

    pub fn compute_center(&mut self) {
        self.center = Vec3::new((
            (self.verts[0].x + self.verts[1].x + self.verts[2].x) / 3.0,
            (self.verts[0].y + self.verts[1].y + self.verts[2].y) / 3.0,
            (self.verts[0].z + self.verts[1].z + self.verts[2].z) / 3.0,
        ));
    }
}

impl Clone for Face {
    fn clone(&self) -> Face {
        Face::new(
            self.verts[0].clone(),
            self.verts[1].clone(),
            self.verts[2].clone(),
        )
    }
}
