use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, SubAssign};
use num_rational::Ratio;
use num_traits::ToPrimitive;

const EPS:f64 = 1e-6_f64;

pub trait Amount : Display {
    fn is_nil(&self) -> bool;
    fn multiply(&self,factor:u32) -> Self;
    fn per_minute(&self, duration:u32) -> Self;
}

#[derive(Copy, Clone,Default)]
pub struct AmountF64 {
    value:f64,
}

impl AddAssign for AmountF64 {
    fn add_assign(&mut self, rhs: Self) {
        *self = AmountF64{value:self.value+rhs.value};
    }
}

impl SubAssign for AmountF64 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = AmountF64{value:self.value-rhs.value};
    }
}

impl Amount for AmountF64 {
    fn is_nil(&self) -> bool {
        self.value.abs() < EPS
    }

    fn multiply(&self, factor: u32) -> AmountF64 {
        let value = self.value * (factor as f64);
        AmountF64{value}
    }

    fn per_minute(&self, duration: u32) -> Self {
        let value = self.value * (duration as f64) / 60_f64;
        AmountF64{value}
    }
}
impl Display for AmountF64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:.3}",self.value))
    }
}


impl From<u32> for AmountF64 {
    fn from(value: u32) -> Self {
        AmountF64{value:value as f64}
    }
}

impl From<f64> for AmountF64 {
    fn from(value: f64) -> Self {
        AmountF64{value}
    }
}



#[derive(Copy,Clone)]
pub struct AmountRatio {
    value:Ratio<i32>,
}

impl Default for AmountRatio {
    fn default() -> Self {
        AmountRatio{value:0i32.into()}
    }
}



impl Amount for AmountRatio {
    fn is_nil(&self) -> bool {
        self.value.to_f64().unwrap().abs() < EPS
    }

    fn multiply(&self, factor: u32) -> AmountRatio {
        let value = self.value * (factor as i32);
        AmountRatio{value}
    }

    fn per_minute(&self, duration: u32) -> Self {
        let value = (self.value/60)*(duration as i32);
        AmountRatio{value}
    }
}

impl Display for AmountRatio {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.value.is_integer() {
            f.write_fmt(format_args!("{}",self.value.numer()))
        } else {
            f.write_fmt(format_args!("{}/{}", self.value.numer(), self.value.denom()))
        }
    }
}

impl From<AmountF64> for AmountRatio {
    fn from(amount_f64: AmountF64) -> Self {
        Ratio::approximate_float(amount_f64.value).map(|value| AmountRatio{value}).unwrap()
    }
}

impl AddAssign for AmountRatio {
    fn add_assign(&mut self, rhs: Self) {
        *self = AmountRatio{value:self.value+rhs.value};
    }
}

impl SubAssign for AmountRatio {
    fn sub_assign(&mut self, rhs: Self) {
        *self = AmountRatio{value:self.value-rhs.value};
    }
}
