pub trait One {
    type Output;

    fn one() -> Self::Output;
}

impl One for f32 {
    type Output = f32;

    fn one() -> Self::Output {
        1.0_f32
    }
}

impl One for f64 {
    type Output = f64;

    fn one() -> Self::Output {
        1.0_f64
    }
}

impl One for i8 {
    type Output = i8;

    fn one() -> Self::Output {
        1
    }
}

impl One for u8 {
    type Output = u8;

    fn one() -> Self::Output {
        1
    }
}

impl One for i16 {
    type Output = i16;

    fn one() -> Self::Output {
        1
    }
}

impl One for u16 {
    type Output = u16;

    fn one() -> Self::Output {
        1
    }
}

impl One for i32 {
    type Output = i32;

    fn one() -> Self::Output {
        1
    }
}

impl One for u32 {
    type Output = u32;

    fn one() -> Self::Output {
        1
    }
}

impl One for i64 {
    type Output = i64;

    fn one() -> Self::Output {
        1
    }
}

impl One for u64 {
    type Output = u64;

    fn one() -> Self::Output {
        1
    }
}

impl One for i128 {
    type Output = i128;

    fn one() -> Self::Output {
        1
    }
}

impl One for u128 {
    type Output = u128;

    fn one() -> Self::Output {
        1
    }
}

impl One for isize {
    type Output = isize;

    fn one() -> Self::Output {
        1
    }
}

impl One for usize {
    type Output = usize;

    fn one() -> Self::Output {
        1
    }
}
