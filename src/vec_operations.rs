use std::ops::{Add, Sub, Mul, Div, Neg}; // TODO Rem, Not
use crate::vec_data::{RVecData, BaseRVecData};

// uses python adding rules. "1" + 1 = "11", "abc" + "def" = "abcdef", bool + bool = int, bool as int
impl Add for RVecData {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.len() == rhs.len() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 + y).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as f32 + y).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 + y).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| x + *y as i32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| x + *y as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 + *y as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 + y).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| x + *y as i64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x + *y as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 + *y as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 + y).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x + *y as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x + *y as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x + *y as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x + *y as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x + *y as i64 as f64).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Str(a.iter().zip(b.iter()).map(|(x, y)| format!("{}{}", x, y)).collect()),
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 + y).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 + y).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 as f32 + y).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 as f64 + y).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 + *y as i32).collect()),
                
                (a, b) => panic!("Cannot add {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(b.iter().map(|x| a[0] + x).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 + x).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 + x).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 + x).collect()),
                // (RVec::I32(a), RVec::Str(b))  not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(b.iter().map(|x| a[0] + *x as i32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(b.iter().map(|x| a[0] + *x as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] + x).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 + *x as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 + x).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(b.iter().map(|x| a[0] + *x as i64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(b.iter().map(|x| a[0] + *x as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 + *x as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] + x).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 + x).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(b.iter().map(|x| a[0] + *x as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(b.iter().map(|x| a[0] + *x as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] + *x as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 + *x as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] + x).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(b.iter().map(|x| a[0] + *x as i64 as f64).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Str(b.iter().map(|x| format!("{}{}", a[0], x)).collect()),
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 + *x).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 + *x).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 + *x).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 + *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 + *x as i32).collect()),

                (a, b) => panic!("Cannot add {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if rhs.is_scalar() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(a.iter().map(|x| x + b[0]).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(a.iter().map(|x| *x as i64 + b[0]).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| *x as f32 + b[0]).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 + b[0]).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(a.iter().map(|x| x + b[0] as i32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(a.iter().map(|x| x + b[0] as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(a.iter().map(|x| x + b[0]).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| *x as f64 + b[0] as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 + b[0]).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(a.iter().map(|x| x + b[0] as i64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().map(|x| x + b[0] as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 + b[0] as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| x + b[0]).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 + b[0]).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().map(|x| x + b[0] as i32 as f32).collect()),
                
                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().map(|x| x + b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| x + b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| x + b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| x + b[0]).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().map(|x| x + b[0] as i64 as f64).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| format!("{}{}", x, b[0])).collect()),
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 + *x).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 + *x).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 + *x).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 + *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 + *x as i32).collect()),

                (a, b) => panic!("Cannot add {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else {
            panic!("Unsupported lengths: {:?} + {:?}", self.len(), rhs.len())
        }
    }
}

// uses python's multiplication rules, eg. "abc" * 3 = "abcabcabc", bool as int, etc.
impl Mul for RVecData {
    type Output = RVecData;

