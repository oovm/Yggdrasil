module.exports = grammar({
    name: 'yg',

    extras: $ => [
        $.NEWLINE,
        $.WHITESPACE,
    ],

    supertypes: $ => [

    ],
    inline: $ => [
        $._binary_expression,
        $._grammar_exts
    ],
    word: $ => $.id,

    rules: {
        program: $ => repeat($.statement),

        statement: $ => choice(
            $.grammar_statement,
            $.fragment_statement,
            $.assign_statement
        ),

        // GrammarStatement
        grammar_statement: $ => seq(
            $.grammar,
            field("id", $.id),
            optional($._grammar_exts),
            optional($.eos)
        ),
        _grammar_exts: $ => seq(
            "{",
            optional(interleave(field("ext", $.string), ",", 1)),
            "}"
        ),
        grammar: $ => "grammar!",


        // FragmentStatement
        fragment_statement: $ => seq(
            $.fragment,
            field("id", $.id),
            optional($.eos)
        ),
        fragment: $ => "fragment!",


        // IgnoresStatement
        ignore: $ => "ignore!",


        assign_statement: $ => seq(
            field("id", $.id),
            field("eq", $.eq),
            optional("|"),
            $.expression,
            optional($.eos)
        ),

        eq: $ => choice(
            "=",
            "_=",
            "@="
        ),

        expression: $ => choice(
            seq("(", $.expression, ")"),
            $.id,
            $.string,
            $.unsigned,
            $.macro_call,
            $.regex_long,
            $.regex_range,
            $.regex_set,
            $.unary_suffix,
            $.unary_prefix,
            $._binary_expression,
            // ...
        ),

        unary_prefix: $ => prec.left(200, choice(
            seq(field("prefix", $._prefix_op), field("expr", $.expression)),
            // seq(field("prefix", "!"), field("expr", $.expression)),
        )),
        unary_suffix: $ => prec.right(210,
            seq(field("expr", $.expression), field("suffix", $._suffix_op))
        ),

        _prefix_op: $ => choice(
            "^"
        ),
        _suffix_op: $ => choice(
            "?", "*", "+"
        ),

        _binary_expression: $ => choice(
            // 空格连接禁止换行, 否则有可能会把下面几行的函数给吃进去
            // name <- a ~ b | name ~ c
            // <- 是长程符号
            // ~ 等于空格, 是短程符号
            // 因此上式等价于:
            // name <- ((a ~ b) | (name ~ c))
            // binary_left(100, $.expression, token.immediate(" "), $.expression),
            binary_left(30, $.expression, "~", $.expression),
            binary_left(20, $.variant_tag, "|", $.variant_tag),
            binary_left(10, $.expression, "<-", $.expression),
        ),

        or: $ => prec.left(20, seq(
            field("lhs", $.variant_tag),
            "|",
            field("rhs", $.variant_tag),
        )),

        variant_tag: $ => prec.left(20,seq(
            field("expression", $.expression),
            optional(seq(
                field("op", /[!_]?\#/),
                field("name", $.id),
            ))
        )),

        macro_call: $ => seq(
            "@",
            field("name", $.id),
            optional(seq(".", field("dot", $.id))),
            "(",
            interleave($.expression, ",", 1),
            ")"
        ),

        // Atomic
        id: $ => /[_\p{XID_Start}][\p{XID_Continue}]*/,

        integer: $ => seq(optional($._sign), $.unsigned),
        unsigned: $ => token(/0|[1-9][0-9]*/),
        _sign: $ => /[+-]/,

        string: $ => choice(
            seq(
                "'",
                /[^'\\]*(\\.[^'\\]*)*/,
                "'",
            ),
            seq(
                '"',
                /[^"\\]*(\\.[^"\\]*)*/,
                '"',
            )
        ),

        regex_long: $ => seq(
            "/",
            "/",
            optional(/i|g/)
        ),

        regex_range: $ => seq(
            field("is_neg", choice("[^", "[")),
            repeat($.regex_range_item),
            "]"
        ),
        regex_range_item: $ => choice(
            $.regex_set,
            $.regex_range_item_group,
            /[^\]]/
        ),
        regex_range_item_group: $ => binary_left(10, $.regex_range_item, "-", $.regex_range_item),

        regex_set: $ => seq(
            "\\p",
            "{",
            field("set", /[_0-9a-zA-Z]+/),
            "}"
        ),

        eos: $ => ";",

        NEWLINE: $ => /\r|\r|\n\r/,
        WHITESPACE: $ => /\s/,
    }
});

function interleave(rule, sep, trailing) {
    if (trailing > 0) {
        // must add trailing separator
        return seq(rule, repeat(seq(sep, rule)), sep)
    }
    else if (trailing < 0) {
        // disallow add trailing separator
        return seq(rule, repeat(seq(sep, rule)))
    }
    else {
        // trailing separator is optional
        return seq(rule, repeat(seq(sep, rule)), optional(sep))
    }
}


function ternary_left(p, lhs, op1, mid, op2, rhs) {
    return prec.left(
        p,
        seq(
            field("lhs", lhs),
            field("op1", op1),
            field("mid", mid),
            field("op2", op2),
            field("rhs", rhs),
        )
    )
}


function binary_left(p, lhs, op, rhs) {
    return prec.left(
        p,
        seq(
            field("lhs", lhs),
            field("op", op),
            field("rhs", rhs)
        )
    )
}

function unary_prefix(p, op, base) {
    return prec.right(p, seq(
        field("prefix", op),
        field("expr", base),
    ))
}

function unary_suffix(p, expr, op) {
    return prec.right(p, seq(
        field("expr", base),
        field("suffix", op)
    ))
}

