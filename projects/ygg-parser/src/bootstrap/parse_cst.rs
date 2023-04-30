use super::*;

pub(super) fn parse_cst(input: &str, rule: BootstrapRule) -> OutputResult<BootstrapRule> {
    state(input, |state| match rule {
        BootstrapRule::Root => parse_root(state),
        BootstrapRule::Statement => parse_statement(state),
        BootstrapRule::GrammarStatements => parse_grammar_statements(state),
        BootstrapRule::GrammarBlock => parse_grammar_block(state),
        BootstrapRule::ClassStatements => parse_class_statements(state),
        BootstrapRule::ClassBlock => parse_class_block(state),
        BootstrapRule::UnionStatements => parse_union_statements(state),
        BootstrapRule::UnionBlock => parse_union_block(state),
        BootstrapRule::UnionBranch => parse_union_branch(state),
        BootstrapRule::BranchTag => parse_branch_tag(state),
        BootstrapRule::RightAssociativity => parse_right_associativity(state),
        BootstrapRule::GroupStatements => parse_group_statements(state),
        BootstrapRule::GroupBlock => parse_group_block(state),
        BootstrapRule::GroupPair => parse_group_pair(state),
        BootstrapRule::Expression => parse_expression(state),
        BootstrapRule::Term => parse_term(state),
        BootstrapRule::Infix => parse_infix(state),
        BootstrapRule::Prefix => parse_prefix(state),
        BootstrapRule::Suffix => parse_suffix(state),
        BootstrapRule::Atomic => parse_atomic(state),
        BootstrapRule::String => parse_string(state),
        BootstrapRule::Regex => parse_regex(state),
        BootstrapRule::NamepathFree => parse_namepath_free(state),
        BootstrapRule::Namepath => parse_namepath(state),
        BootstrapRule::Identifier => parse_identifier(state),
        BootstrapRule::Boolean => parse_boolean(state),
        BootstrapRule::Modifiers => parse_modifiers(state),
        BootstrapRule::KW_GRAMMAR => parse_kw_grammar(state),
        BootstrapRule::KW_IMPORT => parse_kw_import(state),
        BootstrapRule::KW_CLASS => parse_kw_class(state),
        BootstrapRule::KW_UNION => parse_kw_union(state),
        BootstrapRule::KW_GROUP => parse_kw_group(state),
        BootstrapRule::KW_CLIMB => parse_kw_climb(state),
        BootstrapRule::KW_MACRO => parse_kw_macro(state),
        BootstrapRule::WhiteSpace => parse_white_space(state),
        BootstrapRule::Comment => parse_comment(state),
        BootstrapRule::IgnoreText => unreachable!(),
        BootstrapRule::IgnoreRegex => unreachable!(),
    })
}
#[inline]
fn parse_root(state: Input) -> Output {
    state.rule(BootstrapRule::Root, |s| {
        s.sequence(|s| {
            Ok(s).and_then(|s| parse_statement(s).and_then(|s| s.tag_node("statement"))).and_then(|s| {
                s.repeat(0..4294967295, |s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| builtin_ignore(s))
                            .and_then(|s| parse_statement(s).and_then(|s| s.tag_node("statement")))
                    })
                })
            })
        })
    })
}
#[inline]
fn parse_statement(state: Input) -> Output {
    state.rule(BootstrapRule::Statement, |s| {
        Err(s)
            .or_else(|s| parse_grammar_statements(s).and_then(|s| s.tag_node("grammar_statements")))
            .or_else(|s| parse_class_statements(s).and_then(|s| s.tag_node("class_statements")))
            .or_else(|s| parse_union_statements(s).and_then(|s| s.tag_node("union_statements")))
            .or_else(|s| parse_group_statements(s).and_then(|s| s.tag_node("group_statements")))
    })
}
#[inline]
fn parse_grammar_statements(state: Input) -> Output {
    state.rule(BootstrapRule::GrammarStatements, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_kw_grammar(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_grammar_block(s).and_then(|s| s.tag_node("grammar_block")))
        })
    })
}
#[inline]
fn parse_grammar_block(state: Input) -> Output {
    state.rule(BootstrapRule::GrammarBlock, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_class_statements(state: Input) -> Output {
    state.rule(BootstrapRule::ClassStatements, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_modifiers(s).and_then(|s| s.tag_node("modifiers")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_kw_class(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_class_block(s).and_then(|s| s.tag_node("class_block")))
        })
    })
}
#[inline]
fn parse_class_block(state: Input) -> Output {
    state.rule(BootstrapRule::ClassBlock, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_expression(s).and_then(|s| s.tag_node("expression"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_union_statements(state: Input) -> Output {
    state.rule(BootstrapRule::UnionStatements, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_modifiers(s).and_then(|s| s.tag_node("modifiers")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_kw_union(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_union_block(s).and_then(|s| s.tag_node("union_block")))
        })
    })
}
#[inline]
fn parse_union_block(state: Input) -> Output {
    state.rule(BootstrapRule::UnionBlock, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_union_branch(s).and_then(|s| s.tag_node("union_branch")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_union_branch(s).and_then(|s| s.tag_node("union_branch")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_union_branch(state: Input) -> Output {
    state.rule(BootstrapRule::UnionBranch, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "|", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_expression(s).and_then(|s| s.tag_node("expression")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_branch_tag(s).and_then(|s| s.tag_node("branch_tag"))))
        })
    })
}

#[inline]
fn parse_branch_tag(state: Input) -> Output {
    state.rule(BootstrapRule::BranchTag, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "#", false))
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| s.optional(|s| parse_right_associativity(s).and_then(|s| s.tag_node("right_associativity"))))
        })
    })
}

