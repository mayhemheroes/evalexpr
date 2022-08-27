use std::fmt::{Debug, Display};
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Sub, Not, Shl, Shr};
use std::str::FromStr;

/// An integer type usable with evalexpr.
/// The type parameter `FloatType` is the corresponding floating point type.
pub trait Integer<FloatType>: Number + Ord {
    /// Convert the integer into its corresponding floating point type.
    fn as_float(&self) -> FloatType;

    /// Return the bitwise and of `self` and `other`.
    fn bitand(&self, other: &Self) -> Self;
    /// Return the bitwise or of `self` and `other`.
    fn bitor(&self, other: &Self) -> Self;
    /// Return the bitwise xor of `self` and `other`.
    fn bitxor(&self, other: &Self) -> Self;
    /// Return the bitwise not of `self`.
    fn bitnot(&self) -> Self;
    /// Return `self` shifted to the left by `other` bits, shifting in zeros from the least significant bit.
    fn shl(&self, other: &Self) -> Self;
    /// Return `self` shifted to the right by `other` bits, without rotation.
    fn shr(&self, other: &Self) -> Self;

    /// Return the sum of `self` and `other`, or `None`, if an overflow occurred.
    fn checked_add(&self, other: &Self) -> Option<Self>;
    /// Return the difference of `self` and `other`, or `None`, if an overflow occurred.
    fn checked_sub(&self, other: &Self) -> Option<Self>;
    /// Return the product of `self` and `other`, or `None`, if an overflow occurred.
    fn checked_mul(&self, other: &Self) -> Option<Self>;
    /// Return the quotient of `self` and `other`, or `None`, if an overflow occurred.
    fn checked_div(&self, other: &Self) -> Option<Self>;
    /// Return the remainder of dividing `self` by `other`, or `None`, if an overflow occurred.
    fn checked_rem(&self, other: &Self) -> Option<Self>;
    /// Return the negation of `self`, or `None`, if an overflow occurred.
    fn checked_neg(&self) -> Option<Self>;

    /// Convert `usize` to this type, ignoring any losses occurring during conversion.
    fn from_usize_lossy(value: usize) -> Self;
}

/// A floating point type usable with evalexpr.
/// The type parameter `IntType` is the corresponding integer type.
pub trait Float<IntType>: Number + RandSampleUniform + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Neg<Output = Self> + Rem<Output = Self>{
    /// Convert the float into its corresponding integer type.
    fn as_int(&self) -> IntType;

    /// Compute the logarithm to the base `e` of `self`.
    fn ln(&self) -> Self;
    /// Compute the logarithm to the base `other` of `self`.
    fn log(&self, other: &Self) -> Self;
    /// Compute the logarithm to the base `2` of `self`.
    fn log2(&self) -> Self;
    /// Compute the logarithm to the base `10` of `self`.
    fn log10(&self) -> Self;

    /// Compute `e` to the power of `self`.
    fn exp(&self) -> Self;
    /// Compute `2` to the power of `self`.
    fn exp2(&self) -> Self;

    /// Compute `self` to the power of `other`.
    fn pow(&self, other: &Self) -> Self;

    /// Compute the cosine of `self`.
    fn cos(&self) -> Self;
    /// Compute the arcus cosine of `self`.
    fn acos(&self) -> Self;
    /// Compute the hyperbolic cosine of `self`.
    fn cosh(&self) -> Self;
    /// Compute the hyperbolic arcus cosine of `self`.
    fn acosh(&self) -> Self;
    /// Compute the sine of `self`.
    fn sin(&self) -> Self;
    /// Compute the arcus sine of `self`.
    fn asin(&self) -> Self;
    /// Compute the hyperbolic sine of `self`.
    fn sinh(&self) -> Self;
    /// Compute the hyperbolic arcus sine of `self`.
    fn asinh(&self) -> Self;

