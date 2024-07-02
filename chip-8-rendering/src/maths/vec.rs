use std::ops::{self, Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::{
    mat::{Mat4, Mat4Index},
    sqrt::Sqrt,
};

pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

// Surcharge l'opérateur '+'.
impl<T> ops::Add<T> for Vec2<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

// Surcharge l'opérateur '+='.
impl<T> ops::AddAssign<T> for Vec2<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
    }
}

// Surcharge l'opérateur '-'.
impl<T> ops::Sub<T> for Vec2<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

// Surcharge l'opérateur '-='.
impl<T> ops::SubAssign<T> for Vec2<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

#[derive(Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// Surcharge l'opérateur '+'.
impl<T> ops::Add<T> for Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<T> ops::Add<&T> for Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn add(self, rhs: &T) -> Self::Output {
        Vec3 {
            x: self.x + *rhs,
            y: self.y + *rhs,
            z: self.z + *rhs,
        }
    }
}

impl<T> ops::Add<T> for &Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<T> ops::Add<&T> for &Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn add(self, rhs: &T) -> Self::Output {
        Vec3 {
            x: self.x + *rhs,
            y: self.y + *rhs,
            z: self.z + *rhs,
        }
    }
}

impl<T> ops::Add<Vec3<T>> for Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> ops::Add<Vec3<T>> for &Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> ops::Add<&Vec3<T>> for Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn add(self, rhs: &Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> ops::Add<&Vec3<T>> for &Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn add(self, rhs: &Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// Surcharge l'opérateur '+='.
impl<T> ops::AddAssign<T> for Vec3<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl<T> ops::AddAssign<&T> for Vec3<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: &T) {
        self.x += *rhs;
        self.y += *rhs;
        self.z += *rhs;
    }
}

impl<T> ops::AddAssign<Vec3<T>> for Vec3<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> ops::AddAssign<&Vec3<T>> for Vec3<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: &Vec3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// Surcharge l'opérateur '-'.
impl<T> ops::Sub<T> for Vec3<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<T> ops::Sub<&T> for Vec3<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: &T) -> Self::Output {
        Vec3 {
            x: self.x - *rhs,
            y: self.y - *rhs,
            z: self.z - *rhs,
        }
    }
}

impl<T> ops::Sub<T> for &Vec3<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<T> ops::Sub<&T> for &Vec3<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: &T) -> Self::Output {
        Vec3 {
            x: self.x - *rhs,
            y: self.y - *rhs,
            z: self.z - *rhs,
        }
    }
}

// Surcharge l'opérateur '-='.
impl<T> ops::SubAssign<T> for Vec3<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl<T> ops::SubAssign<&T> for Vec3<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: &T) {
        self.x -= *rhs;
        self.y -= *rhs;
        self.z -= *rhs;
    }
}

impl<T> ops::SubAssign<Vec3<T>> for Vec3<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> ops::SubAssign<&Vec3<T>> for Vec3<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: &Vec3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// Surcharge l'opérateur '*'.
impl<T> ops::Mul<T> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> ops::Mul<&T> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: &T) -> Self::Output {
        Vec3 {
            x: self.x * *rhs,
            y: self.y * *rhs,
            z: self.z * *rhs,
        }
    }
}

impl<T> ops::Mul<T> for &Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> ops::Mul<&T> for &Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: &T) -> Self::Output {
        Vec3 {
            x: self.x * *rhs,
            y: self.y * *rhs,
            z: self.z * *rhs,
        }
    }
}

impl<T> ops::Mul<Vec3<T>> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T> ops::Mul<&Vec3<T>> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: &Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T> ops::Mul<Vec3<T>> for &Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T> ops::Mul<&Vec3<T>> for &Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: &Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

// Surcharge l'opérateur '*='.
impl<T> ops::MulAssign<T> for Vec3<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T> ops::MulAssign<&T> for Vec3<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: &T) {
        self.x *= *rhs;
        self.y *= *rhs;
        self.z *= *rhs;
    }
}

impl<T> ops::MulAssign<Vec3<T>> for Vec3<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: Vec3<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T> ops::MulAssign<&Vec3<T>> for Vec3<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: &Vec3<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

// Surcharge l'opérateur '/'.
impl<T> ops::Div<T> for Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> ops::Div<&T> for Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn div(self, rhs: &T) -> Self::Output {
        Vec3 {
            x: self.x / *rhs,
            y: self.y / *rhs,
            z: self.z / *rhs,
        }
    }
}

