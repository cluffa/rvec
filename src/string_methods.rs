use crate::{vec_data::RVecData, Idef};

/// A trait for PythonStr that implements string methods, on strings
pub trait PyStringMethods {
    fn capitalize(&self) -> String;
    fn center(&self, width: usize, fill_char: char) -> String;
    fn count(&self, sub: &str) -> usize;
    fn endswith(&self, suffix: &str) -> bool;
    fn startswith(&self, prefix: &str) -> bool;
    fn find(&self, sub: &str) -> Option<usize>;
    // fn join(&self, sep: &str) -> String;
    fn lower(&self) -> String;
    fn upper(&self) -> String;
    fn replace(&self, old: &str, new: &str, count: Option<usize>) -> String;
    fn split(&self, sep: &str) -> Vec<&str>;
    fn strip(&self) -> String;
    fn lstrip(&self) -> String;
    fn rstrip(&self) -> String;
}

impl PyStringMethods for str {
    fn capitalize(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().chain(chars.flat_map(|c| c.to_lowercase())).collect(),
        }
    }

    fn center(&self, width: usize, fill_char: char) -> String {
        let len = self.chars().count();
        if width <= len {
            return self.to_owned();
        }
        let pad_left = (width - len) / 2;
        let pad_right = width - len - pad_left;
        format!("{}{}{}", fill_char.to_string().repeat(pad_left), self, fill_char.to_string().repeat(pad_right))
    }

    fn count(&self, sub: &str) -> usize {
        self.matches(sub).count()
    }

    fn endswith(&self, suffix: &str) -> bool {
        self.ends_with(suffix)
    }

    fn startswith(&self, prefix: &str) -> bool {
        self.starts_with(prefix)
    }

    fn find(&self, sub: &str) -> Option<usize> {
        self.find(sub)
    }

    fn lower(&self) -> String {
        self.to_lowercase()
    }

    fn upper(&self) -> String {
        self.to_uppercase()
    }

    fn replace(&self, old: &str, new: &str, count: Option<usize>) -> String {
        self.replacen(old, new, count.unwrap_or(usize::MAX))
    }

    fn split(&self, sep: &str) -> Vec<&str> {
        self.split(sep).collect()
    }

    fn strip(&self) -> String {
        self.trim().to_owned()
    }

    fn lstrip(&self) -> String {
        self.trim_start().to_owned()
    }

    fn rstrip(&self) -> String {
        self.trim_end().to_owned()
    }
}

pub trait VecStringMethods {
    fn capitalize(&self) -> Self;
    fn center(&self, width: usize, fill_char: char) -> Self;
    fn count(&self, sub: &str) -> Self;
    fn endswith(&self, suffix: &str) -> Self;
    fn startswith(&self, prefix: &str) -> Self;
    fn find(&self, sub: &str) -> Self;
    // fn join(&self, iter: impl Iterator<Item = &str>) -> Self;
    fn lower(&self) -> Self;
    fn upper(&self) -> Self;
    fn replace(&self, old: &str, new: &str) -> Self;
    fn split(&self, sep: &str) -> Self;
    fn strip(&self) -> Self;
    fn lstrip(&self) -> Self;
    fn rstrip(&self) -> Self;
}

impl VecStringMethods for RVecData {
    fn capitalize(&self) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.capitalize()).collect()),
            _ => panic!("capitalize() called on non-string"),
        }
    }

    fn center(&self, width: usize, fill_char: char) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.center(width, fill_char)).collect()),
            _ => panic!("center() called on non-string"),
        }
    }

    fn count(&self, sub: &str) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Int(s.iter().map(|s| s.count(sub) as Idef).collect()),
            _ => panic!("count() called on non-string"),
        }
    }

    fn endswith(&self, suffix: &str) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Bool(s.iter().map(|s| s.endswith(suffix)).collect()),
            _ => panic!("endswith() called on non-string"),
        }
    }

    fn startswith(&self, prefix: &str) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Bool(s.iter().map(|s| s.startswith(prefix)).collect()),
            _ => panic!("startswith() called on non-string"),
        }
    }

    fn find(&self, sub: &str) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Int(s.iter().map(|s| s.find(sub).unwrap_or(usize::MAX) as Idef).collect()),
            _ => panic!("find() called on non-string"),
        }
    }

    fn lower(&self) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.lower()).collect()),
            _ => panic!("lower() called on non-string"),
        }
    }

    fn upper(&self) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.upper()).collect()),
            _ => panic!("upper() called on non-string"),
        }
    }

    fn replace(&self, old: &str, new: &str) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.replace(old, new)).collect()),
            _ => panic!("replace() called on non-string"),
        }
    }

    fn split(&self, sep: &str) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.split(sep).collect()).collect()),
            _ => panic!("split() called on non-string"),
        }
    }

    fn strip(&self) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.strip()).collect()),
            _ => panic!("strip() called on non-string"),
        }
    }

    fn lstrip(&self) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.lstrip()).collect()),
            _ => panic!("lstrip() called on non-string"),
        }
    }

    fn rstrip(&self) -> Self {
        match self {
            RVecData::Str(s) => RVecData::Str(s.iter().map(|s| s.rstrip()).collect()),
            _ => panic!("rstrip() called on non-string"),
        }
    }
}