#[cfg(feature = "regex_support")]
use regex::Regex;

use crate::{
    EvalexprError, Function, Value, ValueType, TupleType,
};
use crate::value::numeric_types::{Float, Integer};

macro_rules! simple_math {
    ($func:ident) => {
        Some(Function::new(|argument| {
            let num: FloatType = argument.as_number()?;
            Ok(Value::Float(num.$func()))
        }))
    };
    ($func:ident, 2) => {
        Some(Function::new(|argument| {
            let tuple: TupleType<IntType, FloatType> = argument.as_fixed_len_tuple(2)?;
            let (a, b) = (tuple[0].as_number()?, tuple[1].as_number()?);
            Ok(Value::Float(a.$func(&b)))
        }))
    };
}

macro_rules! float_is {
    ($func:ident) => {
        Some(Function::new(|argument| {
            let num: FloatType = argument.as_number()?;
            Ok(Value::Boolean(num.$func()))
        }))
    }
}

macro_rules! int_function {
    ($func:ident) => {
        Some(Function::new(|argument| {
            let int: IntType = argument.as_int()?;
            Ok(Value::Int(int.$func()))
        }))
    };
    ($func:ident, 2) => {
        Some(Function::new(|argument| {
            let tuple: TupleType<IntType, FloatType> = argument.as_fixed_len_tuple(2)?;
            let (a, b) = (tuple[0].as_int()?, tuple[1].as_int()?);
            Ok(Value::Int(a.$func(&b)))
        }))
    };
}

