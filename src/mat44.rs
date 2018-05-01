use std::ops::Index;

pub struct Mat44 {
    pub coefficents: [[f32; 4]; 4],
}

impl Mat44 {
    pub fn new(
        c: (
            (f32, f32, f32, f32),
            (f32, f32, f32, f32),
            (f32, f32, f32, f32),
            (f32, f32, f32, f32),
        ),
    ) -> Mat44 {
        Mat44 {
            coefficents: [
                [(c.0).0, (c.0).1, (c.0).2, (c.0).3],
                [(c.1).0, (c.1).1, (c.1).2, (c.1).3],
                [(c.2).0, (c.2).1, (c.2).2, (c.2).3],
                [(c.3).0, (c.3).1, (c.3).2, (c.3).3],
            ],
        }
    }
}

impl Index<usize> for Mat44 {
    type Output = [f32];
    fn index(&self, num: usize) -> &[f32] {
        match num {
            0...3 => &self.coefficents[num],
            _ => panic!("Invalid index"),
        }
    }
}
