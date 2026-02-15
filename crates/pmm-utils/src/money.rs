// ./pmm-utils/src/money.rs

//! # Scaled Integer Arithmetic for Monetary Values
//!
//! This module provides [`ScaledInt`], a fixed-point decimal type that stores
//! monetary values as scaled `i64` integers. This eliminates IEEE 754
//! floating-point rounding errors that silently corrupt financial calculations.
//!
//! ## Why not `f64`?
//!
//! ```text
//! 0.1 + 0.2 = 0.30000000000000004  (f64)
//! 0.1 + 0.2 = 0.3                  (ScaledInt)
//! ```
//!
//! In portfolio management, these errors compound across thousands of
//! transactions, share lots, fee calculations, and P&L aggregations,
//! eventually producing incorrect balances, tax figures, and audit trails.
//!
//! ## Design
//!
//! - **Scale factor**: 10^8 (100,000,000) — 8 decimal digits of precision.
//! - **Storage**: `i64` — compact, fast, and compatible with databases.
//! - **Overflow protection**: Multiplication and division promote to `i128`
//!   intermediates before rescaling, preventing silent overflow.
//! - **Range**: ±92,233,720,368.54775807 — sufficient for any single
//!   instrument price, position size, or portfolio value.
//!
//! ## Usage
//!
//! ```rust
//! use pmm_utils::money::ScaledInt;
//!
//! let price  = ScaledInt::from_f64(152.35);
//! let shares = ScaledInt::from_f64(10.0);
//! let total  = price * shares; // exact: 1523.50000000
//!
//! assert_eq!(total.to_f64(), 1523.5);
//! ```

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

// ───────────────────────────── Constants ─────────────────────────────

/// Number of decimal digits of fractional precision.
pub const SCALE_DIGITS: u32 = 8;

/// The scale factor: 10^[`SCALE_DIGITS`].
pub const SCALE: i64 = 10_i64.pow(SCALE_DIGITS); // 100_000_000

// ───────────────────────────── Core Type ─────────────────────────────

/// A fixed-point decimal number backed by an `i64` with 8 fractional digits.
///
/// Internally `ScaledInt(100_000_000)` represents the value `1.00000000`.
///
/// This type is [`Copy`], [`Ord`], and [`Hash`] — it can be used as a map key,
/// sorted, compared for equality, and stored in sets without any of the
/// pitfalls that come with floating-point comparisons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ScaledInt(i64);

// ───────────────────────────── Constructors ──────────────────────────

impl ScaledInt {
    /// The additive identity (0).
    pub const ZERO: Self = Self(0);

    /// The multiplicative identity (1).
    pub const ONE: Self = Self(SCALE);

    // ── From raw / primitives ───────────────────────────────────────

    /// Create a `ScaledInt` directly from the internal representation.
    ///
    /// **Use with care** — `raw` must already be pre-scaled.
    #[inline]
    pub const fn from_raw(raw: i64) -> Self {
        Self(raw)
    }

    /// Return the raw underlying `i64` value (pre-scaled).
    #[inline]
    pub const fn raw(self) -> i64 {
        self.0
    }

    /// Create a `ScaledInt` from a whole-number amount (no fractional part).
    ///
    /// ```
    /// # use pmm_utils::money::ScaledInt;
    /// let five = ScaledInt::from_integer(5);
    /// assert_eq!(five.to_f64(), 5.0);
    /// ```
    #[inline]
    pub const fn from_integer(n: i64) -> Self {
        Self(n * SCALE)
    }

    /// Create a `ScaledInt` from an `f64`.
    ///
    /// The conversion rounds to the nearest representable value at 8 decimal
    /// places. This is the *only* point where floating-point is involved —
    /// all subsequent arithmetic is exact integer math.
    ///
    /// # Panics
    ///
    /// Panics if `value` is `NaN`, `+Inf`, or `-Inf`.
    #[inline]
    pub fn from_f64(value: f64) -> Self {
        assert!(
            value.is_finite(),
            "ScaledInt::from_f64 received a non-finite value: {value}"
        );
        // Round to nearest to minimize conversion error.
        Self((value * SCALE as f64).round() as i64)
    }

