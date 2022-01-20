
const EPS:f64 = 1e-3;

pub fn is_nil(v:f64) -> bool {
    v.abs()<EPS
}

