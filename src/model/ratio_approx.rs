use std::ops::{Add, Neg};
use num_rational::Ratio;
use num_traits::Inv;

pub fn ratio_approximate(value:f64) -> Ratio<i32> {
    if value < 0f64 {
        ratio_approximate(-value).neg()
    } else {
        let i = value.round();
        let r = do_ratio_approximate(value - i);
        Ratio::from_integer(i as i32).add(r)
    }
}

pub fn do_ratio_approximate(value:f64) -> Ratio<i32> {
    if value < 0f64 {
        return do_ratio_approximate(-value).neg();
    }
    let mut val = value;
    let mut coef = [0u32;1000];
    for i in 0..coef.len() {
        let e = val.floor() as u32;
        let r = val-(e as f64);
        coef[i] = e;

        let estimation = estimate(&coef,i+1);
        if (estimation-value).abs() < 1e-3 {
            return compute_ratio(&coef,i+1);
        }

        val = 1./r;
    };

    todo!()

}

fn estimate(coef:&[u32], length:usize) -> f64 {

    let mut r = coef[length-1] as f64;
    for i in (0..length-1).rev() {
        r = (coef[i] as f64) + 1f64/r;
    };

    r
}

fn compute_ratio(coef:&[u32], length:usize) -> Ratio<i32> {
    let mut r = Ratio::from_integer(coef[length-1] as i32);

    for i in (0..length-1).rev() {
        r = r.inv().add(coef[i] as i32);
    };

    r


}
