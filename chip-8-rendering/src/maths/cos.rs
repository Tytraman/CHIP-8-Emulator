pub trait Cosinus {
    type Output;

    fn cosinus(&self) -> Self::Output;
}

impl Cosinus for f32 {
    type Output = f32;

    fn cosinus(&self) -> Self::Output {
        self.cos()
    }
}

impl Cosinus for f64 {
    type Output = f64;

    fn cosinus(&self) -> Self::Output {
        self.cos()
    }
}

impl Cosinus for i8 {
    type Output = i8;

    fn cosinus(&self) -> Self::Output {
        (*self as f32).cos() as Self::Output
    }
}

impl Cosinus for u8 {
    type Output = u8;

    fn cosinus(&self) -> Self::Output {
        (*self as f32).cos() as Self::Output
    }
}

impl Cosinus for i16 {
    type Output = i16;

    fn cosinus(&self) -> Self::Output {
        (*self as f32).cos() as Self::Output
    }
}

impl Cosinus for u16 {
    type Output = u16;

    fn cosinus(&self) -> Self::Output {
        (*self as f32).cos() as Self::Output
    }
}

impl Cosinus for i32 {
    type Output = i32;

    fn cosinus(&self) -> Self::Output {
        (*self as f64).cos() as Self::Output
    }
}

impl Cosinus for u32 {
    type Output = u32;

    fn cosinus(&self) -> Self::Output {
        (*self as f64).cos() as Self::Output
    }
}

impl Cosinus for i64 {
    type Output = i64;

    fn cosinus(&self) -> Self::Output {
        (*self as f64).cos() as Self::Output
    }
}

impl Cosinus for u64 {
    type Output = u64;

    fn cosinus(&self) -> Self::Output {
        (*self as f64).cos() as Self::Output
    }
}

impl Cosinus for i128 {
    type Output = i128;

    fn cosinus(&self) -> Self::Output {
        (*self as f64).cos() as Self::Output
    }
}

impl Cosinus for u128 {
    type Output = u128;

    fn cosinus(&self) -> Self::Output {
        (*self as f64).cos() as Self::Output
    }
}

impl Cosinus for isize {
    type Output = isize;

    fn cosinus(&self) -> Self::Output {
        (*self as f64).cos() as Self::Output
    }
}

impl Cosinus for usize {
    type Output = usize;

    fn cosinus(&self) -> Self::Output {
        (*self as f64).cos() as Self::Output
    }
}
