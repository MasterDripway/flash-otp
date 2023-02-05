use std::{ops::{Add, Rem, Sub}, fs};

pub type DEFAULT = u8;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub array: Vec<DEFAULT>,
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            array: Vec::<DEFAULT>::new(),
        }
    }
    pub fn from(arr: Vec<DEFAULT>) -> Matrix {
        Matrix {
            array: arr,
        }
    }
    pub fn dump(&self, f: &str) -> std::io::Result<()>{
        fs::write(f, self.array.clone())?;
        Ok(())
    }
}

impl ToString for Matrix {
    fn to_string(&self) -> String {
        let ch_vec : String = self.array.iter()
        .map(|x| *x as char)
        .collect();
        return ch_vec;
    }
}

fn cyclic_sum(a : isize, b : isize) -> DEFAULT {
    ((a + b) % (DEFAULT::MAX as isize + 1)) as DEFAULT
}

fn cyclic_sub(a : isize, b : isize) -> DEFAULT {
    ((a - b) % (DEFAULT::MAX as isize)) as DEFAULT
}

fn min(a:usize, b:usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

// pub fn test() {
//     for i in 0..=255 {
//         for k in 0..=255 {
//             let o = cyclic_sum(k, i);
//             println!("{k} + {i} = {}", o);
//         }
//     } 
// }



impl<'a, 'b> Add<&'b Matrix> for &'a Matrix {
    type Output = Matrix;
    fn add(self : &'a Matrix, other : &'b Matrix) -> Matrix{
        let fix : usize = min(self.array.len(), other.array.len());
        let mut temp: Vec<DEFAULT> = Vec::new();
        for i in 0..fix {
            temp.push(cyclic_sum(self.array[i] as isize, other.array[i] as isize));
        }
        Matrix {
            array: temp,
        }
    }
}

impl<'a, 'b> Rem<&'b Matrix> for  &'a Matrix {
    type Output = Matrix;
    fn rem(self : &'a Matrix, other: &'b Matrix) -> Matrix {
        let fix:usize = min(self.array.len(), other.array.len());
        let mut temp : Vec<DEFAULT> = Vec::new();
        for i in 0..fix {
            temp.push(self.array[i] % other.array[i])
        }
        Matrix { 
            array: temp 
        }

    }
}

impl<'a, 'b> Sub<&'b Matrix> for &'a Matrix {
    type Output = Matrix;
    fn sub(self : &'a Matrix, other : &'b Matrix) -> Matrix{
        let fix : usize = min(self.array.len(), other.array.len());
        let mut temp: Vec<DEFAULT> = Vec::new();
        for i in 0..fix {
            temp.push(cyclic_sub(self.array[i] as isize, other.array[i] as isize));
        }
        Matrix {
            array: temp,
        }
    }
}