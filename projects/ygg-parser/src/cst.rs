use std::hash::Hash;
use yggdrasil_rt::{CstContext, CstNode, LanguageID, LanguageManager, NodeID, NodeType, ParseResult, ParseState};

#[repr(i16)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum YggdrasilType {
    Program = 0,
    Statement = 1,
    Expression = 2,
    Identifier = 3,
    Namespace = 4,
    Error = -1,
    Ignored = -2,
    IgnoredText = -3,
}

impl From<i16> for YggdrasilType {
    fn from(value: i16) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl Into<i16> for YggdrasilType {
    fn into(self) -> i16 {
        self as i16
    }
}

impl NodeType for YggdrasilType {
    fn get_language_id(&self) -> LanguageID {
        LanguageManager::id_from_name("Yggdrasil")
    }

    fn is_ignored(&self) -> bool {
        matches!(self, YggdrasilType::Ignored | YggdrasilType::IgnoredText)
    }
}

pub type ParseContext = CstContext<YggdrasilType>;

pub fn parse_namespace<'i>(i: ParseState<'i>, ctx: &mut ParseContext) -> ParseResult<'i, NodeID> {
    let o = i;
    let (o, n) = o.match_fn(|i1| consume_namespace(i1, ctx))?;
    let r = ctx.add_root(n);
    o.finish(r)
}

#[inline]
pub fn consume_namespace<'i>(i: ParseState<'i>, ctx: &mut ParseContext) -> ParseResult<'i, CstNode> {
    ctx.random_scope();
    let o = i;
    let (o, n) = o.match_fn(|i1| consume_identifier(i1, ctx))?;
    ctx.add_node(n);
    let (o, _) = o.match_repeats(|i1| consume_namespace_aux1(i1, ctx))?;
    let r = ctx.end_scope().with_kind(YggdrasilType::Namespace).with_range(i.start_offset, o.start_offset);
    o.finish(r)
}

/// `a ( :: b)*`
#[inline]
pub fn consume_namespace_aux1<'i>(i: ParseState<'i>, ctx: &mut ParseContext) -> ParseResult<'i, ()> {
    let o = i;
    let (o, n) = o.match_optional(|i1| consume_ignored(i1, ctx))?;
    ctx.add_option(n);
    let (o, n) = o.match_fn(|i1| consume_str_static(i1, ctx, "::", false))?;
    ctx.add_node(n);
    let (o, n) = o.match_optional(|i1| consume_ignored(i1, ctx))?;
    ctx.add_option(n);
    let (o, n) = o.match_fn(|i1| consume_identifier(i1, ctx))?;
    ctx.add_node(n);
    o.finish(())
}

#[inline]
pub fn consume_ignored<'i>(i: ParseState<'i>, ctx: &mut ParseContext) -> ParseResult<'i, CstNode> {
    let this = ctx.random_id();
    let (o, _) = i.match_char_if(|c| c.is_whitespace(), "IGNORED")?;
    let node = CstNode::new(this).with_kind(YggdrasilType::Ignored);
    o.finish(node)
}

#[inline]
#[rustfmt::skip]
pub fn consume_str_static<'i>(i: ParseState<'i>, ctx: &mut ParseContext, s: &'static str, insensitive: bool) -> ParseResult<'i, CstNode> {
    let this = ctx.random_id();
    let (o, _) = i.match_str_static(s, insensitive)?;
    let node = CstNode::new(this).with_kind(YggdrasilType::Ignored);
    o.finish(node)
}

#[inline]
pub fn consume_identifier<'i>(i: ParseState<'i>, ctx: &mut ParseContext) -> ParseResult<'i, CstNode> {
    let id = ctx.random_id();
    let o = i;
    let (o, _) = o.match_str_if(|c| c.is_alphabetic(), "IDENTIFIER")?;
    o.finish(CstNode::new(id).with_kind(YggdrasilType::Identifier).with_range(i.start_offset, o.start_offset))
}