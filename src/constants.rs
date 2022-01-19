
const EPS:f64 = 1e-6;

pub fn is_nil(v:f64) -> bool {
    v.abs()<EPS
}

pub fn is_not_nil(v:f64) -> bool {
    v.abs()>=EPS
}