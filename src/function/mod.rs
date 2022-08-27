use std::fmt;

use crate::{error::EvalexprResult, value::Value};

pub(crate) mod builtin;

/// A helper trait to enable cloning through `Fn` trait objects.
trait ClonableFn<IntType, FloatType>
where
    Self: Fn(&Value<IntType, FloatType>) -> EvalexprResult<Value<IntType, FloatType>, IntType, FloatType>,
    Self: Send + Sync + 'static,
{
    fn dyn_clone(&self) -> Box<dyn ClonableFn<IntType, FloatType>>;
}

impl<F, IntType, FloatType> ClonableFn<IntType, FloatType> for F
where
    F: Fn(&Value<IntType, FloatType>) -> EvalexprResult<Value<IntType, FloatType>, IntType, FloatType>,
    F: Send + Sync + 'static,
    F: Clone,
{
    fn dyn_clone(&self) -> Box<dyn ClonableFn<IntType, FloatType>> {
        Box::new(self.clone()) as _
    }
}

/// A user-defined function.
/// Functions can be used in expressions by storing them in a `Context`.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let mut context = HashMapContext::new();
/// context.set_function("id".into(), Function::new(|argument| {
///     Ok(argument.clone())
/// })).unwrap(); // Do proper error handling here
/// assert_eq!(eval_with_context("id(4)", &context), Ok(Value::from(4)));
/// ```
pub struct Function<IntType = i64, FloatType = f64> {
    function: Box<dyn ClonableFn<IntType, FloatType>>,
}

impl<IntType: 'static, FloatType: 'static> Clone for Function<IntType, FloatType> {
    fn clone(&self) -> Self {
        Self {
            function: self.function.dyn_clone(),
        }
    }
}

impl<IntType, FloatType> Function<IntType, FloatType> {
    /// Creates a user-defined function.
    ///
    /// The `function` is boxed for storage.
    pub fn new<F>(function: F) -> Self
    where
        F: Fn(&Value<IntType, FloatType>) -> EvalexprResult<Value<IntType, FloatType>, IntType, FloatType>,
        F: Send + Sync + 'static,
        F: Clone,
    {
        Self {
            function: Box::new(function) as _,
        }
    }

    pub(crate) fn call(&self, argument: &Value<IntType, FloatType>) -> EvalexprResult<Value<IntType, FloatType>, IntType, FloatType> {
        (self.function)(argument)
    }
}

impl<IntType, FloatType> fmt::Debug for Function<IntType, FloatType> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Function {{ [...] }}")
    }
}

/// A trait to ensure a type is `Send` and `Sync`.
/// If implemented for a type, the crate will not compile if the type is not `Send` and `Sync`.
trait IsSendAndSync: Send + Sync {}

impl IsSendAndSync for Function {}
