use crate::{
    records::{ASTBuilder, CSTNode},
    Error, Result,
};
use std::collections::HashMap;

/// It's a node contained in the Strongly Typed Abstract Syntax Tree
/// Implement the `parse` method to express how to become a typed node
/// Semantic analysis, interpreter, LSP can start at this level
pub trait ASTNode<N>
where
    Self: Sized,
{
    /// This node will appear multiple times
    /// If the child node fails, it will be abandoned
    fn many(node: Vec<N>, builder: &mut ASTBuilder) -> Vec<Self> {
        let mut out = Vec::with_capacity(node.len());
        for pair in node {
            match Self::parse(pair, builder) {
                Ok(o) => out.push(o),
                Err(Error::Unwinding) => (),
                Err(e) => builder.error.push(e),
            }
        }
        return out;
    }
    /// This node is optional
    /// If the parsing fails, it will be abandoned and return null
    fn some(node: N, builder: &mut ASTBuilder) -> Option<Self> {
        match Self::parse(node, builder) {
            Ok(o) => return Some(o),
            Err(Error::Unwinding) => (),
            Err(e) => builder.error.push(e),
        }
        return None;
    }
    /// This node is required
    /// If it fails, it will start to roll back
    /// Destruct level by level, until a node can tolerate this problem
    fn one(node: N, builder: &mut ASTBuilder) -> Result<Self> {
        match Self::parse(node, builder) {
            Ok(o) => Ok(o),
            Err(Error::Unwinding) => Err(Error::Unwinding),
            Err(e) => {
                builder.error.push(e);
                Err(Error::Unwinding)
            }
        }
    }
    /// many
    fn named_many(map: &mut HashMap<&'static str, Vec<N>>, tag: &str, builder: &mut ASTBuilder) -> Vec<Self> {
        match map.remove(tag) {
            Some(s) => Self::many(s, builder),
            _ => vec![],
        }
    }
    /// some
    fn named_some(map: &mut HashMap<&'static str, Vec<N>>, tag: &str, builder: &mut ASTBuilder) -> Option<Self> {
        match map.remove(tag).as_mut().map(|v| v.remove(0)) {
            Some(s) => Self::some(s, builder),
            _ => None,
        }
    }
    /// one
    fn named_one(map: &mut HashMap<&'static str, Vec<N>>, tag: &str, builder: &mut ASTBuilder) -> Result<Self> {
        match map.remove(tag).as_mut().map(|v| v.remove(0)) {
            Some(s) => Self::one(s, builder),
            _ => Err(Error::node_tag_missing(tag)),
        }
    }
    /// parse
    fn parse(pairs: N, builder: &mut ASTBuilder) -> Result<Self>;
}

macro_rules! ast_node_num {
    ($t:ty) => {
        impl<R> ASTNode<CSTNode<R>> for $t {
            fn parse(node: CSTNode<R>, builder: &mut ASTBuilder) -> Result<Self> {
                Ok(node.get_string(&builder.input).parse::<$t>()?)
            }
        }
    };
    ($($t:ty),+ $(,)?) => {
        $(ast_node_num!($t);)+
    };
}

ast_node_num![u8, u16, u32, u64, u128, usize];
ast_node_num![i8, i16, i32, i64, i128, isize];
ast_node_num![f32, f64];

impl<R> ASTNode<CSTNode<R>> for String {
    fn parse(node: CSTNode<R>, builder: &mut ASTBuilder) -> Result<Self> {
        Ok(node.get_string(&builder.input).to_string())
    }
}
