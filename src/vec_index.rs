use crate::{RVecData};

// TODO: slice

pub trait Indexing {
    /// bool is same length, and returns each element of a if the corresponding element of b is true
    /// int is any length, and returns each element of a if the corresponding element of b is in range
    fn getindex(&self, index: Self) -> Self;
}

impl Indexing for RVecData {
    fn getindex(&self, index: RVecData) -> Self {
        match (self, index) {
            (RVecData::Int(a), RVecData::Int(b)) => RVecData::Int(b.iter().map(|x| a[*x as usize]).collect()),
            (RVecData::Float(a), RVecData::Int(b)) => RVecData::Float(b.iter().map(|x| a[*x as usize]).collect()),
            (RVecData::Str(a), RVecData::Int(b)) => RVecData::Str(b.iter().map(|x| a[*x as usize].clone()).collect()),
            (RVecData::Bool(a), RVecData::Int(b)) => RVecData::Bool(b.iter().map(|x| a[*x as usize]).collect()),

            (RVecData::Int(a), RVecData::Bool(b)) => RVecData::Int(a.iter().zip(b.iter()).filter(|(_, x)| **x).map(|(x, _)| *x).collect()),
            (RVecData::Float(a), RVecData::Bool(b)) => RVecData::Float(a.iter().zip(b.iter()).filter(|(_, x)| **x).map(|(x, _)| *x).collect()),
            (RVecData::Str(a), RVecData::Bool(b)) => RVecData::Str(a.iter().zip(b.iter()).filter(|(_, x)| **x).map(|(x, _)| x.clone()).collect()),
            (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().zip(b.iter()).filter(|(_, x)| **x).map(|(x, _)| *x).collect()),
            _ => panic!("Invalid indexing"),
        }
    }
}