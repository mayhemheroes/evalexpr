use crate::{interface::build_operator_tree, Node};
use serde::{de, Deserialize, Deserializer};
use std::fmt;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::str::FromStr;

impl<'de, IntType: Debug + Display + FromStr+Clone+PartialEq, FloatType: Debug + Display + FromStr+Clone+PartialEq> Deserialize<'de> for Node<IntType, FloatType> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(NodeVisitor::default())
    }
}

struct NodeVisitor<IntType, FloatType> {
    int_type: PhantomData<IntType>,
    float_type: PhantomData<FloatType>,
}

impl<IntType, FloatType> Default for NodeVisitor<IntType, FloatType> {
    fn default() -> Self {
        Self {int_type:Default::default(), float_type: Default::default()}
    }
}

impl<'de, IntType: Debug + Display + FromStr+Clone+PartialEq, FloatType: Debug + Display + FromStr+Clone+PartialEq> de::Visitor<'de> for NodeVisitor<IntType, FloatType> {
    type Value = Node<IntType, FloatType>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "a string in the expression format of the `evalexpr` crate"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match build_operator_tree(v) {
            Ok(node) => Ok(node),
            Err(error) => Err(E::custom(error)),
        }
    }
}
