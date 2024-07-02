use std::ops::{self};

use super::{
    cos::Cosinus,
    one::One,
    rad::Radians,
    sin::Sinus,
    vec::{Vec3, Vec4},
    zero::Zero,
};

pub enum Mat4Index {
    X1,
    X2,
    X3,
    X4,
    Y1,
    Y2,
    Y3,
    Y4,
    Z1,
    Z2,
    Z3,
    Z4,
    W1,
    W2,
    W3,
    W4,
}

impl Mat4Index {
    fn value(&self) -> usize {
        match *self {
            Mat4Index::X1 => 0,
            Mat4Index::Y1 => 1,
            Mat4Index::Z1 => 2,
            Mat4Index::W1 => 3,
            Mat4Index::X2 => 4,
            Mat4Index::Y2 => 5,
            Mat4Index::Z2 => 6,
            Mat4Index::W2 => 7,
            Mat4Index::X3 => 8,
            Mat4Index::Y3 => 9,
            Mat4Index::Z3 => 10,
            Mat4Index::W3 => 11,
            Mat4Index::X4 => 12,
            Mat4Index::Y4 => 13,
            Mat4Index::Z4 => 14,
            Mat4Index::W4 => 15,
        }
    }
}

pub struct Mat4<T> {
    data: [T; 4 * 4],
}

impl<T> ops::Index<Mat4Index> for Mat4<T> {
    type Output = T;

    fn index(&self, index: Mat4Index) -> &Self::Output {
        &self.data[index.value()]
    }
}

impl<T> ops::Mul<Vec4<T>> for Mat4<T>
where
    T: ops::Mul<Output = T> + ops::Add<Output = T> + Copy,
{
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        rhs * self
    }
}