    fn mul(self, rhs: RVecData) -> RVecData {
        if self.len() == rhs.len() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| x * y).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 * *y).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as f32 * *y).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 * *y).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| x * (*y as i32)).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| *x * *y as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| x * y).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 * *y as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 * *y).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| x * (*y as i64)).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x * *y as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 * *y as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x * y).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 * *y).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x * (*y as i32 as f32)).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x * *y as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x * *y as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x * *y as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x * y).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x * (*y as i64 as f64)).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported
                
                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| (*x as i32) * y).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| (*x as i64) * y).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| (*x as i32 as f32) * y).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| (*x as i64 as f64) * y).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().zip(b.iter()).map(|(x, y)| x & y).collect()),
                
                (a, b) => panic!("Cannot multiply {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(a.iter().map(|x| x * b[0]).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(a.iter().map(|x| *x as i64 * b[0]).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| *x as f32 * b[0]).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0]).collect()),
                (RVecData::I32(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| b[0].repeat(*x as usize)).collect()),
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(a.iter().map(|x| x * b[0] as i32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(a.iter().map(|x| x * b[0] as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(a.iter().map(|x| x * b[0]).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0] as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0]).collect()),
                (RVecData::I64(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| b[0].repeat(*x as usize)).collect()),
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(a.iter().map(|x| x * b[0] as i64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().map(|x| x * b[0] as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0] as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| x * b[0]).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0]).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().map(|x| x * b[0] as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().map(|x| x * b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| x * b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| x * b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| x * b[0]).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().map(|x| x * b[0] as i64 as f64).collect()),

                (RVecData::Str(a), RVecData::I32(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0] as usize)).collect()),
                (RVecData::Str(a), RVecData::I64(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0] as usize)).collect()),
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0].len())).collect()),
                (RVecData::Str(a), RVecData::Bool(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0] as usize)).collect()),

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 * x).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 * x).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 * x).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 * x).collect()),
                (RVecData::Bool(a), RVecData::Str(b)) => RVecData::Str(b.iter().map(|x| x.repeat(a[0] as usize)).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 * *x as i32).collect()),

                (a, b) => panic!("Cannot multiply {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if rhs.is_scalar() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(a.iter().map(|x| x * b[0]).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(a.iter().map(|x| *x as i64 * b[0]).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| *x as f32 * b[0]).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0]).collect()),
                (RVecData::I32(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| b[0].repeat(*x as usize)).collect()),
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(a.iter().map(|x| x * b[0] as i32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(a.iter().map(|x| x * b[0] as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(a.iter().map(|x| x * b[0]).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0] as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0]).collect()),
                (RVecData::I64(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| b[0].repeat(*x as usize)).collect()),
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(a.iter().map(|x| x * b[0] as i64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().map(|x| x * b[0] as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0] as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| x * b[0]).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 * b[0]).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().map(|x| x * b[0] as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().map(|x| x * b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| x * b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| x * b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| x * b[0]).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().map(|x| x * b[0] as i64 as f64).collect()),

                (RVecData::Str(a), RVecData::I32(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0] as usize)).collect()),
                (RVecData::Str(a), RVecData::I64(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0] as usize)).collect()),
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0].len())).collect()),
                (RVecData::Str(a), RVecData::Bool(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0] as usize)).collect()),

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 * x).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 * x).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 * x).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 * x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().map(|x| x & b[0]).collect()),

                (a, b) => panic!("Cannot multiply {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else {
            panic!("mismatched lengths {} and {}", self.len(), rhs.len());
        }
    }
}