    /// Create a `ScaledInt` from a string representation of a decimal number.
    ///
    /// Supports formats like `"123.456"`, `"-0.001"`, `"42"`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the string cannot be parsed as a decimal number.
    pub fn from_str_decimal(s: &str) -> Result<Self, ScaledIntParseError> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ScaledIntParseError::Empty);
        }

        let (negative, s) = if let Some(rest) = s.strip_prefix('-') {
            (true, rest)
        } else {
            (false, s)
        };

        let (integer_part, fractional_part) = match s.split_once('.') {
            Some((int_s, frac_s)) => (int_s, frac_s),
            None => (s, ""),
        };

        let int_val: i64 = if integer_part.is_empty() {
            0
        } else {
            integer_part
                .parse::<i64>()
                .map_err(|_| ScaledIntParseError::InvalidFormat(s.to_string()))?
        };

        // Pad or truncate fractional part to SCALE_DIGITS digits
        let frac_str = if fractional_part.len() >= SCALE_DIGITS as usize {
            &fractional_part[..SCALE_DIGITS as usize]
        } else {
            // Will be padded below
            fractional_part
        };

        let frac_val: i64 = if frac_str.is_empty() {
            0
        } else {
            let padded = format!("{:0<width$}", frac_str, width = SCALE_DIGITS as usize);
            padded
                .parse::<i64>()
                .map_err(|_| ScaledIntParseError::InvalidFormat(s.to_string()))?
        };

        let raw = int_val * SCALE + frac_val;
        Ok(Self(if negative { -raw } else { raw }))
    }

    // ── To primitives ───────────────────────────────────────────────

    /// Convert to `f64` for display, plotting, or interop with external APIs.
    ///
    /// This is a *lossy* conversion — use it at the boundary (UI, charts),
    /// never for further arithmetic.
    #[inline]
    pub fn to_f64(self) -> f64 {
        self.0 as f64 / SCALE as f64
    }

    /// Return the integer (whole-number) part, truncating toward zero.
    #[inline]
    pub const fn integer_part(self) -> i64 {
        self.0 / SCALE
    }

    /// Return the fractional part as a raw scaled value.
    #[inline]
    pub const fn fractional_part(self) -> i64 {
        self.0 % SCALE
    }

    // ── Helpers ─────────────────────────────────────────────────────

    /// Absolute value.
    #[inline]
    pub const fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Returns `true` if the value is zero.
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if the value is strictly positive.
    #[inline]
    pub const fn is_positive(self) -> bool {
        self.0 > 0
    }

    /// Returns `true` if the value is strictly negative.
    #[inline]
    pub const fn is_negative(self) -> bool {
        self.0 < 0
    }

    /// Returns the minimum of two values.
    #[inline]
    pub const fn min(self, other: Self) -> Self {
        if self.0 <= other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of two values.
    #[inline]
    pub const fn max(self, other: Self) -> Self {
        if self.0 >= other.0 {
            self
        } else {
            other
        }
    }

    /// Sum an iterator of `ScaledInt` values.
    pub fn sum_iter(iter: impl Iterator<Item = ScaledInt>) -> Self {
        iter.fold(Self::ZERO, |acc, x| acc + x)
    }
}

// ───────────────────────────── Arithmetic ────────────────────────────

impl Add for ScaledInt {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(
            self.0
                .checked_add(rhs.0)
                .expect("ScaledInt addition overflow"),
        )
    }
}

impl Sub for ScaledInt {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(
            self.0
                .checked_sub(rhs.0)
                .expect("ScaledInt subtraction overflow"),
        )
    }
}

/// Multiplication uses `i128` intermediates to avoid overflow.
///
/// `(a × SCALE) × (b × SCALE) = a×b × SCALE²`, so we divide by `SCALE`
/// once to get back to `a×b × SCALE`.
impl Mul for ScaledInt {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        let wide = (self.0 as i128) * (rhs.0 as i128);
        Self((wide / SCALE as i128) as i64)
    }
}

/// Division uses `i128` intermediates to preserve precision.
///
/// `(a × SCALE) / (b × SCALE) = a/b`, so we pre-multiply the numerator
/// by `SCALE` to get `(a/b) × SCALE`.
impl Div for ScaledInt {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        assert!(!rhs.is_zero(), "ScaledInt division by zero");
        let wide = (self.0 as i128) * (SCALE as i128);
        Self((wide / rhs.0 as i128) as i64)
    }
}

