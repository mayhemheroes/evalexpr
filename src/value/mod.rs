use crate::error::{EvalexprError, EvalexprResult};
use std::convert::TryFrom;
use crate::value::numeric_types::Integer;

mod display;
pub mod value_type;
pub mod numeric_types;

/// The type used to represent tuples in `Value::Tuple`.
pub type TupleType<IntType, FloatType> = Vec<Value<IntType, FloatType>>;

/// The type used to represent empty values in `Value::Empty`.
pub type EmptyType = ();

/// The value of the empty type to be used in rust.
pub const EMPTY_VALUE: () = ();

#[cfg(test)]
pub type DefaultValue = Value<i64, f64>;

/// The value type used by the parser.
/// Values can be of different subtypes that are the variants of this enum.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum Value<IntType = i64, FloatType = f64> {
    /// A string value.
    String(String),
    /// A float value.
    Float(FloatType),
    /// An integer value.
    Int(IntType),
    /// A boolean value.
    Boolean(bool),
    /// A tuple value.
    Tuple(TupleType<IntType, FloatType>),
    /// An empty value.
    Empty,
}

impl<IntType, FloatType> Value<IntType, FloatType> {
    /// Returns true if `self` is a `Value::String`.
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }
    /// Returns true if `self` is a `Value::Int`.
    pub fn is_int(&self) -> bool {
        matches!(self, Value::Int(_))
    }

    /// Returns true if `self` is a `Value::Float`.
    pub fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    /// Returns true if `self` is a `Value::Int` or `Value::Float`.
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Int(_) | Value::Float(_))
    }

    /// Returns true if `self` is a `Value::Boolean`.
    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    /// Returns true if `self` is a `Value::Tuple`.
    pub fn is_tuple(&self) -> bool {
        matches!(self, Value::Tuple(_))
    }

    /// Returns true if `self` is a `Value::Empty`.
    pub fn is_empty(&self) -> bool {
        matches!(self, Value::Empty)
    }

    /// Create an integer value.
    pub fn int(integer: IntType) -> Self {
        Self::Int(integer)
    }

    /// Create a float value.
    pub fn float(float: FloatType) -> Self {
        Self::Float(float)
    }
}