#[inline]
fn parse_right_associativity(state: Input) -> Output {
    state.rule(BootstrapRule::RightAssociativity, |s| s.match_string(">", false))
}
#[inline]
fn parse_group_statements(state: Input) -> Output {
    state.rule(BootstrapRule::GroupStatements, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_modifiers(s).and_then(|s| s.tag_node("modifiers")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_kw_group(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.optional(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_group_block(s).and_then(|s| s.tag_node("group_block")))
        })
    })
}
#[inline]
fn parse_group_block(state: Input) -> Output {
    state.rule(BootstrapRule::GroupBlock, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_group_pair(s).and_then(|s| s.tag_node("group_pair"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_group_pair(state: Input) -> Output {
    state.rule(BootstrapRule::GroupPair, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, ":", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_atomic(s).and_then(|s| s.tag_node("atomic")))
        })
    })
}
#[inline]
fn parse_expression(state: Input) -> Output {
    state.rule(BootstrapRule::Expression, |s| {
        s.sequence(|s| {
            Ok(s).and_then(|s| parse_term(s).and_then(|s| s.tag_node("term"))).and_then(|s| {
                s.repeat(0..4294967295, |s| {
                    s.sequence(|s| {
                        Ok(s)
                            .and_then(|s| parse_infix(s).and_then(|s| s.tag_node("infix")))
                            .and_then(|s| parse_term(s).and_then(|s| s.tag_node("term")))
                    })
                })
            })
        })
    })
}
#[inline]
fn parse_term(state: Input) -> Output {
    state.rule(BootstrapRule::Term, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| s.repeat(0..4294967295, |s| parse_prefix(s).and_then(|s| s.tag_node("prefix"))))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_atomic(s).and_then(|s| s.tag_node("atomic")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_suffix(s).and_then(|s| s.tag_node("suffix"))))
        })
    })
}
#[inline]
fn parse_infix(state: Input) -> Output {
    state.rule(BootstrapRule::Infix, |s| {
        Err(s).or_else(|s| builtin_ignore(s).and_then(|s| s.tag_node("infix_0"))).or_else(|s| {
            s.sequence(|s| {
                Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| builtin_text(s, "~", false)).and_then(|s| builtin_ignore(s))
            })
            .and_then(|s| s.tag_node("infix_1"))
        })
    })
}
#[inline]
fn parse_prefix(state: Input) -> Output {
    state.rule(BootstrapRule::Prefix, |s| {
        Err(s)
            .or_else(|s| builtin_text(s, "!", false).and_then(|s| s.tag_node("prefix_0")))
            .or_else(|s| builtin_text(s, "&", false).and_then(|s| s.tag_node("prefix_1")))
            .or_else(|s| builtin_text(s, "^", false).and_then(|s| s.tag_node("prefix_2")))
    })
}
#[inline]
fn parse_suffix(state: Input) -> Output {
    state.rule(BootstrapRule::Suffix, |s| {
        Err(s)
            .or_else(|s| builtin_text(s, "?", false).and_then(|s| s.tag_node("suffix_0")))
            .or_else(|s| builtin_text(s, "*", false).and_then(|s| s.tag_node("suffix_1")))
            .or_else(|s| builtin_text(s, "+", false).and_then(|s| s.tag_node("suffix_2")))
    })
}
#[inline]
fn parse_atomic(state: Input) -> Output {
    state.rule(BootstrapRule::Atomic, |s| {
        Err(s)
            .or_else(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
            .or_else(|s| parse_boolean(s).and_then(|s| s.tag_node("boolean")))
    })
}
#[inline]
fn parse_string(state: Input) -> Output {
    state.rule(BootstrapRule::String, |s| {
        Err(s)
            .or_else(|s| {
                s.sequence(|s| Ok(s).and_then(|s| builtin_text(s, "'", false)).and_then(|s| builtin_text(s, "'", false)))
                    .and_then(|s| s.tag_node("string_0"))
            })
            .or_else(|s| {
                s.sequence(|s| Ok(s).and_then(|s| builtin_text(s, "\"", false)).and_then(|s| builtin_text(s, "\"", false)))
                    .and_then(|s| s.tag_node("string_1"))
            })
    })
}
#[inline]
fn parse_regex(state: Input) -> Output {
    state.rule(BootstrapRule::Regex, |s| s.match_string("/", false))
}
#[inline]
fn parse_namepath_free(state: Input) -> Output {
    state.rule(BootstrapRule::NamepathFree, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| {
                                    Err(s).or_else(|s| builtin_text(s, ".", false)).or_else(|s| builtin_text(s, "::", false))
                                })
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                        })
                    })
                })
        })
    })
}
#[inline]
fn parse_namepath(state: Input) -> Output {
    state.rule(BootstrapRule::Namepath, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_text(s, "::", false))
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                        })
                    })
                })
        })
    })
}
#[inline]
fn parse_identifier(state: Input) -> Output {
    state.rule(BootstrapRule::Identifier, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^([_\\p{XID_start}]\\p{XID_continue}*)").unwrap())
        })
    })
}
#[inline]
fn parse_boolean(state: Input) -> Output {
    state.rule(BootstrapRule::Boolean, |s| {
        Err(s)
            .or_else(|s| builtin_text(s, "true", false).and_then(|s| s.tag_node("boolean_0")))
            .or_else(|s| builtin_text(s, "false", false).and_then(|s| s.tag_node("boolean_1")))
    })
}

