use std::{fmt};

use rand::Rng;
use crate::backend::matrix::{DEFAULT, Matrix};

pub struct Binary(String);

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


// fn expand<T: Copy>(o:T, len:usize) -> Vec<T> {
//     let mut temp: Vec<T> = Vec::new();
//     for _ in 0..len {
//         temp.push(o);
//     }
//     temp
// }

pub fn random_vec(len: usize) -> Vec<DEFAULT> {
    let mut rng = rand::thread_rng();
    let mut out: Vec<DEFAULT> = Vec::new();
    for _ in 0..len {
        let g : DEFAULT =  rng.gen();
        out.push(g);
    }
    out
}

pub fn encode(i: &Matrix, key: &Matrix) -> Matrix {
    i + key 
}

pub fn decode(i: &Matrix, key: &Matrix) -> Matrix {
    i - key
}

// pub fn to_bin(num: u8) -> Binary {
//     Binary(format!("{:b}", num))
// }

// pub fn as_utf16(s: &str) -> Vec<u16> {
//     s.encode_utf16().collect()
// }
