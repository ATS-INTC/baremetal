use alloc::string::ToString;
use alloc::{string::String, vec::Vec};
use alloc::vec;
use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use spin::{Lazy, Mutex};


static mut RNG: Lazy<Mutex<XorShiftRng>> = Lazy::new(|| Mutex::new(XorShiftRng::seed_from_u64(0x1020304050607080u64)));

pub type Matrix = Vec<Vec<u64>>;

#[allow(unused)]
pub fn gen_matrix(scale: usize) -> Matrix {
    let mut matrix = vec![vec![0u64; scale]; scale];
    for i in 0..scale {
        for j in 0..scale {
            matrix[i][j] = unsafe { RNG.lock().next_u64() } % 1000;
        }
    }
    matrix
}

#[allow(unused)]
pub fn matrix_multiply(a1: &Matrix, a2: &Matrix) -> Matrix
{
    let scale = a1.len();
    let mut matrix = vec![vec![0u64; scale]; scale];
    for i in 0..scale
    {
        for j in 0..scale
        {
            for k in 0..scale
            {
                matrix[i][j] += a1[i][k] * a2[k][j];
            }
        }
    }
    matrix
}

#[allow(unused)]
pub fn matrix_to_string(matrix: Matrix) -> String {
    let mut ans = String::new();
    let scale = matrix.len();
    for i in 0..scale {
        for j in 0..scale {
            ans += &matrix[i][j].to_string();
            ans += " ";
        }
    }
    ans.pop();
    ans
}

#[allow(unused)]
pub fn string_to_matrix(content: &str, scale: usize) -> Matrix {
    let mut matrix = vec![vec![0u64; scale]; scale];
    let vec_string: Vec<&str> = content.split(" ").collect();
    assert_eq!(scale * scale, vec_string.len());
    for i in 0..scale {
        for j in 0..scale {
            matrix[i][j] = vec_string[i * scale + j].parse::<u64>().unwrap();
            // println!("{}", ans[i][j]);
        }
    }
    matrix
}