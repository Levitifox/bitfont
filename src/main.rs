use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
    str::FromStr,
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
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if self[(x, y)] { "#" } else { "." })?;
            }
            if y != self.height - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseBitmapError;

impl FromStr for Bitmap {
    type Err = ParseBitmapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n").collect::<Vec<_>>();
        let height = if s.is_empty() { 0 } else { lines.len() };
        let width = lines[0].chars().count();
        let mut bitmap = Bitmap::new(width, height);
        for (y, line) in lines.into_iter().enumerate() {
            let chars = line.chars().collect::<Vec<_>>();
            if chars.len() != width {
                return Err(ParseBitmapError);
            }
            for (x, char) in chars.into_iter().enumerate() {
                bitmap[(x, y)] = match char {
                    '.' => false,
                    '#' => true,
                    _ => return Err(ParseBitmapError),
                };
            }
        }
        Ok(bitmap)
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
    let s = "\
...#...
..#.#..
.#####.
#.....#";
    dbg!(s.parse::<Bitmap>());
    println!("Hello, world!");
}
