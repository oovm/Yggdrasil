The ygg language is a parser expression language, and its definition is as follows:

```ygg
grammar Bootstrap {

}

class Root {
    Statement*
}

union Statement {
    | GrammarStatement
    | ClassStatement
    | UnionStatement
    | GroupStatement
    | ExternalStatement
}
// === grammar === -----------------------------------------------------------------------------------------------------
class GrammarStatement {
    ^KW_GRAMMAR Identifier GrammarBlock
}
class GrammarBlock {
    '{' '}'
}
// === class === -----------------------------------------------------------------------------------------------------
class ClassStatement {
    DecoratorCall* ModifierCall* ^KW_CLASS (name:Identifier) ('->' cast:Identifier)? OP_REMARK? ClassBlock
}
class ClassBlock {
    '{' '|'? Expression '}'
}
token {
    OP_REMARK: '^'
}
// === union === -----------------------------------------------------------------------------------------------------
class UnionStatement {
    DecoratorCall* ModifierCall* ^KW_UNION (name:Identifier) OP_REMARK? UnionBlock
}
class UnionBlock {
    '{' UnionBranch* '}'
}
class UnionBranch {
    '|' ExpressionHard BranchTag?
}
@style(field)
atomic class BranchTag {
    '#' Identifier RightAssociativity?
}
class RightAssociativity {
    '>'
}
// === group === -------------------------------------------------------------------------------------------------------
class GroupStatement {
    DecoratorCall* ModifierCall* ^KW_GROUP Identifier? GroupBlock
}
class GroupBlock {
    '{' GroupPair* '}'
}
class GroupPair {
    Identifier ':' Atomic
}
// === external === -------------------------------------------------------------------------------------------------------
class ExternalStatement {
    KW_EXTERNAL Identifier LinkerBlock
}
class LinkerBlock {
    '{' LinkerPair* '}'
}
class LinkerPair {
    Identifier ':' NamepathFree
}
// === decorators === --------------------------------------------------------------------------------------------------
class DecoratorCall {
    DecoratorName CallBody
}
@style(annotation)
class DecoratorName {
    [@#] Identifier
}
class FunctionCall {
    FunctionName CallBody
}
@style(function)
class FunctionName {
    '@' Identifier
}
class CallBody {
    '(' (Expression (',' Expression)* ','?)? ')'
}
// === expression === --------------------------------------------------------------------------------------------------
class Expression     { ExpressionHard ('|' ExpressionHard)* }
class ExpressionHard { ExpressionSoft ('~' ExpressionSoft)* }
class ExpressionSoft { ExpressionTag ExpressionTag* }
class ExpressionTag  { (Identifier ':')? Term }
class Term {
    Prefix* Atomic Suffix*
}
union Prefix {
    | '!' #Negative
    | '&' #Positive
    | '^' #Remark
}
union Suffix {
    | '?' #Optional
    | '*' #Many
    | '+' #Many1
    | RangeExact
    | Range
}
// === atoms === -------------------------------------------------------------------------------------------------------
union Atomic {
    | GroupExpression
    | FunctionCall
    | Boolean
    | Integer
    | StringRaw
    | StringNormal
    | Category
    | EscapedUnicode
    | RegexEmbed
    | RegexRange
    | Identifier
}
class GroupExpression {
    '(' '|'? Expression ')'
}
// === String === -------------------------------------------------------------------------------------------------------
atomic class StringRaw {
    "'" StringRawText "'"
}
text class StringRawText {
    /[^']*/
}
atomic class StringNormal {
    '"' StringItem* '"'
}
union StringItem {
    | EscapedUnicode
    | EscapedCharacter
    | TextAny
}
@style(escape)
atomic class EscapedUnicode {
    '\u{' HEX '}'
}
@style(escape)
text class EscapedCharacter {
    /\\./
}
text class HEX {
    [0-9a-fA-F]{1, 6}
}
text class TextAny {
    /[^"\\]+/
}
// === regex === -------------------------------------------------------------------------------------------------------
atomic class RegexEmbed {
    '/' RegexInner '/'
}
text class RegexInner {
    ([^\/\\] | /\\./)+
}
text atomic class RegexRange {
    '[' RegexNegative? (!']' ANY)* ']'
}
class RegexNegative {
    '^'
}
// === unicode category === --------------------------------------------------------------------------------------------------
class Category {
    ^OP_CATEGORY '{' (group:Identifier '=')? script:Identifier '}'
}
// === identifier === --------------------------------------------------------------------------------------------------
class NamepathFree -> Namepath {
    Identifier (('.' | '::') Identifier)*
}
class Namepath {
    Identifier ('::' Identifier)*
}
text class Identifier {
    /[_\p{XID_start}]\p{XID_continue}*/
}
union Boolean {
    | 'true'  #True
    | 'false' #False
}
// === number === ----------------------------------------------------------------------------------------------------
text class Integer {
    /0|[1-9][0-9]*/
}
class RangeExact {
    '{' Integer '}'
}
class Range {
    '{' (min:Integer)? ',' (max:Integer)? '}'
}
// === keywords === ----------------------------------------------------------------------------------------------------
atomic class ModifierCall ^ {
    !(KW_CLASS | KW_UNION | KW_GROUP | KW_MACRO | KW_CLIMB) ^Identifier
}
@railroad(false)
@style(keyword)
token {
    OP_CATEGORY: '\p'
}
@railroad(false)
@style(keyword)
union KW_EXTERNAL {
    | 'parser'    #Parser
    | 'inspector' #Inspector
    | 'external'  #External
}
@railroad(false)
@style(keyword)
token {
    KW_GRAMMAR: /grammar/
    KW_IMPORT: /using|use|import/
    KW_CLASS: /class|struct/
    KW_UNION: /union|enum/
    KW_GROUP: /group|token/
    KW_CLIMB: /climb/
    KW_MACRO: /macro|def|function|func|fun|fn/
}
// === ignores === -----------------------------------------------------------------------------------------------------
@railroad(false)
hide class WhiteSpace {
    /\p{White_Space}+/
}
@style(comment)
@railroad(false)
hide class Comment {
    | /\/\/[^\n\r]*/
//  | '/*'  '*/'
}
```

- `hide` indicates that the rule is a hidden rule, use `HIDE` to indicate all hide rules.
- `atomic` indicates that they do not contain any hidden rules.
