use crate::RVecData;

pub trait ElementLogic {
    fn and_ew(&self, other: &Self) -> Self;
    fn or_ew(&self, other: &Self) -> Self;
    fn xor_ew(&self, other: &Self) -> Self;
    fn not_ew(&self) -> Self;
}

impl ElementLogic for RVecData {
    fn and_ew(&self, other: &Self) -> Self {
        match (self, other) {
            (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().zip(b.iter()).map(|(x, y)| *x && *y).collect()),
            _ => panic!("Cannot perform element-wise AND on non-bool vectors"),
        }
    }

    fn or_ew(&self, other: &Self) -> Self {
        match (self, other) {
            (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().zip(b.iter()).map(|(x, y)| *x || *y).collect()),
            _ => panic!("Cannot perform element-wise OR on non-bool vectors"),
        }
    }

    fn xor_ew(&self, other: &Self) -> Self {
        match (self, other) {
            (RVecData::Bool(a), RVecData::Bool(b)) => RVecData::Bool(a.iter().zip(b.iter()).map(|(x, y)| *x ^ *y).collect()),
            _ => panic!("Cannot perform element-wise XOR on non-bool vectors"),
        }
    }

    fn not_ew(&self) -> Self {
        match self {
            RVecData::Bool(a) => RVecData::Bool(a.iter().map(|x| !x).collect()),
            _ => panic!("Cannot perform element-wise NOT on non-bool vectors"),
        }
    }
}