pub fn builtin_function<IntType: Integer<FloatType>, FloatType: Float<IntType>>(identifier: &str) -> Option<Function<IntType, FloatType>> {
    match identifier {
        // Log
        "math::ln" => simple_math!(ln),
        "math::log" => simple_math!(log, 2),
        "math::log2" => simple_math!(log2),
        "math::log10" => simple_math!(log10),
        // Exp
        "math::exp" => simple_math!(exp),
        "math::exp2" => simple_math!(exp2),
        // Pow
        "math::pow" => simple_math!(pow, 2),
        // Cos
        "math::cos" => simple_math!(cos),
        "math::acos" => simple_math!(acos),
        "math::cosh" => simple_math!(cosh),
        "math::acosh" => simple_math!(acosh),
        // Sin
        "math::sin" => simple_math!(sin),
        "math::asin" => simple_math!(asin),
        "math::sinh" => simple_math!(sinh),
        "math::asinh" => simple_math!(asinh),
        // Tan
        "math::tan" => simple_math!(tan),
        "math::atan" => simple_math!(atan),
        "math::tanh" => simple_math!(tanh),
        "math::atanh" => simple_math!(atanh),
        "math::atan2" => simple_math!(atan2, 2),
        // Root
        "math::sqrt" => simple_math!(sqrt),
        "math::cbrt" => simple_math!(cbrt),
        // Hypotenuse
        "math::hypot" => simple_math!(hypot, 2),
        // Rounding
        "floor" => simple_math!(floor),
        "round" => simple_math!(round),
        "ceil" => simple_math!(ceil),
        // Float special values
        "math::is_nan" => float_is!(is_nan),
        "math::is_finite" => float_is!(is_finite),
        "math::is_infinite" => float_is!(is_infinite),
        "math::is_normal" => float_is!(is_normal),
        // Other
        "typeof" => Some(Function::new(move |argument| {
            Ok(match argument {
                Value::String(_) => "string",
                Value::Float(_) => "float",
                Value::Int(_) => "int",
                Value::Boolean(_) => "boolean",
                Value::Tuple(_) => "tuple",
                Value::Empty => "empty",
            }
            .into())
        })),
        "min" => Some(Function::new(|argument: &Value<IntType, FloatType>| {
            let arguments = argument.as_tuple()?;
            let min_int = IntType::min_value();
            let min_float = FloatType::min_value();
            let mut min_int = min_int.as_ref();
            let mut min_float = min_float.as_ref();

            for argument in &arguments {
                if let Value::Float(float) = argument {
                    min_float = min_float.map(|this| this.min(float));
                } else if let Value::Int(int) = argument {
                    min_int = min_int.map(|this|this.min(int));
                } else {
                    return Err(EvalexprError::expected_number(argument.clone()));
                }
            }

            if let (Some(min_int), Some(min_float)) = (min_int, min_float) {
                if &min_int.as_float() < min_float {
                    Ok(Value::Int(min_int.clone()))
                } else {
                    Ok(Value::Float(min_float.clone()))
                }
            } else if let Some(min_int) = min_int {
                Ok(Value::Int(min_int.clone()))
            } else if let Some(min_float) = min_float {
                Ok(Value::Float(min_float.clone()))
            } else {
                Err(EvalexprError::NoMinValue)
            }         
        })),
        "max" => Some(Function::new(|argument: &Value<IntType, FloatType>| {
            let arguments = argument.as_tuple()?;
            let max_int = IntType::min_value();
            let max_float = FloatType::min_value();
            let mut max_int = max_int.as_ref();
            let mut max_float = max_float.as_ref();

            for argument in &arguments {
                if let Value::Float(float) = argument {
                    max_float = max_float.map(|this|this.max(float));
                } else if let Value::Int(int) = argument {
                    max_int = max_int.map(|this|this.max(int));
                } else {
                    return Err(EvalexprError::expected_number(argument.clone()));
                }
            }

            if let (Some(max_int), Some(max_float)) = (max_int, max_float) {
                if &max_int.as_float() > max_float {
                    Ok(Value::Int(max_int.clone()))
                } else {
                    Ok(Value::Float(max_float.clone()))
                }
            } else if let Some(max_int) = max_int {
                Ok(Value::Int(max_int.clone()))
            } else if let Some(max_float) = max_float {
                Ok(Value::Float(max_float.clone()))
            } else {
                Err(EvalexprError::NoMinValue)
            }
        })),
        "if" => Some(Function::new(|argument| {
            let mut arguments = argument.as_fixed_len_tuple(3)?;
            let result_index = if arguments[0].as_boolean()? { 1 } else { 2 };
            Ok(arguments.swap_remove(result_index))
        })),
        "len" => Some(Function::new(|argument| {
            if let Ok(subject) = argument.as_string() {
                Ok(Value::int(IntType::from_usize_lossy(subject.len())))
            } else if let Ok(subject) = argument.as_tuple() {
                Ok(Value::int(IntType::from_usize_lossy(subject.len())))
            } else {
                Err(EvalexprError::type_error(
                    argument.clone(),
                    vec![ValueType::String, ValueType::Tuple],
                ))
            }
        })),
        // String functions
        #[cfg(feature = "regex_support")]
        "str::regex_matches" => Some(Function::new(|argument| {
            let arguments = argument.as_tuple()?;

            let subject = arguments[0].as_string()?;
            let re_str = arguments[1].as_string()?;
            match Regex::new(&re_str) {
                Ok(re) => Ok(Value::Boolean(re.is_match(&subject))),
                Err(err) => Err(EvalexprError::invalid_regex(
                    re_str.to_string(),
                    format!("{}", err),
                )),
            }
        })),
        #[cfg(feature = "regex_support")]
        "str::regex_replace" => Some(Function::new(|argument| {
            let arguments = argument.as_tuple()?;

            let subject = arguments[0].as_string()?;
            let re_str = arguments[1].as_string()?;
            let repl = arguments[2].as_string()?;
            match Regex::new(&re_str) {
                Ok(re) => Ok(Value::String(
                    re.replace_all(&subject, repl.as_str()).to_string(),
                )),
                Err(err) => Err(EvalexprError::invalid_regex(
                    re_str.to_string(),
                    format!("{}", err),
                )),
            }
        })),
        "str::to_lowercase" => Some(Function::new(|argument| {
            let subject = argument.as_string()?;
            Ok(Value::from(subject.to_lowercase()))
        })),
        "str::to_uppercase" => Some(Function::new(|argument| {
            let subject = argument.as_string()?;
            Ok(Value::from(subject.to_uppercase()))
        })),
        "str::trim" => Some(Function::new(|argument| {
            let subject = argument.as_string()?;
            Ok(Value::from(subject.trim()))
        })),
        "str::from" => Some(Function::new(|argument| {
            Ok(Value::String(argument.to_string()))
        })),
        #[cfg(feature = "rand")]
        "random" => Some(Function::new(|argument| {
            argument.as_empty()?;
            let min_value = FloatType::min_value().ok_or(EvalexprError::NoMinValue)?;
            let max_value = FloatType::max_value().ok_or(EvalexprError::NoMaxValue)?;
            let uniform = rand::distributions::Uniform::new_inclusive(min_value, max_value);
            Ok(Value::Float(rand::distributions::Distribution::sample(&uniform, &mut rand::thread_rng())))
        })),
        // Bitwise operators
        "bitand" => int_function!(bitand, 2),
        "bitor" => int_function!(bitor, 2),
        "bitxor" => int_function!(bitxor, 2),
        "bitnot" => int_function!(bitnot),
        "shl" => int_function!(shl, 2),
        "shr" => int_function!(shr, 2),
        _ => None,
    }
}