impl<T> ops::Div<T> for &Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> ops::Div<&T> for &Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn div(self, rhs: &T) -> Self::Output {
        Vec3 {
            x: self.x / *rhs,
            y: self.y / *rhs,
            z: self.z / *rhs,
        }
    }
}

impl<T> ops::Div<Vec3<T>> for Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn div(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T> ops::Div<&Vec3<T>> for Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn div(self, rhs: &Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T> ops::Div<Vec3<T>> for &Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn div(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T> ops::Div<&Vec3<T>> for &Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn div(self, rhs: &Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T> Vec3<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Sqrt<Type = T>
        + Copy,
{
    pub fn scale(v: &Self, scalar: T) -> Self {
        v * scalar
    }

    pub fn magn(v: &Self) -> T {
        T::sqrt(v.x * v.x + v.y * v.y + v.z * v.z)
    }

    pub fn normalize(v: &Self) -> Self {
        let length = Self::magn(&v);

        Self {
            x: v.x / length,
            y: v.y / length,
            z: v.z / length,
        }
    }

    pub fn cross(v1: &Self, v2: &Self) -> Self {
        Self {
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z,
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }

    pub fn dot(v1: &Self, v2: &Self) -> T {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }
}

pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

// Surcharge l'opérateur '+'.
impl<T> ops::Add<T> for Vec4<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
            w: self.w + rhs,
        }
    }
}

impl<T> ops::Add<Vec4<T>> for Vec4<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Vec4<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

// Surcharge l'opérateur '+='.
impl<T> ops::AddAssign<T> for Vec4<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self.w += rhs;
    }
}

impl<T> ops::AddAssign<Vec4<T>> for Vec4<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Vec4<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

// Surcharge l'opérateur '-'.
impl<T> ops::Sub<T> for Vec4<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
            w: self.w - rhs,
        }
    }
}

impl<T> ops::Sub<Vec4<T>> for Vec4<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Vec4<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

// Surcharge l'opérateur '-='.
impl<T> ops::SubAssign<T> for Vec4<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self.w -= rhs;
    }
}

impl<T> ops::SubAssign<Vec4<T>> for Vec4<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Vec4<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

// Surcharge l'opérateur '*'.
impl<T> ops::Mul<T> for Vec4<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<T> ops::Mul<T> for &Vec4<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<T> ops::Mul<Vec4<T>> for Vec4<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl<T> ops::Mul<&Vec4<T>> for Vec4<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: &Vec4<T>) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl<T> ops::Mul<Mat4<T>> for Vec4<T>
where
    T: ops::Mul<Output = T> + ops::Add<Output = T> + Copy,
{
    type Output = Vec4<T>;

    fn mul(self, rhs: Mat4<T>) -> Self::Output {
        Vec4 {
            x: (self.x * rhs[Mat4Index::X1])
                + (self.y * rhs[Mat4Index::Y1])
                + (self.z * rhs[Mat4Index::Z1])
                + (self.w * rhs[Mat4Index::W1]),
            y: (self.x * rhs[Mat4Index::X2])
                + (self.y * rhs[Mat4Index::Y2])
                + (self.z * rhs[Mat4Index::Z2])
                + (self.w * rhs[Mat4Index::W2]),
            z: (self.x * rhs[Mat4Index::X3])
                + (self.y * rhs[Mat4Index::Y3])
                + (self.z * rhs[Mat4Index::Z3])
                + (self.w * rhs[Mat4Index::W3]),
            w: (self.x * rhs[Mat4Index::X4])
                + (self.y * rhs[Mat4Index::Y4])
                + (self.z * rhs[Mat4Index::Z4])
                + (self.w * rhs[Mat4Index::W4]),
        }
    }
}

// Surcharge l'opérateur '*='.
impl<T> ops::MulAssign<T> for Vec4<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl<T> ops::MulAssign<Vec4<T>> for Vec4<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: Vec4<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

// Surcharge l'opérateur '/'.
impl<T> ops::Div<T> for Vec4<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl<T> ops::Div<Vec4<T>> for Vec4<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: Vec4<T>) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}

// Surcharge l'opérateur '/='.
impl<T> ops::DivAssign<T> for Vec4<T>
where
    T: DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl<T> ops::DivAssign<Vec4<T>> for Vec4<T>
where
    T: DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: Vec4<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

impl<T> Vec4<T>
where
    T: Mul<Output = T> + Copy,
{
    pub fn scale(v: &Self, scalar: T) -> Self {
        v * scalar
    }
}
