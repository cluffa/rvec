use std::ops::{Add, Sub, Mul, Div, Neg}; // TODO Rem, Not
use crate::{vec_data::{RVecData, BaseRVecData}, Fdef, Idef};

// uses python adding rules. "1" + 1 = "11", "abc" + "def" = "abcdef", bool + bool = int, bool as int
impl Add for RVecData {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.len() == rhs.len() {
            match (self, rhs) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Int(a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| *x as Fdef + y).collect()),
                // (RVec::Int(a), RVec::Str(b)) not supported
                (RVecData::Int(a), RVecData::Bool(b)) => RVecData::Int(a.iter().zip(b.iter()).map(|(x, y)| x + *y as Idef).collect()),

                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| x + *y as Fdef).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()),
                // (RVec::Float(a), RVec::Str(b)) not supported
                (RVecData::Float(a), RVecData::Bool(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| x + *y as Idef as Fdef).collect()),

                // (RVec::Str(a), RVec::Int(b)) not supported
                // (RVec::Str(a), RVec::Float(b)) not supported
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Str(a.iter().zip(b.iter()).map(|(x, y)| format!("{}{}", x, y)).collect()),
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::Int(b)) => RVecData::Int(a.iter().zip(b.iter()).map(|(x, y)| *x as Idef + y).collect()),
                (RVecData::Bool(a), RVecData::Float(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| *x as Idef as Fdef + y).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Int(a.iter().zip(b.iter()).map(|(x, y)| *x as Idef + *y as Idef).collect()),
                
                (a, b) => panic!("Cannot add {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, rhs) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Int(b.iter().map(|x| a[0] + x).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Float(b.iter().map(|x| a[0] as Fdef + x).collect()),
                // (RVec::Int(a), RVec::Str(b))  not supported
                (RVecData::Int(a), RVecData::Bool(b)) => RVecData::Int(b.iter().map(|x| a[0] + *x as Idef).collect()),

                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Float(b.iter().map(|x| a[0] + *x as Fdef).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Float(b.iter().map(|x| a[0] + x).collect()),
                // (RVec::Float(a), RVec::Str(b)) not supported
                (RVecData::Float(a), RVecData::Bool(b)) => RVecData::Float(b.iter().map(|x| a[0] + *x as Idef as Fdef).collect()),

                // (RVec::Str(a), RVec::Int(b)) not supported
                // (RVec::Str(a), RVec::Float(b)) not supported
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Str(b.iter().map(|x| format!("{}{}", a[0], x)).collect()),
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::Int(b)) => RVecData::Int(b.iter().map(|x| a[0] as Idef + *x).collect()),
                (RVecData::Bool(a), RVecData::Float(b)) => RVecData::Float(b.iter().map(|x| a[0] as Idef as Fdef + *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Int(b.iter().map(|x| a[0] as Idef + *x as Idef).collect()),

                (a, b) => panic!("Cannot add {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if rhs.is_scalar() {
            match (self, rhs) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Int(a.iter().map(|x| x + b[0]).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Float(a.iter().map(|x| *x as Fdef + b[0]).collect()),
                // (RVec::Int(a), RVec::Str(b)) not supported
                (RVecData::Int(a), RVecData::Bool(b)) => RVecData::Int(a.iter().map(|x| x + b[0] as Idef).collect()),

                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Float(a.iter().map(|x| x + b[0] as Fdef).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Float(a.iter().map(|x| x + b[0]).collect()),
                // (RVec::Float(a), RVec::Str(b)) not supported
                (RVecData::Float(a), RVecData::Bool(b)) => RVecData::Float(a.iter().map(|x| x + b[0] as Idef as Fdef).collect()),

                // (RVec::Str(a), RVec::Int(b)) not supported
                // (RVec::Str(a), RVec::Float(b)) not supported
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| format!("{}{}", x, b[0])).collect()),
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::Int(b)) => RVecData::Int(b.iter().map(|x| a[0] as Idef + *x).collect()),
                (RVecData::Bool(a), RVecData::Float(b)) => RVecData::Float(b.iter().map(|x| a[0] as Idef as Fdef + *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Int(b.iter().map(|x| a[0] as Idef + *x as Idef).collect()),

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
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Int(a.iter().zip(b.iter()).map(|(x, y)| x * y).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| *x as Fdef * *y).collect()),
                // (RVec::Int(a), RVec::Str(b)) not supported
                (RVecData::Int(a), RVecData::Bool(b)) => RVecData::Int(a.iter().zip(b.iter()).map(|(x, y)| x * (*y as Idef)).collect()),

                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| x * *y as Fdef).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| x * y).collect()),
                // (RVec::Float(a), RVec::Str(b)) not supported
                (RVecData::Float(a), RVecData::Bool(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| x * (*y as Idef as Fdef)).collect()),

                // (RVec::Str(a), RVec::Int(b)) not supported
                // (RVec::Str(a), RVec::Float(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported
                
                (RVecData::Bool(a), RVecData::Int(b)) => RVecData::Int(a.iter().zip(b.iter()).map(|(x, y)| (*x as Idef) * y).collect()),
                (RVecData::Bool(a), RVecData::Float(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| (*x as Idef as Fdef) * y).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().zip(b.iter()).map(|(x, y)| x & y).collect()),
                
                (a, b) => panic!("Cannot multiply {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, rhs) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Int(a.iter().map(|x| x * b[0]).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Float(a.iter().map(|x| *x as Fdef * b[0]).collect()),
                (RVecData::Int(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| b[0].repeat(*x as usize)).collect()),
                (RVecData::Int(a), RVecData::Bool(b)) => RVecData::Int(a.iter().map(|x| x * b[0] as Idef).collect()),

                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Float(a.iter().map(|x| x * b[0] as Fdef).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Float(a.iter().map(|x| x * b[0]).collect()),
                // (RVec::Float(a), RVec::Str(b)) not supported
                (RVecData::Float(a), RVecData::Bool(b)) => RVecData::Float(a.iter().map(|x| x * b[0] as Idef as Fdef).collect()),

                (RVecData::Str(a), RVecData::Int(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0] as usize)).collect()),
                // (RVec::Str(a), RVec::Float(b)) not supported
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0].len())).collect()),
                (RVecData::Str(a), RVecData::Bool(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0] as usize)).collect()),

                (RVecData::Bool(a), RVecData::Int(b)) => RVecData::Int(b.iter().map(|x| a[0] as Idef * x).collect()),
                (RVecData::Bool(a), RVecData::Float(b)) => RVecData::Float(b.iter().map(|x| a[0] as Idef as Fdef * x).collect()),
                (RVecData::Bool(a), RVecData::Str(b)) => RVecData::Str(b.iter().map(|x| x.repeat(a[0] as usize)).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Int(b.iter().map(|x| a[0] as Idef * *x as Idef).collect()),

                (a, b) => panic!("Cannot multiply {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if rhs.is_scalar() {
            match (self, rhs) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Int(a.iter().map(|x| x * b[0]).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Float(a.iter().map(|x| *x as Fdef * b[0]).collect()),
                (RVecData::Int(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| b[0].repeat(*x as usize)).collect()),
                (RVecData::Int(a), RVecData::Bool(b)) => RVecData::Int(a.iter().map(|x| x * b[0] as Idef).collect()),

                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Float(a.iter().map(|x| x * b[0] as Fdef).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Float(a.iter().map(|x| x * b[0]).collect()),
                // (RVec::Float(a), RVec::Str(b)) not supported
                (RVecData::Float(a), RVecData::Bool(b)) => RVecData::Float(a.iter().map(|x| x * b[0] as Idef as Fdef).collect()),

                (RVecData::Str(a), RVecData::Int(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0] as usize)).collect()),
                // (RVec::Str(a), RVec::Float(b)) not supported
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0].len())).collect()),
                (RVecData::Str(a), RVecData::Bool(b)) => RVecData::Str(a.iter().map(|x| x.repeat(b[0] as usize)).collect()),

                (RVecData::Bool(a), RVecData::Int(b)) => RVecData::Int(b.iter().map(|x| a[0] as Idef * x).collect()),
                (RVecData::Bool(a), RVecData::Float(b)) => RVecData::Float(b.iter().map(|x| a[0] as Idef as Fdef * x).collect()),
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
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Int(a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| *x as Fdef - y).collect()),
                // (RVec::Int(a), RVec::Str(b)) not supported
                (RVecData::Int(a), RVecData::Bool(b)) => RVecData::Int(a.iter().zip(b.iter()).map(|(x, y)| x - *y as Idef).collect()),

                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| x - *y as Fdef).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()),
                // (RVec::Float(a), RVec::Str(b)) not supported
                (RVecData::Float(a), RVecData::Bool(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| x - *y as Idef as Fdef).collect()),

                // (RVec::Str(a), RVec::Int(b)) not supported
                // (RVec::Str(a), RVec::Float(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::Int(b)) => RVecData::Int(a.iter().zip(b.iter()).map(|(x, y)| *x as Idef - y).collect()),
                (RVecData::Bool(a), RVecData::Float(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| *x as Idef as Fdef - y).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Int(a.iter().zip(b.iter()).map(|(x, y)| *x as Idef - *y as Idef).collect()),

                (a, b) => panic!("Unsupported types: {:?} - {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, rhs) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Int(b.iter().map(|x| a[0] as Idef - x).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Float(b.iter().map(|x| a[0] as Fdef - x).collect()),
                // (RVec::Int(a), RVec::Str(b)) not supported
                (RVecData::Int(a), RVecData::Bool(b)) => RVecData::Int(b.iter().map(|x| a[0] as Idef - *x as Idef).collect()),

                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Float(b.iter().map(|x| a[0] as Fdef - *x as Fdef).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Float(b.iter().map(|x| a[0] as Fdef - x).collect()),
                // (RVec::Float(a), RVec::Str(b)) not supported
                (RVecData::Float(a), RVecData::Bool(b)) => RVecData::Float(b.iter().map(|x| a[0] as Fdef - *x as Idef as Fdef).collect()),

                // (RVec::Str(a), RVec::Int(b)) not supported
                // (RVec::Str(a), RVec::Float(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::Int(b)) => RVecData::Int(b.iter().map(|x| a[0] as Idef - *x).collect()),
                (RVecData::Bool(a), RVecData::Float(b)) => RVecData::Float(b.iter().map(|x| a[0] as Idef as Fdef - *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(b.iter().map(|x| a[0] ^ *x).collect()),

                (a, b) => panic!("Unsupported types: {:?} - {:?}", a.element_type(), b.element_type()),
            }
            
        } else if rhs.is_scalar() {
            match (self, rhs) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Int(a.iter().map(|x| x - b[0]).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Float(a.iter().map(|x| *x as Fdef - b[0]).collect()),
                // (RVec::Int(a), RVec::Str(b)) not supported
                (RVecData::Int(a), RVecData::Bool(b)) => RVecData::Int(a.iter().map(|x| *x - b[0] as Idef).collect()),

                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Float(a.iter().map(|x| *x - b[0] as Fdef).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Float(a.iter().map(|x| *x - b[0]).collect()),
                // (RVec::Float(a), RVec::Str(b)) not supported
                (RVecData::Float(a), RVecData::Bool(b)) => RVecData::Float(a.iter().map(|x| *x - b[0] as Idef as Fdef).collect()),

                // (RVec::Str(a), RVec::Int(b)) not supported
                // (RVec::Str(a), RVec::Float(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::Int(b)) => RVecData::Int(b.iter().map(|x| a[0] as Idef - *x).collect()),
                (RVecData::Bool(a), RVecData::Float(b)) => RVecData::Float(b.iter().map(|x| a[0] as Idef as Fdef - *x).collect()),
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
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| *x as Fdef / *y as Fdef).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| *x as Fdef / *y).collect()),
                // (RVec::Int(a), RVec::Str(b)) not supported
                (RVecData::Int(a), RVecData::Bool(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| *x as Fdef / *y as Idef as Fdef).collect()),

                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| *x / *y as Fdef).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| *x / *y).collect()),
                // (RVec::Float(a), RVec::Str(b)) not supported
                (RVecData::Float(a), RVecData::Bool(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| *x / *y as Idef as Fdef).collect()),

                // (RVec::Str(a), RVec::Int(b)) not supported
                // (RVec::Str(a), RVec::Float(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::Int(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| *x as Idef as Fdef / *y as Fdef).collect()),
                (RVecData::Bool(a), RVecData::Float(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| *x as Idef as Fdef / *y).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Float(a.iter().zip(b.iter()).map(|(x, y)| *x as Idef as Fdef / *y as Idef as Fdef).collect()),

                (a, b) => panic!("unsupported types {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, rhs) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Float(b.iter().map(|x| a[0] as Fdef / *x as Fdef).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Float(b.iter().map(|x| a[0] as Fdef / *x).collect()),
                // (RVec::Int(a), RVec::Str(b)) not supported
                (RVecData::Int(a), RVecData::Bool(b)) => RVecData::Float(b.iter().map(|x| a[0] as Fdef / *x as Idef as Fdef).collect()),

                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Float(b.iter().map(|x| a[0] / *x as Fdef).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Float(b.iter().map(|x| a[0] / *x).collect()),
                // (RVec::Float(a), RVec::Str(b)) not supported
                (RVecData::Float(a), RVecData::Bool(b)) => RVecData::Float(b.iter().map(|x| a[0] / *x as Idef as Fdef).collect()),

                // (RVec::Str(a), RVec::Int(b)) not supported
                // (RVec::Str(a), RVec::Float(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::Int(b)) => RVecData::Float(b.iter().map(|x| a[0] as Idef as Fdef / *x as Fdef).collect()),
                (RVecData::Bool(a), RVecData::Float(b)) => RVecData::Float(b.iter().map(|x| a[0] as Idef as Fdef / *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Float(b.iter().map(|x| a[0] as Idef as Fdef / *x as Idef as Fdef).collect()),

                (a, b) => panic!("unsupported types {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if rhs.is_scalar() {
            match (self, rhs) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Float(a.iter().map(|x| *x as Fdef / b[0] as Fdef).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Float(a.iter().map(|x| *x as Fdef / b[0]).collect()),
                // (RVec::Int(a), RVec::Str(b)) not supported
                (RVecData::Int(a), RVecData::Bool(b)) => RVecData::Float(a.iter().map(|x| *x as Fdef / b[0] as Idef as Fdef).collect()),

                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Float(a.iter().map(|x| *x / b[0] as Fdef).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Float(a.iter().map(|x| *x / b[0]).collect()),
                // (RVec::Float(a), RVec::Str(b)) not supported
                (RVecData::Float(a), RVecData::Bool(b)) => RVecData::Float(a.iter().map(|x| *x / b[0] as Idef as Fdef).collect()),

                // (RVec::Str(a), RVec::Int(b)) not supported
                // (RVec::Str(a), RVec::Float(b)) not supported
                // (RVec::Str(a), RVec::Str(b)) not supported
                // (RVec::Str(a), RVec::Bool(b)) not supported

                (RVecData::Bool(a), RVecData::Int(b)) => RVecData::Float(b.iter().map(|x| a[0] as Idef as Fdef / *x as Fdef).collect()),
                (RVecData::Bool(a), RVecData::Float(b)) => RVecData::Float(b.iter().map(|x| a[0] as Idef as Fdef / *x).collect()),
                // (RVec::Bool(a), RVec::Str(b)) not supported
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Float(vec![a[0] as Idef as Fdef / b[0] as Idef as Fdef]),

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
            RVecData::Int(a) => RVecData::Int(a.iter().map(|x| -*x).collect()),
            RVecData::Float(a) => RVecData::Float(a.iter().map(|x| -*x).collect()),
            // RVec::Str(a) not supported
            RVecData::Bool(a) => RVecData::Int(a.iter().map(|x| -(*x as Idef)).collect()),

            a => panic!("unsupported type {:?}", a.element_type()),
        }
    }
}