impl Neg for ScaledInt {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl AddAssign for ScaledInt {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for ScaledInt {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

// Allow `ScaledInt * i64` and `i64 * ScaledInt` for whole-number scaling.
impl Mul<i64> for ScaledInt {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i64) -> Self {
        Self(
            self.0
                .checked_mul(rhs)
                .expect("ScaledInt * i64 overflow"),
        )
    }
}

impl Mul<ScaledInt> for i64 {
    type Output = ScaledInt;
    #[inline]
    fn mul(self, rhs: ScaledInt) -> ScaledInt {
        rhs * self
    }
}

impl Div<i64> for ScaledInt {
    type Output = Self;
    #[inline]
    fn div(self, rhs: i64) -> Self {
        assert!(rhs != 0, "ScaledInt division by zero (i64)");
        Self(self.0 / rhs)
    }
}

// ───────────────────── Iterator Sum ─────────────────────────────────

impl std::iter::Sum for ScaledInt {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::sum_iter(iter)
    }
}

// ───────────────────────────── Display / Debug ───────────────────────

impl fmt::Display for ScaledInt {
    /// Formats the value as a human-readable decimal string with up to
    /// 8 fractional digits, trimming trailing zeros.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let abs = self.0.unsigned_abs();
        let int_part = abs / SCALE as u64;
        let frac_part = abs % SCALE as u64;

        if self.0 < 0 {
            write!(f, "-")?;
        }

        if frac_part == 0 {
            write!(f, "{int_part}.0")
        } else {
            // Format fractional part with leading zeros, then trim trailing zeros.
            let frac_str = format!("{:0>width$}", frac_part, width = SCALE_DIGITS as usize);
            let trimmed = frac_str.trim_end_matches('0');
            write!(f, "{int_part}.{trimmed}")
        }
    }
}

// ───────────────────────────── Serde ─────────────────────────────────

/// Serializes as a decimal string `"123.456"` for JSON / database
/// interoperability. This avoids any precision loss that would occur
/// if we serialized as an `f64`.
impl Serialize for ScaledInt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ScaledInt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        ScaledInt::from_str_decimal(&s).map_err(serde::de::Error::custom)
    }
}

// ───────────────────────────── Errors ────────────────────────────────

/// Errors that can occur when parsing a string into a [`ScaledInt`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaledIntParseError {
    /// The input string was empty.
    Empty,
    /// The input string was not a valid decimal number.
    InvalidFormat(String),
}

impl fmt::Display for ScaledIntParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty string cannot be parsed as ScaledInt"),
            Self::InvalidFormat(s) => write!(f, "invalid decimal format: '{s}'"),
        }
    }
}

impl std::error::Error for ScaledIntParseError {}

// ───────────────────────────── From impls ────────────────────────────

impl From<i64> for ScaledInt {
    /// Converts a whole number `i64` into a `ScaledInt`.
    #[inline]
    fn from(n: i64) -> Self {
        Self::from_integer(n)
    }
}

impl From<i32> for ScaledInt {
    #[inline]
    fn from(n: i32) -> Self {
        Self::from_integer(n as i64)
    }
}