impl<T> Mat4<T>
where
    T: ops::Add<Output = T>
        + ops::Mul<Output = T>
        + ops::Neg<Output = T>
        + Zero<Output = T>
        + One<Output = T>
        + Radians<Output = T>
        + Cosinus<Output = T>
        + Sinus<Output = T>
        + Copy,
{
    pub fn new() -> Self {
        Self {
            data: [
                T::one(),
                T::zero(),
                T::zero(),
                T::zero(),
                T::zero(),
                T::one(),
                T::zero(),
                T::zero(),
                T::zero(),
                T::zero(),
                T::one(),
                T::zero(),
                T::zero(),
                T::zero(),
                T::zero(),
                T::one(),
            ],
        }
    }

    pub fn mul(m1: &Self, m2: &Self) -> Self {
        let data: [T; 4 * 4] = [
            (m1[Mat4Index::X1] * m2[Mat4Index::X1]
                + m1[Mat4Index::Y1] * m2[Mat4Index::X2]
                + m1[Mat4Index::Z1] * m2[Mat4Index::X3]
                + m1[Mat4Index::W1] * m2[Mat4Index::X4]),
            (m1[Mat4Index::X1] * m2[Mat4Index::Y1]
                + m1[Mat4Index::Y1] * m2[Mat4Index::Y2]
                + m1[Mat4Index::Z1] * m2[Mat4Index::Y3]
                + m1[Mat4Index::W1] * m2[Mat4Index::Y4]),
            (m1[Mat4Index::X1] * m2[Mat4Index::Z1]
                + m1[Mat4Index::Y1] * m2[Mat4Index::Z2]
                + m1[Mat4Index::Z1] * m2[Mat4Index::Z3]
                + m1[Mat4Index::W1] * m2[Mat4Index::Z4]),
            (m1[Mat4Index::X1] * m2[Mat4Index::W1]
                + m1[Mat4Index::Y1] * m2[Mat4Index::W2]
                + m1[Mat4Index::Z1] * m2[Mat4Index::W3]
                + m1[Mat4Index::W1] * m2[Mat4Index::W4]),
            (m1[Mat4Index::X2] * m2[Mat4Index::X1]
                + m1[Mat4Index::Y2] * m2[Mat4Index::X2]
                + m1[Mat4Index::Z2] * m2[Mat4Index::X3]
                + m1[Mat4Index::W2] * m2[Mat4Index::X4]),
            (m1[Mat4Index::X2] * m2[Mat4Index::Y1]
                + m1[Mat4Index::Y2] * m2[Mat4Index::Y2]
                + m1[Mat4Index::Z2] * m2[Mat4Index::Y3]
                + m1[Mat4Index::W2] * m2[Mat4Index::Y4]),
            (m1[Mat4Index::X2] * m2[Mat4Index::Z1]
                + m1[Mat4Index::Y2] * m2[Mat4Index::Z2]
                + m1[Mat4Index::Z2] * m2[Mat4Index::Z3]
                + m1[Mat4Index::W2] * m2[Mat4Index::Z4]),
            (m1[Mat4Index::X2] * m2[Mat4Index::W1]
                + m1[Mat4Index::Y2] * m2[Mat4Index::W2]
                + m1[Mat4Index::Z2] * m2[Mat4Index::W3]
                + m1[Mat4Index::W2] * m2[Mat4Index::W4]),
            (m1[Mat4Index::X3] * m2[Mat4Index::X1]
                + m1[Mat4Index::Y3] * m2[Mat4Index::X2]
                + m1[Mat4Index::Z3] * m2[Mat4Index::X3]
                + m1[Mat4Index::W3] * m2[Mat4Index::X4]),
            (m1[Mat4Index::X3] * m2[Mat4Index::Y1]
                + m1[Mat4Index::Y3] * m2[Mat4Index::Y2]
                + m1[Mat4Index::Z3] * m2[Mat4Index::Y3]
                + m1[Mat4Index::W3] * m2[Mat4Index::Y4]),
            (m1[Mat4Index::X3] * m2[Mat4Index::Z1]
                + m1[Mat4Index::Y3] * m2[Mat4Index::Z2]
                + m1[Mat4Index::Z3] * m2[Mat4Index::Z3]
                + m1[Mat4Index::W3] * m2[Mat4Index::Z4]),
            (m1[Mat4Index::X3] * m2[Mat4Index::W1]
                + m1[Mat4Index::Y3] * m2[Mat4Index::W2]
                + m1[Mat4Index::Z3] * m2[Mat4Index::W3]
                + m1[Mat4Index::W3] * m2[Mat4Index::W4]),
            (m1[Mat4Index::X4] * m2[Mat4Index::X1]
                + m1[Mat4Index::Y4] * m2[Mat4Index::X2]
                + m1[Mat4Index::Z4] * m2[Mat4Index::X3]
                + m1[Mat4Index::W4] * m2[Mat4Index::X4]),
            (m1[Mat4Index::X4] * m2[Mat4Index::Y1]
                + m1[Mat4Index::Y4] * m2[Mat4Index::Y2]
                + m1[Mat4Index::Z4] * m2[Mat4Index::Y3]
                + m1[Mat4Index::W4] * m2[Mat4Index::Y4]),
            (m1[Mat4Index::X4] * m2[Mat4Index::Z1]
                + m1[Mat4Index::Y4] * m2[Mat4Index::Z2]
                + m1[Mat4Index::Z4] * m2[Mat4Index::Z3]
                + m1[Mat4Index::W4] * m2[Mat4Index::Z4]),
            (m1[Mat4Index::X4] * m2[Mat4Index::W1]
                + m1[Mat4Index::Y4] * m2[Mat4Index::W2]
                + m1[Mat4Index::Z4] * m2[Mat4Index::W3]
                + m1[Mat4Index::W4] * m2[Mat4Index::W4]),
        ];

        Self { data }
    }

    pub fn translate(m: &Self, v: &Vec3<T>) -> Self {
        let data: [T; 4 * 4] = [
            T::one(),
            T::zero(),
            T::zero(),
            v.x,
            T::zero(),
            T::one(),
            T::zero(),
            v.y,
            T::zero(),
            T::zero(),
            T::one(),
            v.z,
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
        ];

        Self::mul(&m, &Self { data })
    }

    pub fn scale(m: &Self, v: &Vec3<T>) -> Self {
        let data: [T; 4 * 4] = [
            v.x,
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            v.y,
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            v.z,
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
        ];

        Self::mul(&m, &Self { data })
    }

    pub fn rotate_x(m: &Self, degrees: &T) -> Self {
        let rad = degrees.to_rad();

        let data: [T; 4 * 4] = [
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            rad.cosinus(),
            -rad.sinus(),
            T::zero(),
            T::zero(),
            rad.sinus(),
            rad.cosinus(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
        ];

        Self::mul(&m, &Self { data })
    }

    pub fn rotate_y(m: &Self, degrees: &T) -> Self {
        let rad = degrees.to_rad();

        let data: [T; 4 * 4] = [
            rad.cosinus(),
            T::zero(),
            rad.sinus(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
            -rad.sinus(),
            T::zero(),
            rad.cosinus(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
        ];

        Self::mul(&m, &Self { data })
    }

    pub fn rotate_z(m: &Self, degrees: &T) -> Self {
        let rad = degrees.to_rad();

        let data: [T; 4 * 4] = [
            rad.cosinus(),
            -rad.sinus(),
            T::zero(),
            T::zero(),
            rad.sinus(),
            rad.cosinus(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
        ];

        Self::mul(&m, &Self { data })
    }

    pub fn borrow_data(&self) -> &[T; 4 * 4] {
        &self.data
    }
}
