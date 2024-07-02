pub trait Zero {
    type Output;

    fn zero() -> Self::Output;
}

impl Zero for f32 {
    type Output = f32;

    fn zero() -> Self::Output {
        0.0_f32
    }
}

impl Zero for f64 {
    type Output = f64;

    fn zero() -> Self::Output {
        0.0_f64
    }
}

impl Zero for i8 {
    type Output = i8;

    fn zero() -> Self::Output {
        0
    }
}

impl Zero for u8 {
    type Output = u8;

    fn zero() -> Self::Output {
        0
    }
}

impl Zero for i16 {
    type Output = i16;

    fn zero() -> Self::Output {
        0
    }
}

impl Zero for u16 {
    type Output = u16;

    fn zero() -> Self::Output {
        0
    }
}

impl Zero for i32 {
    type Output = i32;

    fn zero() -> Self::Output {
        0
    }
}

impl Zero for u32 {
    type Output = u32;

    fn zero() -> Self::Output {
        0
    }
}

impl Zero for i64 {
    type Output = i64;

    fn zero() -> Self::Output {
        0
    }
}

impl Zero for u64 {
    type Output = u64;

    fn zero() -> Self::Output {
        0
    }
}

impl Zero for i128 {
    type Output = i128;

    fn zero() -> Self::Output {
        0
    }
}

impl Zero for u128 {
    type Output = u128;

    fn zero() -> Self::Output {
        0
    }
}

impl Zero for isize {
    type Output = isize;

    fn zero() -> Self::Output {
        0
    }
}

impl Zero for usize {
    type Output = usize;

    fn zero() -> Self::Output {
        0
    }
}