// ───────────────────────────── Tests ─────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_construction() {
        let v = ScaledInt::from_f64(1.0);
        assert_eq!(v.raw(), SCALE);
        assert_eq!(v.to_f64(), 1.0);

        let v = ScaledInt::from_integer(42);
        assert_eq!(v.to_f64(), 42.0);

        let v = ScaledInt::ZERO;
        assert!(v.is_zero());
    }

    #[test]
    fn test_floating_point_problem_solved() {
        // The classic floating-point pitfall:
        // In f64: 0.1 + 0.2 = 0.30000000000000004
        let a = ScaledInt::from_f64(0.1);
        let b = ScaledInt::from_f64(0.2);
        let c = a + b;
        assert_eq!(c, ScaledInt::from_f64(0.3));
        assert_eq!(c.to_string(), "0.3");
    }

    #[test]
    fn test_addition_subtraction() {
        let a = ScaledInt::from_f64(100.50);
        let b = ScaledInt::from_f64(49.50);
        assert_eq!((a + b).to_f64(), 150.0);
        assert_eq!((a - b).to_f64(), 51.0);
    }

    #[test]
    fn test_multiplication() {
        let price = ScaledInt::from_f64(152.35);
        let shares = ScaledInt::from_f64(10.0);
        let total = price * shares;
        assert_eq!(total.to_f64(), 1523.5);
    }

    #[test]
    fn test_division() {
        let total = ScaledInt::from_f64(1523.5);
        let shares = ScaledInt::from_f64(10.0);
        let price = total / shares;
        assert_eq!(price.to_f64(), 152.35);
    }

    #[test]
    fn test_negation() {
        let v = ScaledInt::from_f64(42.0);
        assert_eq!((-v).to_f64(), -42.0);
    }

    #[test]
    fn test_ordering() {
        let a = ScaledInt::from_f64(1.0);
        let b = ScaledInt::from_f64(2.0);
        assert!(a < b);
        assert!(b > a);
        assert_eq!(a.max(b), b);
        assert_eq!(a.min(b), a);
    }

    #[test]
    fn test_display() {
        assert_eq!(ScaledInt::from_f64(123.456).to_string(), "123.456");
        assert_eq!(ScaledInt::from_f64(-0.001).to_string(), "-0.001");
        assert_eq!(ScaledInt::from_integer(42).to_string(), "42.0");
        assert_eq!(ScaledInt::ZERO.to_string(), "0.0");
    }

    #[test]
    fn test_from_str_decimal() {
        assert_eq!(
            ScaledInt::from_str_decimal("123.456").unwrap(),
            ScaledInt::from_f64(123.456)
        );
        assert_eq!(
            ScaledInt::from_str_decimal("-0.001").unwrap(),
            ScaledInt::from_f64(-0.001)
        );
        assert_eq!(
            ScaledInt::from_str_decimal("42").unwrap(),
            ScaledInt::from_integer(42)
        );
        assert!(ScaledInt::from_str_decimal("").is_err());
        assert!(ScaledInt::from_str_decimal("abc").is_err());
    }

    #[test]
    fn test_serde_roundtrip() {
        let original = ScaledInt::from_f64(99.99);
        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(json, "\"99.99\"");
        let parsed: ScaledInt = serde_json::from_str(&json).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_sum_iterator() {
        let values = vec![
            ScaledInt::from_f64(10.0),
            ScaledInt::from_f64(20.0),
            ScaledInt::from_f64(30.0),
        ];
        let total: ScaledInt = values.into_iter().sum();
        assert_eq!(total.to_f64(), 60.0);
    }

    #[test]
    fn test_mul_by_integer() {
        let price = ScaledInt::from_f64(9.99);
        let total = price * 3_i64;
        assert_eq!(total.to_f64(), 29.97);
    }

    #[test]
    fn test_abs() {
        let v = ScaledInt::from_f64(-42.5);
        assert_eq!(v.abs().to_f64(), 42.5);
    }

    #[test]
    fn test_add_assign() {
        let mut v = ScaledInt::from_f64(10.0);
        v += ScaledInt::from_f64(5.0);
        assert_eq!(v.to_f64(), 15.0);
    }

    #[test]
    fn test_sub_assign() {
        let mut v = ScaledInt::from_f64(10.0);
        v -= ScaledInt::from_f64(3.0);
        assert_eq!(v.to_f64(), 7.0);
    }

    #[test]
    #[should_panic(expected = "non-finite")]
    fn test_from_f64_nan_panics() {
        ScaledInt::from_f64(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "non-finite")]
    fn test_from_f64_inf_panics() {
        ScaledInt::from_f64(f64::INFINITY);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_div_by_zero_panics() {
        let _ = ScaledInt::from_f64(1.0) / ScaledInt::ZERO;
    }

    #[test]
    fn test_financial_fee_calculation() {
        // Buy 150 shares at $42.37 each, with $9.99 commission
        let shares = ScaledInt::from_f64(150.0);
        let price = ScaledInt::from_f64(42.37);
        let fee = ScaledInt::from_f64(9.99);
        let total_cost = (shares * price) + fee;
        // 150 * 42.37 = 6355.50, + 9.99 = 6365.49
        assert_eq!(total_cost.to_f64(), 6365.49);
    }
}
