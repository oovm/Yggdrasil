use super::*;

#[inline]
pub fn program(s: RuleState) -> RuleResult {
    s.rule(Rule::program, |s| {
        s.sequence(|s| {
            self::SOI(s)
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.sequence(|s| s.optional(|s| tag_node!(s, statement, "statement").and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(tag_node!(statement, "statement"))))))))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| self::EOI(s))
        })
    })
}

#[inline]
#[rustfmt::skip]
pub fn statement(s: RuleState) -> RuleResult {
    tag_branch!(s, statement, "MacroCall", tag_node!(macro_call, "macro_call"));
    tag_branch!(s, statement, "MacroDefine", tag_node!(macro_define, "macro_define"));
    tag_branch!(s, statement, "CommentDoc", tag_node!(comment_doc, "comment_doc"));
    tag_branch!(s, statement, "Grammar", |s| s.sequence(|s| tag_node!(s,grammar_statement, "grammar_statement").and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| self::eos(s)))));
    tag_branch!(s, statement, "Fragment", |s| s.sequence(|s| tag_node!(s,fragment_statement, "fragment_statement").and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| self::eos(s)))));
    tag_branch!(s, statement, "Ignore", |s| s.sequence(|s| tag_node!(s,ignore_statement, "ignore_statement").and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| self::eos(s)))));
    tag_branch!(s, statement, "Assign", |s| s.sequence(|s| tag_node!(s,assign_statement, "assign_statement").and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| self::eos(s)))));
    tag_branch!(s, statement, "Empty", tag_node!(empty_statement, "empty_statement"));
    return Err(s);
}

#[inline]
pub fn empty_statement(s: RuleState) -> RuleResult {
    s.rule(Rule::empty_statement, |s| self::eos(s))
}

#[inline]
pub fn eos(s: RuleState) -> RuleResult {
    s.rule(Rule::eos, |s| s.atomic(Atomicity::Atomic, |s| s.match_string(";")))
}

#[inline]
pub fn grammar_statement(s: RuleState) -> RuleResult {
    s.rule(Rule::grammar_statement, |s| {
        s.sequence(|s| {
            self::grammar(s).and_then(|s| self::SKIP(s)).and_then(|s| self::symbol(s)).and_then(|s| self::SKIP(s)).and_then(|s| {
                self::string(s).or_else(|s| {
                    s.sequence(|s| {
                        s.match_string("{")
                            .and_then(|s| self::SKIP(s))
                            .and_then(|s| {
                                s.optional(|s| {
                                    s.sequence(|s| {
                                        self::string(s)
                                            .and_then(|s| self::SKIP(s))
                                            .and_then(|s| {
                                                s.sequence(|s| {
                                                    s.optional(|s| {
                                                        s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::string(s)))
                                                            .and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(|s| s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::string(s)))))))
                                                    })
                                                })
                                            })
                                            .and_then(|s| self::SKIP(s))
                                            .and_then(|s| s.optional(|s| s.match_string(",")))
                                    })
                                })
                            })
                            .and_then(|s| self::SKIP(s))
                            .and_then(|s| s.match_string("}"))
                    })
                })
            })
        })
    })
}

#[inline]
pub fn grammar(s: RuleState) -> RuleResult {
    s.match_string("grammar!")
}

#[inline]
pub fn fragment_statement(s: RuleState) -> RuleResult {
    s.rule(Rule::fragment_statement, |s| s.sequence(|s| self::fragment(s).and_then(|s| self::SKIP(s)).and_then(tag_node!(symbol, "id"))))
}

#[inline]
pub fn fragment(s: RuleState) -> RuleResult {
    s.match_string("fragment!")
}

