use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

struct Bitmap {
    width: usize,
    height: usize,
    data: Box<[bool]>,
}

impl Bitmap {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![false; width * height].into_boxed_slice(),
        }
    }
}

impl Index<(usize, usize)> for Bitmap {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Bitmap {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[y * self.width + x]
    }
}

impl Debug for Bitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.width {
            for y in 0..self.height {
                write!(f, "{}", if self[(x, y)] { "#" } else { "." })?;
            }
            if x != self.width - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut bitmap = Bitmap::new(4, 5);
    // [false; false; false; false; false; false; false; false; false; false; false; false; false; false; false; false; false; false; false; false]
    // [(0,0); (1,0); (2,0); (3,0); (0,1); (1,1); (2,1); (3,1); (0,2); (1,2); (2,2); (3,2); (0,3); (1,3); (2,3); (3,3); (0,4); (1,4); (2,4); (3,4)]
    // [0    ; 1    ; 2    ; 3    ; 4    ; 5    ; 6    ; 7    ; 8    ; 9    ; 10   ; 11   ; 12   ; 13   ; 14   ; 15   ; 16   ; 17   ; 18   ; 19   ]
    dbg!(bitmap[(2, 4)]);
    bitmap[(2, 4)] = true;
    dbg!((bitmap, 123));
    println!("Hello, world!");
}
