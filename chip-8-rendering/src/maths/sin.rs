pub trait Sinus {
    type Output;

    fn sinus(&self) -> Self::Output;
}

impl Sinus for f32 {
    type Output = f32;

    fn sinus(&self) -> Self::Output {
        self.sin()
    }
}

impl Sinus for f64 {
    type Output = f64;

    fn sinus(&self) -> Self::Output {
        self.sin()
    }
}

impl Sinus for i8 {
    type Output = i8;

    fn sinus(&self) -> Self::Output {
        (*self as f32).sin() as Self::Output
    }
}

impl Sinus for u8 {
    type Output = u8;

    fn sinus(&self) -> Self::Output {
        (*self as f32).sin() as Self::Output
    }
}

impl Sinus for i16 {
    type Output = i16;

    fn sinus(&self) -> Self::Output {
        (*self as f32).sin() as Self::Output
    }
}

impl Sinus for u16 {
    type Output = u16;

    fn sinus(&self) -> Self::Output {
        (*self as f32).sin() as Self::Output
    }
}

impl Sinus for i32 {
    type Output = i32;

    fn sinus(&self) -> Self::Output {
        (*self as f64).sin() as Self::Output
    }
}

impl Sinus for u32 {
    type Output = u32;

    fn sinus(&self) -> Self::Output {
        (*self as f64).sin() as Self::Output
    }
}

impl Sinus for i64 {
    type Output = i64;

    fn sinus(&self) -> Self::Output {
        (*self as f64).sin() as Self::Output
    }
}

impl Sinus for u64 {
    type Output = u64;

    fn sinus(&self) -> Self::Output {
        (*self as f64).sin() as Self::Output
    }
}

impl Sinus for i128 {
    type Output = i128;

    fn sinus(&self) -> Self::Output {
        (*self as f64).sin() as Self::Output
    }
}

impl Sinus for u128 {
    type Output = u128;

    fn sinus(&self) -> Self::Output {
        (*self as f64).sin() as Self::Output
    }
}

impl Sinus for isize {
    type Output = isize;

    fn sinus(&self) -> Self::Output {
        (*self as f64).sin() as Self::Output
    }
}

impl Sinus for usize {
    type Output = usize;

    fn sinus(&self) -> Self::Output {
        (*self as f64).sin() as Self::Output
    }
}