#[inline]
pub fn import_statement(s: RuleState) -> RuleResult {
    s.rule(Rule::import_statement, |s| {
        s.sequence(|s| {
            self::UNNAMED(s, "import!").and_then(|s| self::SKIP(s)).and_then(|s| {
                self::string(s).or_else(|s| {
                    s.sequence(|s| {
                        self::string(s)
                            .and_then(|s| self::SKIP(s))
                            .and_then(|s| s.match_string("{"))
                            .and_then(|s| self::SKIP(s))
                            .and_then(|s| {
                                s.optional(|s| {
                                    s.sequence(|s| {
                                        self::symbol_alias(s)
                                            .and_then(|s| self::SKIP(s))
                                            .and_then(|s| {
                                                s.sequence(|s| {
                                                    s.optional(|s| {
                                                        s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::symbol_alias(s)))
                                                            .and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(|s| s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::symbol_alias(s)))))))
                                                    })
                                                })
                                            })
                                            .and_then(|s| self::SKIP(s))
                                            .and_then(|s| s.optional(|s| s.match_string(",")))
                                    })
                                })
                            })
                            .and_then(|s| self::SKIP(s))
                            .and_then(|s| s.match_string("}"))
                    })
                })
            })
        })
    })
}

#[inline]
pub fn ignore_statement(s: RuleState) -> RuleResult {
    s.rule(Rule::ignore_statement, |s| {
        s.sequence(|s| {
            self::UNNAMED(s, "ignore!").and_then(|s| self::SKIP(s)).and_then(|s| {
                self::symbol(s).or_else(|s| {
                    s.sequence(|s| {
                        s.match_string("{")
                            .and_then(|s| self::SKIP(s))
                            .and_then(|s| {
                                s.optional(|s| {
                                    s.sequence(|s| {
                                        self::symbol(s)
                                            .and_then(|s| self::SKIP(s))
                                            .and_then(|s| {
                                                s.sequence(|s| {
                                                    s.optional(|s| {
                                                        s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::symbol(s)))
                                                            .and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(|s| s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::symbol(s)))))))
                                                    })
                                                })
                                            })
                                            .and_then(|s| self::SKIP(s))
                                            .and_then(|s| s.optional(|s| s.match_string(",")))
                                    })
                                })
                            })
                            .and_then(|s| self::SKIP(s))
                            .and_then(|s| s.match_string("}"))
                    })
                })
            })
        })
    })
}

#[inline]
pub fn assign_statement(s: RuleState) -> RuleResult {
    s.rule(Rule::assign_statement, |s| {
        s.sequence(|s| {
            tag_node!(s, symbol, "id")
                .and_then(|s| self::SKIP(s))
                .and_then(tag_node!(assign_kind, "eq"))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.optional(|s| s.match_string("|")))
                .and_then(|s| self::SKIP(s))
                .and_then(tag_node!(expr, "rhs"))
        })
    })
}

#[inline]
pub fn assign_kind(s: RuleState) -> RuleResult {
    s.rule(Rule::assign_kind, |s| {
        s.atomic(Atomicity::Atomic, |s| s.sequence(|s| s.repeat(|s| s.match_string("^").or_else(|s| s.match_string("_")).or_else(|s| s.match_string("@"))).and_then(|s| s.match_string("="))))
    })
}

#[inline]
pub fn expr(s: RuleState) -> RuleResult {
    s.rule(Rule::expr, |s| {
        s.sequence(|s| {
            tag_node!(s,__rec_expr_left,"__rec_expr_left")
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.sequence(|s| s.optional(|s| tag_node!(s,__rec_expr_rest,"__rec_expr_rest").and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(tag_node!(__rec_expr_rest,"__rec_expr_rest"))))))))
        })
    })
}

#[inline]
pub fn __rec_expr_left(s: RuleState) -> RuleResult {
    tag_branch!(s, BRANCH, "Priority", self::__aux_expr_priority);
    tag_branch!(s, BRANCH, "Mark", self::__aux_expr_mark);
    tag_branch!(s, BRANCH, "Prefix", self::__aux_expr_prefix);
    tag_branch!(s, BRANCH, "Data", tag_node!(data, "data"));
    return Err(s);
}

pub fn __aux_expr_priority(s: RuleState) -> RuleResult {
    s.sequence(|s| {
        s.match_string("(")
            .and_then(|s| self::SKIP(s))
            .and_then(|s| s.optional(|s| s.match_string("|")))
            .and_then(|s| self::SKIP(s))
            .and_then(|s| self::expr(s))
            .and_then(|s| self::SKIP(s))
            .and_then(|s| s.match_string(")"))
    })
}

