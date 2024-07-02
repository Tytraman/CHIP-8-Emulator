pub trait Sqrt {
    type Type;

    fn sqrt(value: Self) -> Self::Type;
}

impl Sqrt for f32 {
    type Type = f32;

    fn sqrt(value: Self) -> Self::Type {
        value.sqrt()
    }
}

impl Sqrt for f64 {
    type Type = f64;

    fn sqrt(value: Self) -> Self::Type {
        value.sqrt()
    }
}

impl Sqrt for i8 {
    type Type = i8;

    fn sqrt(value: Self) -> Self::Type {
        (value as f32).sqrt() as Self::Type
    }
}

impl Sqrt for u8 {
    type Type = u8;

    fn sqrt(value: Self) -> Self::Type {
        (value as f32).sqrt() as Self::Type
    }
}

impl Sqrt for i16 {
    type Type = i16;

    fn sqrt(value: Self) -> Self::Type {
        (value as f32).sqrt() as Self::Type
    }
}

impl Sqrt for u16 {
    type Type = u16;

    fn sqrt(value: Self) -> Self::Type {
        (value as f32).sqrt() as Self::Type
    }
}

impl Sqrt for i32 {
    type Type = i32;

    fn sqrt(value: Self) -> Self::Type {
        (value as f64).sqrt() as Self::Type
    }
}

impl Sqrt for u32 {
    type Type = u32;

    fn sqrt(value: Self) -> Self::Type {
        (value as f64).sqrt() as Self::Type
    }
}

impl Sqrt for i64 {
    type Type = i64;

    fn sqrt(value: Self) -> Self::Type {
        (value as f64).sqrt() as Self::Type
    }
}

impl Sqrt for u64 {
    type Type = u64;

    fn sqrt(value: Self) -> Self::Type {
        (value as f64).sqrt() as Self::Type
    }
}

impl Sqrt for i128 {
    type Type = i128;

    fn sqrt(value: Self) -> Self::Type {
        (value as f64).sqrt() as Self::Type
    }
}

impl Sqrt for u128 {
    type Type = u128;

    fn sqrt(value: Self) -> Self::Type {
        (value as f64).sqrt() as Self::Type
    }
}

impl Sqrt for isize {
    type Type = isize;

    fn sqrt(value: Self) -> Self::Type {
        (value as f64).sqrt() as Self::Type
    }
}

impl Sqrt for usize {
    type Type = usize;

    fn sqrt(value: Self) -> Self::Type {
        (value as f64).sqrt() as Self::Type
    }
}
