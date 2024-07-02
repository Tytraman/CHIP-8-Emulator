pub trait Radians {
    type Output;

    fn to_rad(&self) -> Self::Output;
}

impl Radians for f32 {
    type Output = f32;

    fn to_rad(&self) -> Self::Output {
        self.to_radians()
    }
}

impl Radians for f64 {
    type Output = f64;

    fn to_rad(&self) -> Self::Output {
        self.to_radians()
    }
}

impl Radians for i8 {
    type Output = i8;

    fn to_rad(&self) -> Self::Output {
        (*self as f32).to_radians() as Self::Output
    }
}

impl Radians for u8 {
    type Output = u8;

    fn to_rad(&self) -> Self::Output {
        (*self as f32).to_radians() as Self::Output
    }
}

impl Radians for i16 {
    type Output = i16;

    fn to_rad(&self) -> Self::Output {
        (*self as f32).to_radians() as Self::Output
    }
}

impl Radians for u16 {
    type Output = u16;

    fn to_rad(&self) -> Self::Output {
        (*self as f32).to_radians() as Self::Output
    }
}

impl Radians for i32 {
    type Output = i32;

    fn to_rad(&self) -> Self::Output {
        (*self as f64).to_radians() as Self::Output
    }
}

impl Radians for u32 {
    type Output = u32;

    fn to_rad(&self) -> Self::Output {
        (*self as f64).to_radians() as Self::Output
    }
}

impl Radians for i64 {
    type Output = i64;

    fn to_rad(&self) -> Self::Output {
        (*self as f64).to_radians() as Self::Output
    }
}

impl Radians for u64 {
    type Output = u64;

    fn to_rad(&self) -> Self::Output {
        (*self as f64).to_radians() as Self::Output
    }
}

impl Radians for i128 {
    type Output = i128;

    fn to_rad(&self) -> Self::Output {
        (*self as f64).to_radians() as Self::Output
    }
}

impl Radians for u128 {
    type Output = u128;

    fn to_rad(&self) -> Self::Output {
        (*self as f64).to_radians() as Self::Output
    }
}

impl Radians for isize {
    type Output = isize;

    fn to_rad(&self) -> Self::Output {
        (*self as f64).to_radians() as Self::Output
    }
}

impl Radians for usize {
    type Output = usize;

    fn to_rad(&self) -> Self::Output {
        (*self as f64).to_radians() as Self::Output
    }
}
