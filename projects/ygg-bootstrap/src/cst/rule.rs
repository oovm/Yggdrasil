use super::*;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    program,
    statement,
    empty_statement,
    eos,
    grammar_statement,
    grammar,
    fragment_statement,
    fragment,
    mark_branch,
    mark_type,
    import_statement,
    import,
    ignore_statement,
    ignore,
    assign_statement,
    assign_kind,
    expr,
    __rec_expr_left,
    __rec_expr_rest,
    BRANCH,
    branch_tag,
    prefix,
    suffix,
    data,
    list,
    slice,
    regex_range,
    macro_call,
    macro_define,
    macro_kv,
    macro_arg,
    block,
    string,
    integer,
    special,
    comment_doc,
    comment_s_l,
    comment_m_l,
    id_path,
    SYMBOL,
    WHITESPACE,
    NEWLINE,
    COMMENT,
    symbol_alias,
    __rec_expr,
    __rec_expr_concat,
    choice,
    __rec_choice,
    __rec_expr_choice,
    IGNORE,
    UNMARKED,
    UNNAMED,
    EOI,
}

impl Parser<Rule> for CSTBuilder {
    fn parse(rule: Rule, input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
        pest::state(input, |state| match rule {
            Rule::program => parse::program(state),
            Rule::statement => parse::statement(state),
            Rule::empty_statement => parse::empty_statement(state),
            Rule::eos => parse::eos(state),
            Rule::grammar_statement => parse::grammar_statement(state),
            Rule::grammar => parse::grammar(state),
            Rule::fragment_statement => parse::fragment_statement(state),
            Rule::fragment => parse::fragment(state),
            Rule::import_statement => parse::import_statement(state),
            Rule::ignore_statement => parse::ignore_statement(state),
            Rule::assign_statement => parse::assign_statement(state),
            Rule::assign_kind => parse::assign_kind(state),
            Rule::expr => parse::expr(state),
            Rule::prefix => parse::prefix(state),
            Rule::suffix => parse::suffix(state),
            Rule::data => parse::data(state),
            Rule::list => parse::list(state),
            Rule::slice => parse::slice(state),
            Rule::regex_range => parse::regex_range(state),
            Rule::macro_call => parse::macro_call(state),
            Rule::macro_define => parse::macro_define(state),
            Rule::macro_kv => parse::macro_kv(state),
            Rule::macro_arg => parse::macro_arg(state),
            Rule::block => parse::block(state),
            Rule::string => parse::string(state),
            Rule::integer => parse::integer(state),
            Rule::special => parse::special(state),
            Rule::comment_doc => parse::comment_doc(state),
            Rule::comment_s_l => parse::comment_s_l(state),
            Rule::comment_m_l => parse::comment_m_l(state),
            Rule::COMMENT => parse::COMMENT(state),
            Rule::id_path => parse::symbol_path(state),
            Rule::symbol_alias => parse::symbol_alias(state),
            Rule::SYMBOL => parse::symbol(state),
            Rule::IGNORE => parse::IGNORE(state),
            Rule::WHITESPACE => parse::WHITESPACE(state),
            Rule::NEWLINE => parse::NEWLINE(state),
            _ => unreachable!("cannot start with such rule {:?}", rule),
        })
    }
}