pub fn __aux_expr_mark(s: RuleState) -> RuleResult {
    s.sequence(|s| {
        self::symbol(s)
            .and_then(|s| self::SKIP(s))
            .and_then(|s| s.optional(|s| self::mark_type(s)))
            .and_then(|s| self::SKIP(s))
            .and_then(|s| s.match_string("<-"))
            .and_then(|s| self::SKIP(s))
            .and_then(|s| self::expr(s))
    })
}

pub fn __aux_expr_prefix(s: RuleState) -> RuleResult {
    s.sequence(|s| self::prefix(s).and_then(|s| self::SKIP(s)).and_then(|s| self::expr(s)))
}

#[inline]
pub fn __rec_expr_rest(s: RuleState) -> RuleResult {
    tag_branch!(s, BRANCH, "Choice", self::__aux_expr_choice);
    tag_branch!(s, BRANCH, "Concat", self::__aux_expr_concat);
    tag_branch!(s, BRANCH, "Slice", self::__aux_expr_slice);
    tag_branch!(s, BRANCH, "Suffix", self::__aux_expr_suffix);

    s.rule(Rule::__rec_expr_rest, |s| {
        s.sequence(|s| s.match_string("~").and_then(|s| self::SKIP(s)).and_then(|s| self::expr(s)))
            .or_else(|s| {
                s.sequence(|s| {
                    s.optional(|s| s.sequence(|s| self::mark_branch(s).and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| self::mark_type(s)))))
                        .and_then(|s| self::SKIP(s))
                        .and_then(|s| s.match_string("|"))
                        .and_then(|s| self::SKIP(s))
                        .and_then(|s| self::expr(s))
                        .and_then(|s| self::SKIP(s))
                        .and_then(|s| s.optional(|s| s.sequence(|s| self::mark_branch(s).and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| self::mark_type(s))))))
                })
            })
            .or_else(|s| self::slice(s))
            .or_else(|s| self::suffix(s))
    })
}

#[inline]
pub fn mark_branch(s: RuleState) -> RuleResult {
    s.atomic(Atomicity::CompoundAtomic, |s| {
        s.rule(Rule::mark_branch, |s| s.sequence(|s| s.optional(|s| s.match_string("^").or_else(|s| s.match_string("!"))).and_then(|s| s.match_string("#")).and_then(|s| self::symbol(s))))
    })
}

#[inline]
pub fn mark_type(s: RuleState) -> RuleResult {
    s.rule(Rule::mark_type, |s| s.sequence(|s| s.match_string(":").and_then(|s| self::SKIP(s)).and_then(|s| self::symbol(s))))
}

#[inline]
pub fn prefix(s: RuleState) -> RuleResult {
    s.rule(Rule::prefix, |s| s.atomic(Atomicity::Atomic, match_charset!('!' | '&' | '^' | '*' | '%')))
}

#[inline]
pub fn suffix(s: RuleState) -> RuleResult {
    s.rule(Rule::suffix, |s| s.atomic(Atomicity::Atomic, match_charset!('?' | '+' | '*')))
}

#[inline]
#[rustfmt::skip]
pub fn data(s: RuleState) -> RuleResult {
    tag_branch!(s, BRANCH, "MacroCall", tag_node!(macro_call, "macro_call"));
    tag_branch!(s, BRANCH, "RegexRange", tag_node!(regex_range, "regex_range"));
    tag_branch!(s, BRANCH, "List", tag_node!(list, "list"));
    tag_branch!(s, BRANCH, "SymbolPath",tag_node!(symbol_path, "symbol_path"));
    tag_branch!(s, BRANCH, "Integer", tag_node!(integer, "integer"));
    return Err(s);
}

