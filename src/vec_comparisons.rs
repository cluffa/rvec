use crate::{Idef, Fdef, RVecData, vec_data::BaseRVecData};

pub trait ElementCmp {
    fn eq_ew(&self, other: &Self) -> RVecData;
    fn ne_ew(&self, other: &Self) -> RVecData;
    fn gt_ew(&self, other: &Self) -> RVecData;
    fn ge_ew(&self, other: &Self) -> RVecData;
    fn lt_ew(&self, other: &Self) -> RVecData;
    fn le_ew(&self, other: &Self) -> RVecData;
}

impl ElementCmp for RVecData {
    fn eq_ew(&self, other: &Self) -> RVecData {
        if self.len() == other.len() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x == y).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x == y).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| *x as Fdef == *y).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| *x == *y as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x == y).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x == y).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(b.iter().map(|x| a[0] == *x).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(b.iter().map(|x| a[0] == *x).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(b.iter().map(|x| a[0] as Fdef == *x).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(b.iter().map(|x| a[0] == *x as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(b.iter().map(|x| a[0] == *x).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(b.iter().map(|x| a[0] == *x).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if other.is_scalar() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(a.iter().map(|x| *x == b[0]).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(a.iter().map(|x| *x == b[0]).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(a.iter().map(|x| *x as Fdef == b[0]).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(a.iter().map(|x| *x == b[0] as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(a.iter().map(|x| *x == b[0]).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().map(|x| *x == b[0]).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else {
            panic!("Cannot compare vectors of different lengths: {} and {}", self.len(), other.len())
        }
    }

    fn ne_ew(&self, other: &Self) -> RVecData {
        if self.len() == other.len() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x != y).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x != y).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| *x as Fdef != *y).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| *x != *y as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x != y).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x != y).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(b.iter().map(|x| a[0] != *x).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(b.iter().map(|x| a[0] != *x).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(b.iter().map(|x| a[0] as Fdef != *x).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(b.iter().map(|x| a[0] != *x as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(b.iter().map(|x| a[0] != *x).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(b.iter().map(|x| a[0] != *x).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if other.is_scalar() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(a.iter().map(|x| *x != b[0]).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(a.iter().map(|x| *x != b[0]).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(a.iter().map(|x| *x as Fdef != b[0]).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(a.iter().map(|x| *x != b[0] as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(a.iter().map(|x| *x != b[0]).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().map(|x| *x != b[0]).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else {
            panic!("Cannot compare vectors of different lengths: {} and {}", self.len(), other.len())
        }
    }

    fn gt_ew(&self, other: &Self) -> RVecData {
        if self.len() == other.len() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x > y).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x > y).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| *x as Fdef > *y).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| *x > *y as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x > y).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x > y).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(b.iter().map(|x| a[0] > *x).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(b.iter().map(|x| a[0] > *x).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(b.iter().map(|x| a[0] as Fdef > *x).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(b.iter().map(|x| a[0] > *x as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(b.iter().map(|x| a[0] > *x).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(b.iter().map(|x| a[0] > *x).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if other.is_scalar() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(a.iter().map(|x| *x > b[0]).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(a.iter().map(|x| *x > b[0]).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(a.iter().map(|x| *x as Fdef > b[0]).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(a.iter().map(|x| *x > b[0] as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(a.iter().map(|x| *x > b[0]).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().map(|x| *x > b[0]).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else {
            panic!("Cannot compare vectors of different lengths: {} and {}", self.len(), other.len())
        }
    }

    fn ge_ew(&self, other: &Self) -> RVecData {
        if self.len() == other.len() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x >= y).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x >= y).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| *x as Fdef >= *y).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| *x >= *y as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x >= y).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x >= y).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(b.iter().map(|x| a[0] >= *x).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(b.iter().map(|x| a[0] >= *x).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(b.iter().map(|x| a[0] as Fdef >= *x).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(b.iter().map(|x| a[0] >= *x as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(b.iter().map(|x| a[0] >= *x).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(b.iter().map(|x| a[0] >= *x).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if other.is_scalar() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(a.iter().map(|x| *x >= b[0]).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(a.iter().map(|x| *x >= b[0]).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(a.iter().map(|x| *x as Fdef >= b[0]).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(a.iter().map(|x| *x >= b[0] as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(a.iter().map(|x| *x >= b[0]).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().map(|x| *x >= b[0]).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else {
            panic!("Cannot compare vectors of different lengths: {} and {}", self.len(), other.len())
        }
    }

    fn lt_ew(&self, other: &Self) -> RVecData {
        if self.len() == other.len() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x < y).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x < y).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| (*x as Fdef) < *y).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| *x < *y as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x < y).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x < y).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(b.iter().map(|x| a[0] < *x).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(b.iter().map(|x| a[0] < *x).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(b.iter().map(|x| (a[0] as Fdef) < *x).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(b.iter().map(|x| a[0] < *x as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(b.iter().map(|x| a[0] < *x).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(b.iter().map(|x| a[0] < *x).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if other.is_scalar() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(a.iter().map(|x| *x < b[0]).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(a.iter().map(|x| *x < b[0]).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(a.iter().map(|x| (*x as Fdef) < b[0]).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(a.iter().map(|x| *x < b[0] as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(a.iter().map(|x| *x < b[0]).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().map(|x| *x < b[0]).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else {
            panic!("Cannot compare vectors of different lengths: {} and {}", self.len(), other.len())
        }
    }

    fn le_ew(&self, other: &Self) -> RVecData {
        if self.len() == other.len() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x <= y).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x <= y).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| (*x as Fdef) <= *y).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| *x <= *y as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x <= y).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().zip(b).map(|(x, y)| x <= y).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if self.is_scalar() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(b.iter().map(|x| a[0] <= *x).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(b.iter().map(|x| a[0] <= *x).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(b.iter().map(|x| a[0] as Fdef <= *x).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(b.iter().map(|x| a[0] <= *x as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(b.iter().map(|x| a[0] <= *x).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(b.iter().map(|x| a[0] <= *x).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else if other.is_scalar() {
            match (self, other) {
                (RVecData::Int(a), RVecData::Int(b)) => RVecData::Bool(a.iter().map(|x| *x <= b[0]).collect()),
                (RVecData::Float(a), RVecData::Float(b)) => RVecData::Bool(a.iter().map(|x| *x <= b[0]).collect()),
                (RVecData::Int(a), RVecData::Float(b)) => RVecData::Bool(a.iter().map(|x| *x as Fdef <= b[0]).collect()),
                (RVecData::Float(a), RVecData::Int(b)) => RVecData::Bool(a.iter().map(|x| *x <= b[0] as Fdef).collect()),
                (RVecData::Str(a), RVecData::Str(b)) => RVecData::Bool(a.iter().map(|x| *x <= b[0]).collect()),
                (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().map(|x| *x <= b[0]).collect()),
                (a, b) => panic!("Cannot compare vectors of different types: {:?} and {:?}", a.element_type(), b.element_type()),
            }
        } else {
            panic!("Cannot compare vectors of different lengths: {} and {}", self.len(), other.len())
        }
    }
}

