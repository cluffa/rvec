use crate::{Idef, Fdef};

pub fn concat_float_int(a: &Vec<Fdef>, b: &Vec<Idef>) -> Vec<Fdef> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    result.extend(a.iter().cloned());
    result.extend(b.iter().map(|x| *x as Fdef));
    result
}

pub fn concat_float_float(a: &Vec<Fdef>, b: &Vec<Fdef>) -> Vec<Fdef> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    result.extend(a.iter().cloned());
    result.extend(b.iter().cloned());
    result
}

pub fn concat_int_int(a: &Vec<Idef>, b: &Vec<Idef>) -> Vec<Idef> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    result.extend(a.iter().cloned());
    result.extend(b.iter().cloned());
    result
}

pub fn concat_str_str(a: &Vec<String>, b: &Vec<String>) -> Vec<String> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    result.extend(a.iter().cloned());
    result.extend(b.iter().cloned());
    result
}

pub fn concat_bool_bool(a: &Vec<bool>, b: &Vec<bool>) -> Vec<bool> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    result.extend(a.iter().cloned());
    result.extend(b.iter().cloned());
    result
}