impl<IntType: Clone, FloatType: Clone> Value<IntType, FloatType> {
    /// Clones the value stored in `self` as `String`, or returns `Err` if `self` is not a `Value::String`.
    pub fn as_string(&self) -> EvalexprResult<String, IntType, FloatType> {
        match self {
            Value::String(string) => Ok(string.clone()),
            value => Err(EvalexprError::expected_string(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `IntType`, or returns `Err` if `self` is not a `Value::Int`.
    pub fn as_int(&self) -> EvalexprResult<IntType, IntType, FloatType> {
        match self {
            Value::Int(i) => Ok(i.clone()),
            value => Err(EvalexprError::expected_int(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `FloatType`, or returns `Err` if `self` is not a `Value::Float`.
    pub fn as_float(&self) -> EvalexprResult<FloatType, IntType, FloatType> {
        match self {
            Value::Float(f) => Ok(f.clone()),
            value => Err(EvalexprError::expected_float(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `bool`, or returns `Err` if `self` is not a `Value::Boolean`.
    pub fn as_boolean(&self) -> EvalexprResult<bool, IntType, FloatType> {
        match self {
            Value::Boolean(boolean) => Ok(*boolean),
            value => Err(EvalexprError::expected_boolean(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `TupleType`, or returns `Err` if `self` is not a `Value::Tuple`.
    pub fn as_tuple(&self) -> EvalexprResult<TupleType<IntType, FloatType>, IntType, FloatType> {
        match self {
            Value::Tuple(tuple) => Ok(tuple.clone()),
            value => Err(EvalexprError::expected_tuple(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `TupleType` or returns `Err` if `self` is not a `Value::Tuple` of the required length.
    pub fn as_fixed_len_tuple(&self, len: usize) -> EvalexprResult<TupleType<IntType, FloatType>, IntType, FloatType> {
        match self {
            Value::Tuple(tuple) => {
                if tuple.len() == len {
                    Ok(tuple.clone())
                } else {
                    Err(EvalexprError::expected_fixed_len_tuple(len, self.clone()))
                }
            },
            value => Err(EvalexprError::expected_tuple(value.clone())),
        }
    }

    /// Returns `()`, or returns`Err` if `self` is not a `Value::Tuple`.
    pub fn as_empty(&self) -> EvalexprResult<(), IntType, FloatType> {
        match self {
            Value::Empty => Ok(()),
            value => Err(EvalexprError::expected_empty(value.clone())),
        }
    }
}

impl<IntType: Integer<FloatType>, FloatType: Clone> Value<IntType, FloatType> {
    /// Clones the value stored in  `self` as `FloatType`, or returns `Err` if `self` is not a `Value::Float` or `Value::Int`.
    /// Note that this method silently converts `IntType` to `FloatType`, if `self` is a `Value::Int`.
    pub fn as_number(&self) -> EvalexprResult<FloatType, IntType, FloatType> {
        match self {
            Value::Float(f) => Ok(f.clone()),
            Value::Int(i) => Ok(i.as_float()),
            value => Err(EvalexprError::expected_number(value.clone())),
        }
    }
}

impl<IntType, FloatType> From<String> for Value<IntType, FloatType> {
    fn from(string: String) -> Self {
        Value::String(string)
    }
}

impl<IntType, FloatType> From<&str> for Value<IntType, FloatType> {
    fn from(string: &str) -> Self {
        Value::String(string.to_string())
    }
}

/*impl<IntType, FloatType>> From<FloatType> for Value<IntType, FloatType> {
    fn from(float: FloatType) -> Self {
        Value::Float(float)
    }
}*/

/*impl<IntType, FloatType> From<IntType> for Value<IntType, FloatType> {
    fn from(int: IntType) -> Self {
        Value::Int(int)
    }
}*/

impl<IntType, FloatType> From<bool> for Value<IntType, FloatType> {
    fn from(boolean: bool) -> Self {
        Value::Boolean(boolean)
    }
}

impl<IntType, FloatType> From<TupleType<IntType, FloatType>> for Value<IntType, FloatType> {
    fn from(tuple: TupleType<IntType, FloatType>) -> Self {
        Value::Tuple(tuple)
    }
}

impl<IntType, FloatType> From<Value<IntType, FloatType>> for EvalexprResult<Value<IntType, FloatType>, IntType, FloatType> {
    fn from(value: Value<IntType, FloatType>) -> Self {
        Ok(value)
    }
}

impl<IntType, FloatType> From<()> for Value<IntType, FloatType> {
    fn from(_: ()) -> Self {
        Value::Empty
    }
}

impl<IntType, FloatType> TryFrom<Value<IntType, FloatType>> for String {
    type Error = EvalexprError<IntType, FloatType>;

    fn try_from(value: Value<IntType, FloatType>) -> Result<Self, Self::Error> {
        if let Value::String(value) = value {
            Ok(value)
        } else {
            Err(EvalexprError::ExpectedString { actual: value })
        }
    }
}

/*impl<IntType, FloatType> TryFrom<Value<IntType, FloatType>> for FloatType {
    type Error = EvalexprError;

    fn try_from(value: Value<IntType, FloatType>) -> Result<Self, Self::Error> {
        if let Value::Float(value) = value {
            Ok(value)
        } else {
            Err(EvalexprError::ExpectedFloat { actual: value })
        }
    }
}*/

/*impl<IntType, FloatType> TryFrom<Value<IntType, FloatType>> for IntType {
    type Error = EvalexprError;

    fn try_from(value: Value<IntType, FloatType>) -> Result<Self, Self::Error> {
        if let Value::Int(value) = value {
            Ok(value)
        } else {
            Err(EvalexprError::ExpectedInt { actual: value })
        }
    }
}*/

impl<IntType, FloatType> TryFrom<Value<IntType, FloatType>> for bool {
    type Error = EvalexprError<IntType, FloatType>;

    fn try_from(value: Value<IntType, FloatType>) -> Result<Self, Self::Error> {
        if let Value::Boolean(value) = value {
            Ok(value)
        } else {
            Err(EvalexprError::ExpectedBoolean { actual: value })
        }
    }
}

impl<IntType, FloatType> TryFrom<Value<IntType, FloatType>> for TupleType<IntType, FloatType> {
    type Error = EvalexprError<IntType, FloatType>;

    fn try_from(value: Value<IntType, FloatType>) -> Result<Self, Self::Error> {
        if let Value::Tuple(value) = value {
            Ok(value)
        } else {
            Err(EvalexprError::ExpectedTuple { actual: value })
        }
    }
}

impl<IntType, FloatType> TryFrom<Value<IntType, FloatType>> for () {
    type Error = EvalexprError<IntType, FloatType>;

    fn try_from(value: Value<IntType, FloatType>) -> Result<Self, Self::Error> {
        if let Value::Empty = value {
            Ok(())
        } else {
            Err(EvalexprError::ExpectedEmpty { actual: value })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::value::{DefaultValue, TupleType, Value};

    #[test]
    fn test_value_conversions() {
        assert_eq!(
            DefaultValue::from("string").as_string(),
            Ok(String::from("string"))
        );
        assert_eq!(DefaultValue::int(3).as_int(), Ok(3));
        assert_eq!(DefaultValue::float(3.3).as_float(), Ok(3.3));
        assert_eq!(DefaultValue::from(true).as_boolean(), Ok(true));
        assert_eq!(
            DefaultValue::from(TupleType::new()).as_tuple(),
            Ok(TupleType::new())
        );
    }

    #[test]
    fn test_value_checks() {
        assert!(DefaultValue::from("string").is_string());
        assert!(DefaultValue::int(3).is_int());
        assert!(DefaultValue::float(3.3).is_float());
        assert!(DefaultValue::from(true).is_boolean());
        assert!(DefaultValue::from(TupleType::new()).is_tuple());
    }
}
