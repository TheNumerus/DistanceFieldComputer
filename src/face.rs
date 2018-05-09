use std::cell::RefCell;
use std::rc::Rc;
use vec3::Vec3;

#[derive(Debug)]
pub struct Face {
    pub verts: [Rc<RefCell<Vec3>>; 3],
    pub normal: Vec3,
}

impl Face {
    pub fn new(
        first: Rc<RefCell<Vec3>>,
        second: Rc<RefCell<Vec3>>,
        third: Rc<RefCell<Vec3>>,
    ) -> Face {
        let mut f = Face {
            verts: [first, second, third],
            normal: Vec3::new((0.0, 0.0, 1.0)),
        };
        f.compute_normal();
        f
    }

    pub fn compute_normal(&mut self) {
        let a = self.verts[0].borrow();
        let b = self.verts[1].borrow();
        let c = self.verts[2].borrow();
        let ac = a.delta(&b);
        let ab = a.delta(&c);
        let norm = ac.cross(&ab).normalized();
        self.normal = norm;
    }
}

impl Clone for Face {
    fn clone(&self) -> Face {
        Face::new(
            Rc::clone(&self.verts[0]),
            Rc::clone(&self.verts[1]),
            Rc::clone(&self.verts[2]),
        )
    }
}