    /// Compute the tangent of `self`.
    fn tan(&self) -> Self;
    /// Compute the arcus tangent of `self`.
    fn atan(&self) -> Self;
    /// Compute the hyperbolic tangent of `self`.
    fn tanh(&self) -> Self;
    /// Compute the hyperbolic arcus tangent of `self`.
    fn atanh(&self) -> Self;
    /// Compute the four quadrant arcus tangent of `self` and `other`.
    fn atan2(&self, other: &Self) -> Self;

    /// Compute the square root of `self`.
    fn sqrt(&self) -> Self;
    /// Compute the cubic root of `self`.
    fn cbrt(&self) -> Self;

    /// Compute the length of the hypotenuse of a right-angle triangle given legs of length `self` and `other`.
    fn hypot(&self, other: &Self) -> Self;

    /// Compute the number rounded down to the next integer.
    fn floor(&self) -> Self;
    /// Compute the number rounded to the next integer.
    /// Rounding half-way cases is implementation-defined.
    /// For types in the standard library, this must behave the same as in the standard library.
    fn round(&self) -> Self;
    /// Compute the number rounded up to the next integer.
    fn ceil(&self) -> Self;

    /// Returns `true` if this number is `NaN`.
    fn is_nan(&self) -> bool;
    /// Returns `true` if this number is finite.
    fn is_finite(&self) -> bool;
    /// Returns `true` if this number is infinite.
    fn is_infinite(&self) -> bool;
    /// Returns `true` if this number is normal by the definition in the Rust standard library.
    /// See also [f64::is_normal].
    fn is_normal(&self) -> bool;
}

/// A number type usable with evalexpr.
/// This trait contains methods that are common between integers and floating point numbers.
pub trait Number: PartialOrd + Sized + Clone + FromStr + Display + Debug {
    /// Return the maximum finite value representable by this type.
    /// If the type is unbounded, then return `None`.
    fn min_value() -> Option<Self>;

    /// Return the minimum finite value representable by this type.
    /// If the type is unbounded, then return `None`.
    fn max_value() -> Option<Self>;


    /// Return the minimum of `self` and `other`.
    fn min<'this: 'result, 'other: 'result, 'result>(&'this self, other: &'other Self) -> &'result Self;
    /// Return the maximum of `self` and `other`.
    fn max<'this: 'result, 'other: 'result, 'result>(&'this self, other: &'other Self) -> &'result Self;
}

#[cfg(not(feature = "rand"))]
pub trait RandSampleUniform {}

#[cfg(feature = "rand")]
pub trait RandSampleUniform: rand::distributions::uniform::SampleUniform {}

impl Number for i64 {
    fn min_value() -> Option<Self> {
        Some(i64::MIN)
    }

    fn max_value() -> Option<Self> {
        Some(i64::MAX)
    }

    fn min<'this: 'result, 'other: 'result, 'result>(&'this self, other: &'other Self) -> &'result Self {
        if self < other {
            self
        } else {
            other
        }
    }

    fn max<'this: 'result, 'other: 'result, 'result>(&'this self, other: &'other Self) -> &'result Self {
        if self > other {
            self
        } else {
            other
        }
    }
}

impl Integer<f64> for i64 {
    fn as_float(&self) -> f64 {
        *self as f64
    }


    fn bitand(&self, other: &Self) -> Self {
        BitAnd::bitand(*self, *other)
    }

    fn bitor(&self, other: &Self) -> Self {
        BitOr::bitor(*self, *other)
    }

    fn bitxor(&self, other: &Self) -> Self {
        BitXor::bitxor(*self, *other)
    }

    fn bitnot(&self) -> Self {
        Not::not(*self)
    }

    fn shl(&self, other: &Self) -> Self {
        Shl::shl(*self, *other)
    }

    fn shr(&self, other: &Self) -> Self {
        Shr::shr(*self, *other)
    }

    fn checked_add(&self, other: &Self) -> Option<Self> {
        i64::checked_add(*self, *other)
    }