#[inline]
fn parse_modifiers(state: Input) -> Output {
    state.rule(BootstrapRule::Modifiers, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| {
                    s.lookahead(false, |s| {
                        Err(s)
                            .or_else(|s| parse_kw_class(s).and_then(|s| s.tag_node("kw_class")))
                            .or_else(|s| parse_kw_union(s).and_then(|s| s.tag_node("kw_union")))
                            .or_else(|s| parse_kw_group(s).and_then(|s| s.tag_node("kw_group")))
                            .or_else(|s| parse_kw_macro(s).and_then(|s| s.tag_node("kw_macro")))
                            .or_else(|s| parse_kw_climb(s).and_then(|s| s.tag_node("kw_climb")))
                    })
                })
                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| {
                                    s.lookahead(false, |s| {
                                        Err(s)
                                            .or_else(|s| parse_kw_class(s).and_then(|s| s.tag_node("kw_class")))
                                            .or_else(|s| parse_kw_union(s).and_then(|s| s.tag_node("kw_union")))
                                            .or_else(|s| parse_kw_group(s).and_then(|s| s.tag_node("kw_group")))
                                            .or_else(|s| parse_kw_macro(s).and_then(|s| s.tag_node("kw_macro")))
                                            .or_else(|s| parse_kw_climb(s).and_then(|s| s.tag_node("kw_climb")))
                                    })
                                })
                                .and_then(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
                        })
                    })
                })
        })
    })
}

#[inline]
fn parse_kw_grammar(state: Input) -> Output {
    state.rule(BootstrapRule::KW_GRAMMAR, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(grammar)").unwrap())
        })
    })
}

#[inline]
fn parse_kw_import(state: Input) -> Output {
    state.rule(BootstrapRule::KW_IMPORT, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(using|import|use)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_class(state: Input) -> Output {
    state.rule(BootstrapRule::KW_CLASS, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(class|struct)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_union(state: Input) -> Output {
    state.rule(BootstrapRule::KW_UNION, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(union|enum)").unwrap())
        })
    })
}
#[inline]
fn parse_kw_group(state: Input) -> Output {
    state.rule(BootstrapRule::KW_GROUP, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(group|token)").unwrap())
        })
    })
}

#[inline]
fn parse_kw_climb(state: Input) -> Output {
    state.rule(BootstrapRule::KW_CLIMB, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(climb)").unwrap())
        })
    })
}

#[inline]
fn parse_kw_macro(state: Input) -> Output {
    state.rule(BootstrapRule::KW_MACRO, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(macro|def|function|func|fun|fn)").unwrap())
        })
    })
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(BootstrapRule::WhiteSpace, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(\\p{White_Space}+)").unwrap())
        })
    })
}
#[inline]
fn parse_comment(state: Input) -> Output {
    state.rule(BootstrapRule::Comment, |s| {
        Err(s)
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s).and_then(|s| builtin_text(s, "//", false)).and_then(|s| {
                        builtin_regex(s, {
                            static REGEX: OnceLock<Regex> = OnceLock::new();
                            REGEX.get_or_init(|| Regex::new("^([^\\n\\r]*)").unwrap())
                        })
                    })
                })
                .and_then(|s| s.tag_node("comment_0"))
            })
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s).and_then(|s| builtin_text(s, "#", false)).and_then(|s| {
                        builtin_regex(s, {
                            static REGEX: OnceLock<Regex> = OnceLock::new();
                            REGEX.get_or_init(|| Regex::new("^([^\\n\\r]*)").unwrap())
                        })
                    })
                })
                .and_then(|s| s.tag_node("comment_1"))
            })
    })
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| parse_white_space(s).or_else(|s| parse_comment(s)))
}

fn builtin_any(state: Input) -> Output {
    state.rule(BootstrapRule::IgnoreText, |s| s.match_char_if(|_| true))
}

fn builtin_text<'i>(state: Input<'i>, text: &'static str, case: bool) -> Output<'i> {
    state.rule(BootstrapRule::IgnoreText, |s| s.match_string(text, case))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(BootstrapRule::IgnoreRegex, |s| s.match_regex(regex))
}