#[inline]
pub fn list(s: RuleState) -> RuleResult {
    s.rule(Rule::list, |s| {
        s.sequence(|s| {
            s.match_string("{")
                .and_then(|s| self::SKIP(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            self::data(s)
                                .and_then(|s| self::SKIP(s))
                                .and_then(|s| {
                                    s.sequence(|s| {
                                        s.optional(|s| {
                                            s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::data(s)))
                                                .and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(|s| s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::data(s)))))))
                                        })
                                    })
                                })
                                .and_then(|s| self::SKIP(s))
                                .and_then(|s| s.optional(|s| s.match_string(",")))
                        })
                    })
                })
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.match_string("}"))
        })
    })
}

#[inline]
pub fn slice(s: RuleState) -> RuleResult {
    s.rule(Rule::slice, |s| {
        s.sequence(|s| {
            s.match_string("{")
                .and_then(|s| self::SKIP(s))
                .and_then(|s| self::integer(s))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.match_string(","))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| self::integer(s))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.match_string("}"))
        })
    })
}

#[inline]
pub fn regex_range(s: RuleState) -> RuleResult {
    s.rule(Rule::regex_range, |s| {
        s.atomic(Atomicity::Atomic, |s| {
            s.sequence(|s| {
                s.match_string("[")
                    .and_then(|s| s.repeat(|s| s.sequence(|s| s.lookahead(false, |s| s.match_string("]")).and_then(|s| self::ANY(s))).or_else(|s| s.sequence(|s| s.match_string("\\").and_then(|s| self::ANY(s))))))
                    .and_then(|s| s.match_string("]"))
            })
        })
    })
}

#[inline]
pub fn macro_call(s: RuleState) -> RuleResult {
    s.rule(Rule::macro_call, |s| {
        s.sequence(|s| {
            s.match_string("@")
                .and_then(|s| self::SKIP(s))
                .and_then(|s| self::symbol_path(s))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.match_string("("))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            self::macro_kv(s)
                                .and_then(|s| self::SKIP(s))
                                .and_then(|s| {
                                    s.sequence(|s| {
                                        s.optional(|s| {
                                            s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::macro_kv(s)))
                                                .and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(|s| s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::macro_kv(s)))))))
                                        })
                                    })
                                })
                                .and_then(|s| self::SKIP(s))
                                .and_then(|s| s.optional(|s| s.match_string(",")))
                        })
                    })
                })
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.match_string(")"))
        })
    })
}

#[inline]
pub fn macro_define(s: RuleState) -> RuleResult {
    s.rule(Rule::macro_define, |s| {
        s.sequence(|s| {
            s.match_string("macro!")
                .and_then(|s| self::SKIP(s))
                .and_then(|s| self::symbol_path(s))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.match_string("("))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            self::macro_arg(s)
                                .and_then(|s| self::SKIP(s))
                                .and_then(|s| {
                                    s.sequence(|s| {
                                        s.optional(|s| {
                                            s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::macro_arg(s)))
                                                .and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(|s| s.sequence(|s| s.match_string(",").and_then(|s| self::SKIP(s)).and_then(|s| self::macro_arg(s)))))))
                                        })
                                    })
                                })
                                .and_then(|s| self::SKIP(s))
                                .and_then(|s| s.optional(|s| s.match_string(",")))
                        })
                    })
                })
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.match_string(")"))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| self::block(s))
        })
    })
}

#[inline]
pub fn macro_kv(s: RuleState) -> RuleResult {
    s.rule(Rule::macro_kv, |s| {
        s.sequence(|s| self::symbol(s).and_then(|s| self::SKIP(s)).and_then(|s| s.match_string("=")).and_then(|s| self::SKIP(s)).and_then(|s| self::expr(s))).or_else(|s| self::expr(s))
    })
}

#[inline]
pub fn macro_arg(s: RuleState) -> RuleResult {
    s.rule(Rule::macro_arg, |s| {
        s.sequence(|s| {
            self::symbol(s)
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.optional(|s| s.sequence(|s| s.match_string(":").and_then(|s| self::SKIP(s)).and_then(|s| self::symbol(s)))))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.optional(|s| s.sequence(|s| s.match_string("=").and_then(|s| self::SKIP(s)).and_then(|s| self::expr(s)))))
        })
    })
}