impl Sub for RVecData {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.len() == rhs.len() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 - y).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as f32 - y).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 - y).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| x - *y as i32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| x - *y as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 - *y as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 - y).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| x - *y as i64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x - *y as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 - *y as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 - y).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| x - *y as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x - *y as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x - *y as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x - *y as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| x - *y as i64 as f64).collect()),
                
                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 - y).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 - y).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 as f32 - y).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 as f64 - y).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::I32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 - *y as i32).collect()),

                (a, b) => panic!("Unsupported types: {:?} - {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 - x).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 - x).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 - x).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 - x).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 - *x as i32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 - *x as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 - x).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 - x).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 - x).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 - *x as i64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 - *x as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 - *x as f32).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 - x).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 - x).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 - *x as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 - *x as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 - *x as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 - *x as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 - x).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 - *x as i64 as f64).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 - *x).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 - *x).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 - *x).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 - *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(b.iter().map(|x| a[0] ^ *x).collect()),

                (a, b) => panic!("Unsupported types: {:?} - {:?}", a.element_type(), b.element_type()),
            }
            
        } else if rhs.is_scalar() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::I32(a.iter().map(|x| x - b[0]).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::I64(a.iter().map(|x| *x as i64 - b[0]).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| *x as f32 - b[0]).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 - b[0]).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::I32(a.iter().map(|x| *x - b[0] as i32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::I64(a.iter().map(|x| *x - b[0] as i64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::I64(a.iter().map(|x| *x - b[0]).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| *x as f32 - b[0]).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 - b[0]).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::I64(a.iter().map(|x| *x - b[0] as i64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().map(|x| *x - b[0] as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 - b[0] as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| *x - b[0]).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 - b[0]).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().map(|x| *x - b[0] as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().map(|x| *x - b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x - b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| *x - b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x - b[0]).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().map(|x| *x - b[0] as i64 as f64).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::I32(b.iter().map(|x| a[0] as i32 - *x).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::I64(b.iter().map(|x| a[0] as i64 - *x).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 - *x).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 - *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(b.iter().map(|x| a[0] ^ *x).collect()),

                (a, b) => panic!("Unsupported types: {:?} - {:?}", a.element_type(), b.element_type()),
            }
        } else {
            panic!("mismatched lengths {} and {}", self.len(), rhs.len())
        }
    }
}

impl Div for RVecData {
    type Output = RVecData;

    fn div(self, rhs: RVecData) -> RVecData {
        if self.len() == rhs.len() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as f32 / *y as f32).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y as f64).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as f32 / *y).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as f32 / *y as i32 as f32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y as f64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y as f64).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y as i64 as f64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x / *y as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x / *y).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as f64 / *y).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x / *y as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x / *y as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x / *y as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x / *y as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x / *y).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x / *y as i64 as f64).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 as f32 / *y as f32).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 as f64 / *y as f64).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 as f32 / *y).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(a.iter().zip(b.iter()).map(|(x, y)| *x as i64 as f64 / *y).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::F32(a.iter().zip(b.iter()).map(|(x, y)| *x as i32 as f32 / *y as i32 as f32).collect()),

                (a, b) => panic!("unsupported types {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 / *x as f32).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x as f64).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 / *x).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::F32(b.iter().map(|x| a[0] as f32 / *x as i32 as f32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x as f64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x as f64).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x as i64 as f64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(b.iter().map(|x| a[0] / *x as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] / *x).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(b.iter().map(|x| a[0] / *x as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(b.iter().map(|x| a[0] / *x as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] / *x as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(b.iter().map(|x| a[0] as f64 / *x as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] / *x).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(b.iter().map(|x| a[0] / *x as i64 as f64).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 / *x as f32).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 / *x as f64).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 / *x).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 / *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 / *x as i32 as f32).collect()),

                (a, b) => panic!("unsupported types {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if rhs.is_scalar() {
            match (self, rhs) {
                (RVecData::I32(a), RVecData::I32(b)) => RVecData::F32(a.iter().map(|x| *x as f32 / b[0] as f32).collect()),
                (RVecData::I32(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0] as f64).collect()),
                (RVecData::I32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| *x as f32 / b[0]).collect()),
                (RVecData::I32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0]).collect()),
                // (RVec::I32(a), RVec::Str(b)) not supported
                (RVecData::I32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().map(|x| *x as f32 / b[0] as i32 as f32).collect()),

                (RVecData::I64(a), RVecData::I32(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0] as f64).collect()),
                (RVecData::I64(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0] as f64).collect()),
                (RVecData::I64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0] as f64).collect()),
                (RVecData::I64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0]).collect()),
                // (RVec::I64(a), RVec::Str(b)) not supported
                (RVecData::I64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0] as i64 as f64).collect()),

                (RVecData::F32(a), RVecData::I32(b)) => RVecData::F32(a.iter().map(|x| *x / b[0] as f32).collect()),
                (RVecData::F32(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0] as f64).collect()),
                (RVecData::F32(a), RVecData::F32(b)) => RVecData::F32(a.iter().map(|x| *x / b[0]).collect()),
                (RVecData::F32(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0]).collect()),
                // (RVec::F32(a), RVec::Str(b)) not supported
                (RVecData::F32(a), RVecData::Bool(b)) => RVecData::F32(a.iter().map(|x| *x / b[0] as i32 as f32).collect()),

                (RVecData::F64(a), RVecData::I32(b)) => RVecData::F64(a.iter().map(|x| *x / b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::I64(b)) => RVecData::F64(a.iter().map(|x| *x / b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F32(b)) => RVecData::F64(a.iter().map(|x| *x as f64 / b[0] as f64).collect()),
                (RVecData::F64(a), RVecData::F64(b)) => RVecData::F64(a.iter().map(|x| *x / b[0]).collect()),
                // (RVec::F64(a), RVec::Str(b)) not supported
                (RVecData::F64(a), RVecData::Bool(b)) => RVecData::F64(a.iter().map(|x| *x / b[0] as i64 as f64).collect()),

                // (RVec::Str(a), RVec::I32(b)) not supported
                // (RVec::Str(a), RVec::I64(b)) not supported
                // (RVec::Str(a), RVec::F32(b)) not supported
                // (RVec::Str(a), RVec::F64(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::I32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 / *x as f32).collect()),
                (RVecData::Bool(a), RVecData::I64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 / *x as f64).collect()),
                (RVecData::Bool(a), RVecData::F32(b)) => RVecData::F32(b.iter().map(|x| a[0] as i32 as f32 / *x).collect()),
                (RVecData::Bool(a), RVecData::F64(b)) => RVecData::F64(b.iter().map(|x| a[0] as i64 as f64 / *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::F32(vec![a[0] as i32 as f32 / b[0] as i32 as f32]),

                (a, b) => panic!("unsupported types {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else {
            panic!("mismatched lengths {} and {}", self.len(), rhs.len())
        }
    }
}

impl Neg for RVecData {
    type Output = RVecData;

    fn neg(self) -> Self::Output {
        match self {
            RVecData::I32(a) => RVecData::I32(a.iter().map(|x| -*x).collect()),
            RVecData::I64(a) => RVecData::I64(a.iter().map(|x| -*x).collect()),
            RVecData::F32(a) => RVecData::F32(a.iter().map(|x| -*x).collect()),
            RVecData::F64(a) => RVecData::F64(a.iter().map(|x| -*x).collect()),
            // RVec::Str(a) not supported
            RVecData::Bool(a) => RVecData::I32(a.iter().map(|x| -(*x as i32)).collect()),

            a => panic!("unsupported type {:?}", a.element_type()),
        }
    }
}
