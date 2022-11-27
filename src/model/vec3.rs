use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::util::rtweekend::{random_double, random_double_by_range};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }

    pub fn random_by_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double_by_range(min, max),
            random_double_by_range(min, max),
            random_double_by_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_by_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                random_double_by_range(-1.0, 1.0),
                random_double_by_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;

        r_out_perp + r_out_parallel
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Index<i32> for Vec3 {
    type Output = f64;

    fn index(&self, index: i32) -> &Self::Output {
        if index >= 0 && index < 3 {
            return &self.e[index as usize];
        }

        panic!("Index out of range for Vec3")
    }
}

impl IndexMut<i32> for Vec3 {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        if index >= 0 && index < 3 {
            return &mut self.e[index as usize];
        }

        panic!("Index out of range for Vec3")
    }
}

macro_rules! gen_binary_ops {
    ($Vector:ident $op:ident $func:ident $symbol:tt) => {
        impl $op for &$Vector {
            type Output = $Vector;

            fn $func(self, other: &$Vector) -> $Vector {
                $Vector::new(self.e[0] $symbol other.e[0], self.e[1] $symbol other.e[1], self.e[2] $symbol other.e[2])
            }
        }

        impl $op<$Vector> for $Vector {
            type Output = $Vector;

            fn $func(self, other: $Vector) -> $Vector {
                &self $symbol &other
            }
        }

        impl $op<& $Vector> for $Vector {
            type Output = $Vector;

            fn $func(self, other: & $Vector) -> $Vector {
                &self $symbol other
            }
        }

        impl $op<$Vector> for & $Vector {
            type Output = $Vector;

            fn $func(self, other: $Vector) -> $Vector {
                self $symbol &other
            }
        }

        impl $op<f64> for & $Vector {
            type Output = $Vector;

            fn $func(self, other: f64) -> $Vector {
                $Vector::new(self.e[0] $symbol other, self.e[1] $symbol other, self.e[2] $symbol other)
            }
        }

        impl $op<f64> for $Vector {
            type Output = $Vector;

            fn $func(self, other: f64) -> $Vector {
                &self $symbol other
            }
        }

        impl $op<$Vector> for f64 {
            type Output = $Vector;

            fn $func(self, other: $Vector) -> $Vector {
                &other $symbol self
            }
        }

        impl $op<&$Vector> for f64 {
            type Output = $Vector;

            fn $func(self, other: &$Vector) -> $Vector {
                other $symbol self
            }
        }
    };
}

macro_rules! gen_unary_ops {
    ($Vector:ident $op:ident $func:ident $symbol:tt) => {
        impl $op for & $Vector {
            type Output = $Vector;

            fn $func(self) -> $Vector {
                $Vector::new($symbol self.e[0], $symbol self.e[1], $symbol self.e[2])
            }
        }

        impl $op for $Vector {
            type Output = $Vector;

            fn $func(self) -> $Vector {
                $symbol &self
            }
        }
    };
}

macro_rules! gen_op_assign {
    ($Vector:ident $op:ident $func:ident $symbol:tt) => {
        impl $op<& $Vector> for $Vector {
            fn $func(&mut self, other: & $Vector) {
                self.e[0] $symbol other.e[0];
                self.e[1] $symbol other.e[1];
                self.e[2] $symbol other.e[2];
            }
        }

        impl $op for $Vector {
            fn $func(&mut self, other: $Vector) {
                self.e[0] $symbol &other.e[0];
                self.e[1] $symbol &other.e[1];
                self.e[2] $symbol &other.e[2];
            }
        }

        impl $op<f64> for $Vector {
            fn $func(&mut self, other: f64) {
                self.e[0] $symbol other;
                self.e[1] $symbol other;
                self.e[2] $symbol other;
            }
        }
    };
}

gen_binary_ops!(Vec3 Add add +);
gen_binary_ops!(Vec3 Sub sub -);
gen_binary_ops!(Vec3 Mul mul *);
gen_binary_ops!(Vec3 Div div /);

gen_unary_ops!(Vec3 Neg neg -);

gen_op_assign!(Vec3 AddAssign add_assign +=);
gen_op_assign!(Vec3 SubAssign sub_assign -=);
gen_op_assign!(Vec3 MulAssign mul_assign *=);
gen_op_assign!(Vec3 DivAssign div_assign /=);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_vec3() {
        let a = Vec3::default();
        assert_eq!(0.0, a.x());
        assert_eq!(0.0, a.y());
        assert_eq!(0.0, a.z());
    }

    #[test]
    fn test_vec3() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, a.x());
        assert_eq!(2.0, a.y());
        assert_eq!(3.0, a.z());
        assert_eq!(14.0, a.length_squared());
        assert_eq!(14.0f64.sqrt(), a.length());
    }

    #[test]
    fn test_vec3_neg() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        a = -a;
        assert_eq!(-1.0, a.x());
        assert_eq!(-2.0, a.y());
        assert_eq!(-3.0, a.z());
    }

    #[test]
    fn test_vec3_addassign_vec3() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a += b;
        assert_eq!(5.0, a.x());
        assert_eq!(7.0, a.y());
        assert_eq!(9.0, a.z());

        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a += &b;
        assert_eq!(5.0, a.x());
        assert_eq!(7.0, a.y());
        assert_eq!(9.0, a.z());
    }

    #[test]
    fn test_vec3_subassign_vec3() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a -= b;
        assert_eq!(-3.0, a.x());
        assert_eq!(-3.0, a.y());
        assert_eq!(-3.0, a.z());

        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a -= &b;
        assert_eq!(-3.0, a.x());
        assert_eq!(-3.0, a.y());
        assert_eq!(-3.0, a.z());
    }

    #[test]
    fn test_vec3_multiplyassign_vec3() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a *= b;
        assert_eq!(4.0, a.x());
        assert_eq!(10.0, a.y());
        assert_eq!(18.0, a.z());

        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        a *= &b;
        assert_eq!(4.0, a.x());
        assert_eq!(10.0, a.y());
        assert_eq!(18.0, a.z());
    }

    #[test]
    fn test_vec3_divideassign_vec3() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let mut b = Vec3::new(4.0, 5.0, 6.0);
        b /= a;
        assert_eq!(4.0, b.x());
        assert_eq!(2.5, b.y());
        assert_eq!(2.0, b.z());

        let a = Vec3::new(1.0, 2.0, 3.0);
        let mut b = Vec3::new(4.0, 5.0, 6.0);
        b /= &a;
        assert_eq!(4.0, b.x());
        assert_eq!(2.5, b.y());
        assert_eq!(2.0, b.z());
    }

    #[test]
    fn test_vec3_multiplyassign_f64() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;
        a *= b;
        assert_eq!(2.0, a.x());
        assert_eq!(4.0, a.y());
        assert_eq!(6.0, a.z());
    }

    #[test]
    fn test_vec3_divideassign_f64() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;
        a /= b;
        assert_eq!(0.5, a.x());
        assert_eq!(1.0, a.y());
        assert_eq!(1.5, a.z());
    }
}