#[inline]
pub fn block(s: RuleState) -> RuleResult {
    s.rule(Rule::block, |s| {
        s.sequence(|s| {
            s.match_string("{")
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.optional(|s| s.match_string("return")))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| self::expr(s))
                .and_then(|s| self::SKIP(s))
                .and_then(|s| s.match_string("}"))
        })
    })
}

#[inline]
pub fn string(s: RuleState) -> RuleResult {
    s.rule(Rule::string, |s| {
        s.atomic(Atomicity::Atomic, |s| {
            s.sequence(|s| {
                s.match_string("'")
                    .and_then(|s| s.repeat(|s| s.sequence(|s| s.lookahead(false, |s| s.match_string("'")).and_then(|s| self::ANY(s))).or_else(|s| s.sequence(|s| s.match_string("\\").and_then(|s| self::ANY(s))))))
                    .and_then(|s| s.match_string("'"))
            })
                .or_else(|s| {
                    s.sequence(|s| {
                        s.match_string("\"")
                            .and_then(|s| s.repeat(|s| s.sequence(|s| s.lookahead(false, |s| s.match_string("\"")).and_then(|s| self::ANY(s))).or_else(|s| s.sequence(|s| s.match_string("\\").and_then(|s| self::ANY(s))))))
                            .and_then(|s| s.match_string("\""))
                    })
                })
        })
    })
}

#[inline]
pub fn integer(s: RuleState) -> RuleResult {
    s.rule(Rule::integer, |s| {
        s.atomic(Atomicity::Atomic, |s| {
            s.match_string("0")
                .or_else(|s| s.sequence(|s| self::ASCII_NONZERO_DIGIT(s).and_then(|s| s.repeat(|s| s.sequence(|s| s.optional(|s| s.match_string("_")).and_then(|s| self::ASCII_DIGIT(s)))))))
        })
    })
}

#[inline]
pub fn special(s: RuleState) -> RuleResult {
    s.rule(Rule::special, |s| s.atomic(Atomicity::Atomic, |s| s.match_string("true").or_else(|s| s.match_string("false")).or_else(|s| s.match_string("null"))))
}

#[inline]
pub fn comment_doc(s: RuleState) -> RuleResult {
    s.rule(Rule::comment_doc, |s| {
        s.atomic(Atomicity::Atomic, |s| s.sequence(|s| s.match_string("///").and_then(|s| s.repeat(|s| s.sequence(|s| s.lookahead(false, |s| self::NEWLINE(s)).and_then(|s| self::ANY(s)))))))
    })
}

#[inline]
pub fn comment_s_l(s: RuleState) -> RuleResult {
    s.rule(Rule::comment_s_l, |s| {
        s.atomic(Atomicity::Atomic, |s| s.sequence(|s| s.match_string("//").and_then(|s| s.repeat(|s| s.sequence(|s| s.lookahead(false, |s| self::NEWLINE(s)).and_then(|s| self::ANY(s)))))))
    })
}

#[inline]
pub fn comment_m_l(s: RuleState) -> RuleResult {
    s.rule(Rule::comment_m_l, |s| {
        s.atomic(Atomicity::Atomic, |s| {
            s.sequence(|s| {
                s.match_string("/*")
                    .and_then(|s| s.repeat(|s| self::comment_m_l(s).or_else(|s| s.sequence(|s| s.lookahead(false, |s| s.match_string("*/")).and_then(|s| self::ANY(s))))))
                    .and_then(|s| s.match_string("*/"))
            })
        })
    })
}

#[inline]
pub fn symbol_path(s: RuleState) -> RuleResult {
    s.rule(Rule::id_path, |s| {
        s.sequence(|s| {
            tag_node!(s, symbol, "symbol").and_then(|s| self::SKIP(s)).and_then(|s| {
                s.sequence(|s| {
                    s.optional(|s| {
                        s.sequence(|s| s.match_string("::").or_else(|s| s.match_string(".")).and_then(|s| self::SKIP(s)).and_then(tag_node!(symbol, "symbol")))
                            .and_then(|s| s.repeat(|s| s.sequence(|s| self::SKIP(s).and_then(|s| s.sequence(|s| s.match_string("::").or_else(|s| s.match_string(".")).and_then(|s| self::SKIP(s)).and_then(tag_node!(symbol, "symbol")))))))
                    })
                })
            })
        })
    })
}

