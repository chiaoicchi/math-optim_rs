//! Galois Field
//!
//! This structure represents **Galois Field**.
//!
//! Galois field is a field with finite number of elements.
//! Every galois field of prime order is isomorphic to `ℤ/pℤ` which `p` is prime.
//!

/// A structure representating a *galois field*.
///
/// Represents integers modulo a prime number `MOD`, forming the finite field `ℤ/MOD ℤ`.
///
/// # Type Parameters
///
/// - `MOD`: The prime modulus
///   (satisfy `MOD` is prime)
///
/// # Requirements
///
/// **`MOD` must be prime.** Non-prime moduli do not form a field.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GF<const MOD: u32> {
    /// The representative of the element in `ℤ/MOD ℤ` (satisfy `0 <= value < MOD`)
    value: u32,
}

impl<const MOD: u32> GF<MOD> {
    /// Returns the modulus.
    #[inline]
    pub const fn modulus() -> u32 {
        MOD
    }

    /// Returns zero (additive identity)
    #[inline]
    pub const fn zero() -> Self {
        Self { value: 0 }
    }

    /// Returns one (multiplicative identity)
    #[inline]
    pub const fn one() -> Self {
        Self { value: 1 }
    }

    /// Creates a new element of `ℤ/MOD ℤ`.
    ///
    /// ## Parameters
    ///
    /// - `value`: One of element in `ℤ`.
    ///
    /// ## Returns
    ///
    /// `value % MOD` becomes representative.
    ///
    /// ## Complexity
    ///
    /// `O(1)`
    #[inline]
    pub fn new(value: u32) -> Self {
        Self { value: value % MOD }
    }

    /// Returns `self^n` which is an element of `ℤ/MOD ℤ`.
    ///
    /// ## Parameters
    ///
    /// - `n`: The exponent.
    ///
    /// ## Returns
    ///
    /// `self^n` in `ℤ/MOD ℤ`
    ///
    /// ## Complexity
    ///
    /// `O(log max(n, MOD))`
    #[inline]
    pub fn pow(&self, mut n: u32) -> Self {
        // Reduce exponent by Fermat's Little Theorem: a^(MOD-1) ≡ 1 (mod MOD).
        if n >= MOD - 1 {
            n %= MOD - 1;
        }
        let mut res = Self::new(1);
        let mut base = *self;
        while n > 0 {
            if n & 1 == 1 {
                res *= base;
            }
            base *= base;
            n >>= 1;
        }
        res
    }

    /// Returns `self^{-1}` which is the inverse of `self` in `ℤ/MOD ℤ`.
    ///
    /// ## Returns
    ///
    /// `self^{-1}` in `ℤ/MOD ℤ`.
    ///
    /// ## Panics
    ///
    /// Panics if `self == 0` (zero has no multiplicative inverse)
    ///
    /// ## Complexity
    ///
    /// `O(log MOD)`
    #[inline]
    pub fn inv(&self) -> Self {
        debug_assert_ne!(
            self.value, 0,
            "The value which you want to calculate inverse must not be 0"
        );
        // Uses Fermat's Little Theorem: a^(MOD-1) ≡ 1 (mod MOD),
        // a^(MOD-2) ≡ a^(-1) is correct.
        self.pow(MOD - 2)
    }
}

use std::fmt::{Debug, Display, Formatter, Result};
impl<const MOD: u32> Debug for GF<MOD> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}
impl<const MOD: u32> Display for GF<MOD> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
impl<const MOD: u32> Neg for GF<MOD> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        if self.value > 0 {
            self.value = MOD - self.value;
        }
        self
    }
}
impl<const MOD: u32> AddAssign<GF<MOD>> for GF<MOD> {
    fn add_assign(&mut self, rhs: GF<MOD>) {
        self.value += rhs.value;
        if self.value >= MOD {
            self.value -= MOD;
        }
    }
}
impl<const MOD: u32> SubAssign<GF<MOD>> for GF<MOD> {
    fn sub_assign(&mut self, rhs: GF<MOD>) {
        if self.value < rhs.value {
            self.value += MOD;
        }
        self.value -= rhs.value;
    }
}
impl<const MOD: u32> MulAssign<GF<MOD>> for GF<MOD> {
    fn mul_assign(&mut self, rhs: GF<MOD>) {
        self.value = ((self.value as u64 * rhs.value as u64) % MOD as u64) as u32;
    }
}
impl<const MOD: u32> DivAssign<GF<MOD>> for GF<MOD> {
    fn div_assign(&mut self, rhs: GF<MOD>) {
        self.value = ((self.value as u64 * rhs.inv().value as u64) % MOD as u64) as u32;
    }
}

macro_rules! gf_ops {
    ($(
            $trait:ident,
            $trait_assign:ident,
            $fn:ident,
            $fn_assign:ident,
    )*) => {$(
        impl<const MOD: u32> $trait_assign<&GF<MOD>> for GF<MOD> {
            fn $fn_assign(&mut self, rhs: &GF<MOD>) {
                self.$fn_assign(*rhs);
            }
        }
        impl<const MOD: u32, T: Into<GF<MOD>>> $trait<T> for GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(mut self, rhs: T) -> Self::Output {
                self.$fn_assign(rhs.into());
                self
            }
        }
        impl<const MOD: u32> $trait<&GF<MOD>> for GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(self, rhs: &GF<MOD>) -> Self::Output {
                self.$fn(*rhs)
            }
        }
        impl<const MOD: u32, T: Into<GF<MOD>>> $trait<T> for &GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(self, rhs: T) -> Self::Output {
                (*self).$fn(rhs.into())
            }
        }
        impl<const MOD: u32> $trait<&GF<MOD>> for &GF<MOD> {
            type Output = GF<MOD>;
            fn $fn(self, rhs: &GF<MOD>) -> Self::Output {
                (*self).$fn(*rhs)
            }
        }
    )*};
}

gf_ops! {
    Add, AddAssign, add, add_assign,
    Sub, SubAssign, sub, sub_assign,
    Mul, MulAssign, mul, mul_assign,
    Div, DivAssign, div, div_assign,
}

use std::iter::{Product, Sum};
impl<const MOD: u32> Sum for GF<MOD> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(0), |acc, a| acc + a)
    }
}
impl<'a, const MOD: u32> Sum<&'a Self> for GF<MOD> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}
impl<const MOD: u32> Product for GF<MOD> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new(1), |acc, a| acc * a)
    }
}
impl<'a, const MOD: u32> Product<&'a Self> for GF<MOD> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

#[macro_export]
macro_rules! gf {
    ($value:expr) => {
        $crate::GF::from($value)
    };
    ($value:expr, $p:expr) => {
        $crate::GF::<$p>::from($value)
    };
}
macro_rules! gf_new_from_signed {
    ($($t:ty), *) => {
        $(
            impl<const MOD: u32> From<$t> for GF<MOD> {
                fn from(x: $t) -> Self {
                    Self::new(
                        (x as i64).rem_euclid(MOD as i64) as u32
                    )
                }
            }
        )*
    };
}
gf_new_from_signed!(i8, i16, i32, i64, i128, isize);

macro_rules! gf_new_from_unsigned {
    ($($t:ty), *) => {
        $(
            impl<const MOD: u32> From<$t> for GF<MOD> {
                fn from(x: $t) -> Self {
                    Self::new(x as u32)
                }
            }
        )*
    };
}
gf_new_from_unsigned!(u8, u16, u32, u64, u128, usize);
