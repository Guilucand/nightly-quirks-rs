pub trait IntRoundings {
    fn nq_div_ceil(self, rhs: Self) -> Self;
}

macro_rules! int_roundings {
    ($int_type:ty) => {
        impl IntRoundings for $int_type {
            #[must_use = "this returns the result of the operation, \
            without modifying the original"]
            #[inline]
            fn nq_div_ceil(self, rhs: Self) -> Self {
                let d = self / rhs;
                let r = self % rhs;
                if r > 0 && rhs > 0 {
                    d + 1
                } else {
                    d
                }
            }
        }
    };
}

int_roundings!(u8);
int_roundings!(i8);
int_roundings!(u16);
int_roundings!(i16);
int_roundings!(u32);
int_roundings!(i32);
int_roundings!(u64);
int_roundings!(i64);
int_roundings!(u128);
int_roundings!(i128);