#[inline]
pub fn symbol_alias(s: RuleState) -> RuleResult {
    s.rule(Rule::symbol_alias, |s| {
        s.sequence(|s| self::symbol(s).and_then(|s| self::SKIP(s)).and_then(|s| s.optional(|s| s.sequence(|s| s.match_string("as").and_then(|s| self::SKIP(s)).and_then(|s| self::symbol(s))))))
    })
}

#[inline]
pub fn symbol(s: RuleState) -> RuleResult {
    s.rule(Rule::SYMBOL, |s| s.atomic(Atomicity::Atomic, |s| s.sequence(|s| s.match_string("_").or_else(|s| self::XID_START(s)).and_then(|s| s.repeat(|s| self::XID_CONTINUE(s))))))
}

////-----------------------------------------------------------------------------------------------------

#[inline]
pub fn COMMENT(s: RuleState) -> RuleResult {
    s.rule(Rule::COMMENT, |s| s.atomic(Atomicity::Atomic, |s| self::comment_s_l(s).or_else(|s| self::comment_m_l(s))))
}

#[inline]
pub fn WHITESPACE(s: RuleState) -> RuleResult {
    s.rule(Rule::WHITESPACE, |s| {
        s.atomic(Atomicity::Atomic, |s| s.sequence(|s| s.match_string(" ").or_else(|s| s.match_string("\t")).and_then(|s| s.repeat(|s| s.match_string(" ").or_else(|s| s.match_string("\t"))))))
    })
}

#[inline]
pub fn NEWLINE(s: RuleState) -> RuleResult {
    s.rule(Rule::NEWLINE, |s| {
        s.atomic(Atomicity::Atomic, |s| {
            s.sequence(|s| {
                s.match_string("\r\n")
                    .or_else(|s| s.match_string("\r"))
                    .or_else(|s| s.match_string("\n"))
                    .and_then(|s| s.repeat(|s| s.match_string("\r\n").or_else(|s| s.match_string("\r")).or_else(|s| s.match_string("\n"))))
            })
        })
    })
}

#[inline]
pub fn ASCII_DIGIT(s: RuleState) -> RuleResult {
    s.match_range('0'..'9')
}

#[inline]
pub fn ASCII_NONZERO_DIGIT(s: RuleState) -> RuleResult {
    s.match_range('1'..'9')
}

#[inline]
fn XID_CONTINUE(s: RuleState) -> RuleResult {
    s.match_char_by(::pest::unicode::XID_CONTINUE)
}

#[inline]
fn XID_START(s: RuleState) -> RuleResult {
    s.match_char_by(::pest::unicode::XID_START)
}

// region Final

#[inline]
pub fn SKIP(s: RuleState) -> RuleResult {
    match s.atomicity() == Atomicity::NonAtomic {
        true => s.repeat(|s| self::IGNORE(s)),
        false => Ok(s),
    }
}

#[inline]
pub fn IGNORE(s: RuleState) -> RuleResult {
    match cfg!(feature = "no-ignored") {
        true => s.atomic(Atomicity::CompoundAtomic, ignore_terms!(COMMENT, WHITESPACE, NEWLINE)),
        false => s.atomic(Atomicity::CompoundAtomic, |s| s.rule(Rule::IGNORE, ignore_terms!(COMMENT, WHITESPACE, NEWLINE))),
    }
}

#[inline]
pub fn UNNAMED<'i>(s: RuleState<'i>, input: &'i str) -> RuleResult<'i> {
    match cfg!(feature = "no-unnamed") {
        true => s.match_string(input),
        false => s.rule(Rule::UNNAMED, |s| s.match_string(input)),
    }
}

#[inline]
pub fn ANY(s: RuleState) -> RuleResult {
    s.skip(1)
}

#[inline]
pub fn EOI(s: RuleState) -> RuleResult {
    s.rule(Rule::EOI, |s| s.end_of_input())
}

#[inline]
pub fn SOI(s: RuleState) -> RuleResult {
    s.start_of_input()
}

// endregion