    fn checked_sub(&self, other: &Self) -> Option<Self> {
        i64::checked_sub(*self, *other)
    }

    fn checked_mul(&self, other: &Self) -> Option<Self> {
        i64::checked_mul(*self, *other)
    }

    fn checked_div(&self, other: &Self) -> Option<Self> {
        i64::checked_div(*self, *other)
    }

    fn checked_rem(&self, other: &Self) -> Option<Self> {
        i64::checked_rem(*self, *other)
    }

    fn checked_neg(&self) -> Option<Self> {
        i64::checked_neg(*self)
    }

    fn from_usize_lossy(value: usize) -> Self {
        value as Self
    }
}

impl Number for f64 {
    fn min_value() -> Option<Self> {
        let result = f64::MIN - 1.0;
        debug_assert!(result.is_infinite());
        debug_assert!(result.is_sign_negative());
        Some(result)
    }

    fn max_value() -> Option<Self> {
        let result = f64::MAX + 1.0;
        debug_assert!(result.is_infinite());
        debug_assert!(result.is_sign_positive());
        Some(result)
    }

    fn min<'this: 'result, 'other: 'result, 'result>(&'this self, other: &'other Self) -> &'result Self {
        if self < other {self} else {other}
    }

    fn max<'this: 'result, 'other: 'result, 'result>(&'this self, other: &'other Self) -> &'result Self {
        if self > other {self} else {other}
    }
}

impl RandSampleUniform for f64 {}

impl Float<i64> for f64 {
    fn as_int(&self) -> i64 {
        *self as i64
    }

    fn ln(&self) -> Self {
        f64::ln(*self)
    }

    fn log(&self, base: &Self) -> Self {
        f64::log(*self, *base)
    }

    fn log2(&self) -> Self {
        f64::log2(*self)
    }

    fn log10(&self) -> Self {
        f64::log10(*self)
    }

    fn exp(&self) -> Self {
        f64::exp(*self)
    }

    fn exp2(&self) -> Self {
        f64::exp2(*self)
    }

    fn pow(&self, other: &Self) -> Self {
        f64::powf(*self, *other)
    }

    fn cos(&self) -> Self {
        f64::cos(*self)
    }

    fn acos(&self) -> Self {
        f64::acos(*self)
    }

    fn cosh(&self) -> Self {
        f64::cosh(*self)
    }

    fn acosh(&self) -> Self {
        f64::acosh(*self)
    }

    fn sin(&self) -> Self {
        f64::sin(*self)
    }

    fn asin(&self) -> Self {
        f64::asin(*self)
    }

    fn sinh(&self) -> Self {
        f64::sinh(*self)
    }

    fn asinh(&self) -> Self {
        f64::asinh(*self)
    }

    fn tan(&self) -> Self {
        f64::tan(*self)
    }

    fn atan(&self) -> Self {
        f64::atan(*self)
    }

    fn tanh(&self) -> Self {
        f64::tanh(*self)
    }

    fn atanh(&self) -> Self {
        f64::atanh(*self)
    }

    fn atan2(&self, other: &Self) -> Self {
        f64::atan2(*self, *other)
    }

    fn sqrt(&self) -> Self {
        f64::sqrt(*self)
    }

    fn cbrt(&self) -> Self {
        f64::cbrt(*self)
    }

    fn hypot(&self, other: &Self) -> Self {
        f64::hypot(*self, *other)
    }

    fn floor(&self) -> Self {
        f64::floor(*self)
    }

    fn round(&self) -> Self {
        f64::round(*self)
    }

    fn ceil(&self) -> Self {
        f64::ceil(*self)
    }

    fn is_nan(&self) -> bool {
        f64::is_nan(*self)
    }

    fn is_finite(&self) -> bool {
        f64::is_finite(*self)
    }

    fn is_infinite(&self) -> bool {
        f64::is_infinite(*self)
    }

    fn is_normal(&self) -> bool {
        f64::is_normal(*self)
    }
}