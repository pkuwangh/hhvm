/**
 * Copyright (c) 2016, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree. An additional
 * directory.
 *
 **
 *
 * THIS FILE IS @generated; DO NOT EDIT IT
 * To regenerate this file, run
 *
 *   buck run //hphp/hack/src:generate_full_fidelity
 *
 **
 *
 */

#[allow(non_camel_case_types)] // allow Include_once and Require_once
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenKind {
    // No text tokens
    EndOfFile,
    // Given text tokens
    Abstract,
    And,
    Array,
    Arraykey,
    As,
    Async,
    Attribute,
    Await,
    Backslash,
    Binary,
    Bool,
    Boolean,
    Break,
    Case,
    Catch,
    Category,
    Children,
    Class,
    Classname,
    Clone,
    Const,
    Construct,
    Continue,
    Coroutine,
    Darray,
    Declare,
    Default,
    Define,
    Destruct,
    Dict,
    Do,
    Double,
    Echo,
    Else,
    Elseif,
    Empty,
    Endfor,
    Endforeach,
    Enddeclare,
    Endif,
    Endswitch,
    Endwhile,
    Enum,
    Eval,
    Extends,
    Fallthrough,
    Float,
    File,
    Final,
    Finally,
    For,
    Foreach,
    From,
    Function,
    Global,
    Concurrent,
    Goto,
    HaltCompiler,
    If,
    Implements,
    Include,
    Include_once,
    Inout,
    Instanceof,
    Insteadof,
    Int,
    Integer,
    Interface,
    Is,
    Isset,
    Keyset,
    Let,
    List,
    Mixed,
    Namespace,
    New,
    Newtype,
    Noreturn,
    Num,
    Object,
    Or,
    Parent,
    Print,
    Private,
    Protected,
    Public,
    Real,
    Reify,
    Require,
    Require_once,
    Required,
    Resource,
    Return,
    SelfToken,
    Shape,
    Static,
    String,
    Super,
    Suspend,
    Switch,
    This,
    Throw,
    Trait,
    Try,
    Tuple,
    Type,
    Unset,
    Use,
    Using,
    Var,
    Varray,
    Vec,
    Void,
    Where,
    While,
    Xor,
    Yield,
    NullLiteral,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Dot,
    MinusGreaterThan,
    PlusPlus,
    MinusMinus,
    StarStar,
    Star,
    Plus,
    Minus,
    Tilde,
    Exclamation,
    Dollar,
    Slash,
    Percent,
    LessThanGreaterThan,
    LessThanEqualGreaterThan,
    LessThanLessThan,
    GreaterThanGreaterThan,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    EqualEqual,
    EqualEqualEqual,
    ExclamationEqual,
    ExclamationEqualEqual,
    Carat,
    Bar,
    Ampersand,
    AmpersandAmpersand,
    BarBar,
    Question,
    QuestionAs,
    QuestionColon,
    QuestionQuestion,
    QuestionQuestionEqual,
    Colon,
    Semicolon,
    Equal,
    StarStarEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    PlusEqual,
    MinusEqual,
    DotEqual,
    LessThanLessThanEqual,
    GreaterThanGreaterThanEqual,
    AmpersandEqual,
    CaratEqual,
    BarEqual,
    Comma,
    At,
    ColonColon,
    EqualGreaterThan,
    EqualEqualGreaterThan,
    QuestionMinusGreaterThan,
    DotDotDot,
    DollarDollar,
    BarGreaterThan,
    SlashGreaterThan,
    LessThanSlash,
    LessThanQuestion,
    QuestionGreaterThan,
    ColonAt,
    // Variable text tokens
    ErrorToken,
    Name,
    Variable,
    DecimalLiteral,
    OctalLiteral,
    HexadecimalLiteral,
    BinaryLiteral,
    FloatingLiteral,
    SingleQuotedStringLiteral,
    DoubleQuotedStringLiteral,
    DoubleQuotedStringLiteralHead,
    StringLiteralBody,
    DoubleQuotedStringLiteralTail,
    HeredocStringLiteral,
    HeredocStringLiteralHead,
    HeredocStringLiteralTail,
    NowdocStringLiteral,
    BooleanLiteral,
    XHPCategoryName,
    XHPElementName,
    XHPClassName,
    XHPStringLiteral,
    XHPBody,
    XHPComment,
    Markup,
}

impl TokenKind {
    pub fn to_string(&self) -> &str {
        match self {
            // No text tokens
            TokenKind::EndOfFile => "end_of_file",
            // Given text tokens
            TokenKind::Abstract => "abstract",
            TokenKind::And => "and",
            TokenKind::Array => "array",
            TokenKind::Arraykey => "arraykey",
            TokenKind::As => "as",
            TokenKind::Async => "async",
            TokenKind::Attribute => "attribute",
            TokenKind::Await => "await",
            TokenKind::Backslash => "\\",
            TokenKind::Binary => "binary",
            TokenKind::Bool => "bool",
            TokenKind::Boolean => "boolean",
            TokenKind::Break => "break",
            TokenKind::Case => "case",
            TokenKind::Catch => "catch",
            TokenKind::Category => "category",
            TokenKind::Children => "children",
            TokenKind::Class => "class",
            TokenKind::Classname => "classname",
            TokenKind::Clone => "clone",
            TokenKind::Const => "const",
            TokenKind::Construct => "__construct",
            TokenKind::Continue => "continue",
            TokenKind::Coroutine => "coroutine",
            TokenKind::Darray => "darray",
            TokenKind::Declare => "declare",
            TokenKind::Default => "default",
            TokenKind::Define => "define",
            TokenKind::Destruct => "__destruct",
            TokenKind::Dict => "dict",
            TokenKind::Do => "do",
            TokenKind::Double => "double",
            TokenKind::Echo => "echo",
            TokenKind::Else => "else",
            TokenKind::Elseif => "elseif",
            TokenKind::Empty => "empty",
            TokenKind::Endfor => "endfor",
            TokenKind::Endforeach => "endforeach",
            TokenKind::Enddeclare => "enddeclare",
            TokenKind::Endif => "endif",
            TokenKind::Endswitch => "endswitch",
            TokenKind::Endwhile => "endwhile",
            TokenKind::Enum => "enum",
            TokenKind::Eval => "eval",
            TokenKind::Extends => "extends",
            TokenKind::Fallthrough => "fallthrough",
            TokenKind::Float => "float",
            TokenKind::File => "file",
            TokenKind::Final => "final",
            TokenKind::Finally => "finally",
            TokenKind::For => "for",
            TokenKind::Foreach => "foreach",
            TokenKind::From => "from",
            TokenKind::Function => "function",
            TokenKind::Global => "global",
            TokenKind::Concurrent => "concurrent",
            TokenKind::Goto => "goto",
            TokenKind::HaltCompiler => "__halt_compiler",
            TokenKind::If => "if",
            TokenKind::Implements => "implements",
            TokenKind::Include => "include",
            TokenKind::Include_once => "include_once",
            TokenKind::Inout => "inout",
            TokenKind::Instanceof => "instanceof",
            TokenKind::Insteadof => "insteadof",
            TokenKind::Int => "int",
            TokenKind::Integer => "integer",
            TokenKind::Interface => "interface",
            TokenKind::Is => "is",
            TokenKind::Isset => "isset",
            TokenKind::Keyset => "keyset",
            TokenKind::Let => "let",
            TokenKind::List => "list",
            TokenKind::Mixed => "mixed",
            TokenKind::Namespace => "namespace",
            TokenKind::New => "new",
            TokenKind::Newtype => "newtype",
            TokenKind::Noreturn => "noreturn",
            TokenKind::Num => "num",
            TokenKind::Object => "object",
            TokenKind::Or => "or",
            TokenKind::Parent => "parent",
            TokenKind::Print => "print",
            TokenKind::Private => "private",
            TokenKind::Protected => "protected",
            TokenKind::Public => "public",
            TokenKind::Real => "real",
            TokenKind::Reify => "reify",
            TokenKind::Require => "require",
            TokenKind::Require_once => "require_once",
            TokenKind::Required => "required",
            TokenKind::Resource => "resource",
            TokenKind::Return => "return",
            TokenKind::SelfToken => "self",
            TokenKind::Shape => "shape",
            TokenKind::Static => "static",
            TokenKind::String => "string",
            TokenKind::Super => "super",
            TokenKind::Suspend => "suspend",
            TokenKind::Switch => "switch",
            TokenKind::This => "this",
            TokenKind::Throw => "throw",
            TokenKind::Trait => "trait",
            TokenKind::Try => "try",
            TokenKind::Tuple => "tuple",
            TokenKind::Type => "type",
            TokenKind::Unset => "unset",
            TokenKind::Use => "use",
            TokenKind::Using => "using",
            TokenKind::Var => "var",
            TokenKind::Varray => "varray",
            TokenKind::Vec => "vec",
            TokenKind::Void => "void",
            TokenKind::Where => "where",
            TokenKind::While => "while",
            TokenKind::Xor => "xor",
            TokenKind::Yield => "yield",
            TokenKind::NullLiteral => "null",
            TokenKind::LeftBracket => "[",
            TokenKind::RightBracket => "]",
            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",
            TokenKind::LeftBrace => "{",
            TokenKind::RightBrace => "}",
            TokenKind::Dot => ".",
            TokenKind::MinusGreaterThan => "->",
            TokenKind::PlusPlus => "++",
            TokenKind::MinusMinus => "--",
            TokenKind::StarStar => "**",
            TokenKind::Star => "*",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Tilde => "~",
            TokenKind::Exclamation => "!",
            TokenKind::Dollar => "$",
            TokenKind::Slash => "/",
            TokenKind::Percent => "%",
            TokenKind::LessThanGreaterThan => "<>",
            TokenKind::LessThanEqualGreaterThan => "<=>",
            TokenKind::LessThanLessThan => "<<",
            TokenKind::GreaterThanGreaterThan => ">>",
            TokenKind::LessThan => "<",
            TokenKind::GreaterThan => ">",
            TokenKind::LessThanEqual => "<=",
            TokenKind::GreaterThanEqual => ">=",
            TokenKind::EqualEqual => "==",
            TokenKind::EqualEqualEqual => "===",
            TokenKind::ExclamationEqual => "!=",
            TokenKind::ExclamationEqualEqual => "!==",
            TokenKind::Carat => "^",
            TokenKind::Bar => "|",
            TokenKind::Ampersand => "&",
            TokenKind::AmpersandAmpersand => "&&",
            TokenKind::BarBar => "||",
            TokenKind::Question => "?",
            TokenKind::QuestionAs => "?as",
            TokenKind::QuestionColon => "?:",
            TokenKind::QuestionQuestion => "??",
            TokenKind::QuestionQuestionEqual => "??=",
            TokenKind::Colon => ":",
            TokenKind::Semicolon => ";",
            TokenKind::Equal => "=",
            TokenKind::StarStarEqual => "**=",
            TokenKind::StarEqual => "*=",
            TokenKind::SlashEqual => "/=",
            TokenKind::PercentEqual => "%=",
            TokenKind::PlusEqual => "+=",
            TokenKind::MinusEqual => "-=",
            TokenKind::DotEqual => ".=",
            TokenKind::LessThanLessThanEqual => "<<=",
            TokenKind::GreaterThanGreaterThanEqual => ">>=",
            TokenKind::AmpersandEqual => "&=",
            TokenKind::CaratEqual => "^=",
            TokenKind::BarEqual => "|=",
            TokenKind::Comma => ",",
            TokenKind::At => "@",
            TokenKind::ColonColon => "::",
            TokenKind::EqualGreaterThan => "=>",
            TokenKind::EqualEqualGreaterThan => "==>",
            TokenKind::QuestionMinusGreaterThan => "?->",
            TokenKind::DotDotDot => "...",
            TokenKind::DollarDollar => "$$",
            TokenKind::BarGreaterThan => "|>",
            TokenKind::SlashGreaterThan => "/>",
            TokenKind::LessThanSlash => "</",
            TokenKind::LessThanQuestion => "<?",
            TokenKind::QuestionGreaterThan => "?>",
            TokenKind::ColonAt => ":@",
            // Variable text tokes
            TokenKind::ErrorToken => "error_token",
            TokenKind::Name => "name",
            TokenKind::Variable => "variable",
            TokenKind::DecimalLiteral => "decimal_literal",
            TokenKind::OctalLiteral => "octal_literal",
            TokenKind::HexadecimalLiteral => "hexadecimal_literal",
            TokenKind::BinaryLiteral => "binary_literal",
            TokenKind::FloatingLiteral => "floating_literal",
            TokenKind::SingleQuotedStringLiteral => "single_quoted_string_literal",
            TokenKind::DoubleQuotedStringLiteral => "double_quoted_string_literal",
            TokenKind::DoubleQuotedStringLiteralHead => "double_quoted_string_literal_head",
            TokenKind::StringLiteralBody => "string_literal_body",
            TokenKind::DoubleQuotedStringLiteralTail => "double_quoted_string_literal_tail",
            TokenKind::HeredocStringLiteral => "heredoc_string_literal",
            TokenKind::HeredocStringLiteralHead => "heredoc_string_literal_head",
            TokenKind::HeredocStringLiteralTail => "heredoc_string_literal_tail",
            TokenKind::NowdocStringLiteral => "nowdoc_string_literal",
            TokenKind::BooleanLiteral => "boolean_literal",
            TokenKind::XHPCategoryName => "XHP_category_name",
            TokenKind::XHPElementName => "XHP_element_name",
            TokenKind::XHPClassName => "XHP_class_name",
            TokenKind::XHPStringLiteral => "XHP_string_literal",
            TokenKind::XHPBody => "XHP_body",
            TokenKind::XHPComment => "XHP_comment",
            TokenKind::Markup => "markup",
        }
    }

    pub fn from_string(
        keyword: &[u8],
        is_hack: bool,
        allow_xhp: bool,
        only_reserved: bool,
    ) -> Option<Self> {
        let keyword = unsafe { std::str::from_utf8_unchecked(keyword) };
        match keyword {
            "true" if !only_reserved => Some(TokenKind::BooleanLiteral),
            "false" if !only_reserved => Some(TokenKind::BooleanLiteral),
            "abstract" => Some(TokenKind::Abstract),
            "and" => Some(TokenKind::And),
            "array" => Some(TokenKind::Array),
            "arraykey" if is_hack && !only_reserved => Some(TokenKind::Arraykey),
            "as" => Some(TokenKind::As),
            "async" if is_hack => Some(TokenKind::Async),
            "attribute" if (is_hack || allow_xhp) && !only_reserved => Some(TokenKind::Attribute),
            "await" if is_hack => Some(TokenKind::Await),
            "\\" => Some(TokenKind::Backslash),
            "binary" if !only_reserved => Some(TokenKind::Binary),
            "bool" if !only_reserved => Some(TokenKind::Bool),
            "boolean" if !only_reserved => Some(TokenKind::Boolean),
            "break" => Some(TokenKind::Break),
            "case" => Some(TokenKind::Case),
            "catch" => Some(TokenKind::Catch),
            "category" if (is_hack || allow_xhp) && !only_reserved => Some(TokenKind::Category),
            "children" if (is_hack || allow_xhp) && !only_reserved => Some(TokenKind::Children),
            "class" => Some(TokenKind::Class),
            "classname" if is_hack && !only_reserved => Some(TokenKind::Classname),
            "clone" => Some(TokenKind::Clone),
            "const" => Some(TokenKind::Const),
            "__construct" => Some(TokenKind::Construct),
            "continue" => Some(TokenKind::Continue),
            "coroutine" if is_hack && !only_reserved => Some(TokenKind::Coroutine),
            "darray" if is_hack && !only_reserved => Some(TokenKind::Darray),
            "declare" => Some(TokenKind::Declare),
            "default" => Some(TokenKind::Default),
            "define" if !only_reserved => Some(TokenKind::Define),
            "__destruct" => Some(TokenKind::Destruct),
            "dict" if !only_reserved => Some(TokenKind::Dict),
            "do" => Some(TokenKind::Do),
            "double" if !only_reserved => Some(TokenKind::Double),
            "echo" => Some(TokenKind::Echo),
            "else" => Some(TokenKind::Else),
            "elseif" => Some(TokenKind::Elseif),
            "empty" => Some(TokenKind::Empty),
            "endfor" => Some(TokenKind::Endfor),
            "endforeach" => Some(TokenKind::Endforeach),
            "enddeclare" => Some(TokenKind::Enddeclare),
            "endif" => Some(TokenKind::Endif),
            "endswitch" => Some(TokenKind::Endswitch),
            "endwhile" => Some(TokenKind::Endwhile),
            "enum" if (is_hack || allow_xhp) && !only_reserved => Some(TokenKind::Enum),
            "eval" => Some(TokenKind::Eval),
            "extends" => Some(TokenKind::Extends),
            "fallthrough" if is_hack && !only_reserved => Some(TokenKind::Fallthrough),
            "float" if !only_reserved => Some(TokenKind::Float),
            "file" if is_hack && !only_reserved => Some(TokenKind::File),
            "final" => Some(TokenKind::Final),
            "finally" => Some(TokenKind::Finally),
            "for" => Some(TokenKind::For),
            "foreach" => Some(TokenKind::Foreach),
            "from" if !only_reserved => Some(TokenKind::From),
            "function" => Some(TokenKind::Function),
            "global" => Some(TokenKind::Global),
            "concurrent" if is_hack => Some(TokenKind::Concurrent),
            "goto" => Some(TokenKind::Goto),
            "__halt_compiler" => Some(TokenKind::HaltCompiler),
            "if" => Some(TokenKind::If),
            "implements" => Some(TokenKind::Implements),
            "include" => Some(TokenKind::Include),
            "include_once" => Some(TokenKind::Include_once),
            "inout" if is_hack => Some(TokenKind::Inout),
            "instanceof" => Some(TokenKind::Instanceof),
            "insteadof" => Some(TokenKind::Insteadof),
            "int" if !only_reserved => Some(TokenKind::Int),
            "integer" if !only_reserved => Some(TokenKind::Integer),
            "interface" => Some(TokenKind::Interface),
            "is" if is_hack && !only_reserved => Some(TokenKind::Is),
            "isset" => Some(TokenKind::Isset),
            "keyset" if !only_reserved => Some(TokenKind::Keyset),
            "let" if is_hack && !only_reserved => Some(TokenKind::Let),
            "list" => Some(TokenKind::List),
            "mixed" if is_hack && !only_reserved => Some(TokenKind::Mixed),
            "namespace" if !only_reserved => Some(TokenKind::Namespace),
            "new" => Some(TokenKind::New),
            "newtype" if is_hack && !only_reserved => Some(TokenKind::Newtype),
            "noreturn" if is_hack && !only_reserved => Some(TokenKind::Noreturn),
            "num" if is_hack && !only_reserved => Some(TokenKind::Num),
            "object" if !only_reserved => Some(TokenKind::Object),
            "or" => Some(TokenKind::Or),
            "parent" if !only_reserved => Some(TokenKind::Parent),
            "print" => Some(TokenKind::Print),
            "private" => Some(TokenKind::Private),
            "protected" => Some(TokenKind::Protected),
            "public" => Some(TokenKind::Public),
            "real" if !only_reserved => Some(TokenKind::Real),
            "reify" if is_hack && !only_reserved => Some(TokenKind::Reify),
            "require" => Some(TokenKind::Require),
            "require_once" => Some(TokenKind::Require_once),
            "required" if (is_hack || allow_xhp) => Some(TokenKind::Required),
            "resource" if !only_reserved => Some(TokenKind::Resource),
            "return" => Some(TokenKind::Return),
            "self" if !only_reserved => Some(TokenKind::SelfToken),
            "shape" if is_hack => Some(TokenKind::Shape),
            "static" => Some(TokenKind::Static),
            "string" if !only_reserved => Some(TokenKind::String),
            "super" if !only_reserved => Some(TokenKind::Super),
            "suspend" if is_hack && !only_reserved => Some(TokenKind::Suspend),
            "switch" => Some(TokenKind::Switch),
            "this" if is_hack && !only_reserved => Some(TokenKind::This),
            "throw" => Some(TokenKind::Throw),
            "trait" => Some(TokenKind::Trait),
            "try" => Some(TokenKind::Try),
            "tuple" if is_hack => Some(TokenKind::Tuple),
            "type" if is_hack && !only_reserved => Some(TokenKind::Type),
            "unset" => Some(TokenKind::Unset),
            "use" => Some(TokenKind::Use),
            "using" if is_hack => Some(TokenKind::Using),
            "var" => Some(TokenKind::Var),
            "varray" if is_hack && !only_reserved => Some(TokenKind::Varray),
            "vec" if !only_reserved => Some(TokenKind::Vec),
            "void" if !only_reserved => Some(TokenKind::Void),
            "where" if is_hack && !only_reserved => Some(TokenKind::Where),
            "while" => Some(TokenKind::While),
            "xor" => Some(TokenKind::Xor),
            "yield" => Some(TokenKind::Yield),
            "null" if !only_reserved => Some(TokenKind::NullLiteral),
            "[" => Some(TokenKind::LeftBracket),
            "]" => Some(TokenKind::RightBracket),
            "(" => Some(TokenKind::LeftParen),
            ")" => Some(TokenKind::RightParen),
            "{" => Some(TokenKind::LeftBrace),
            "}" => Some(TokenKind::RightBrace),
            "." => Some(TokenKind::Dot),
            "->" => Some(TokenKind::MinusGreaterThan),
            "++" => Some(TokenKind::PlusPlus),
            "--" => Some(TokenKind::MinusMinus),
            "**" => Some(TokenKind::StarStar),
            "*" => Some(TokenKind::Star),
            "+" => Some(TokenKind::Plus),
            "-" => Some(TokenKind::Minus),
            "~" => Some(TokenKind::Tilde),
            "!" => Some(TokenKind::Exclamation),
            "$" => Some(TokenKind::Dollar),
            "/" => Some(TokenKind::Slash),
            "%" => Some(TokenKind::Percent),
            "<>" => Some(TokenKind::LessThanGreaterThan),
            "<=>" => Some(TokenKind::LessThanEqualGreaterThan),
            "<<" => Some(TokenKind::LessThanLessThan),
            ">>" => Some(TokenKind::GreaterThanGreaterThan),
            "<" => Some(TokenKind::LessThan),
            ">" => Some(TokenKind::GreaterThan),
            "<=" => Some(TokenKind::LessThanEqual),
            ">=" => Some(TokenKind::GreaterThanEqual),
            "==" => Some(TokenKind::EqualEqual),
            "===" => Some(TokenKind::EqualEqualEqual),
            "!=" => Some(TokenKind::ExclamationEqual),
            "!==" => Some(TokenKind::ExclamationEqualEqual),
            "^" => Some(TokenKind::Carat),
            "|" => Some(TokenKind::Bar),
            "&" => Some(TokenKind::Ampersand),
            "&&" => Some(TokenKind::AmpersandAmpersand),
            "||" => Some(TokenKind::BarBar),
            "?" => Some(TokenKind::Question),
            "?as" => Some(TokenKind::QuestionAs),
            "?:" => Some(TokenKind::QuestionColon),
            "??" => Some(TokenKind::QuestionQuestion),
            "??=" => Some(TokenKind::QuestionQuestionEqual),
            ":" => Some(TokenKind::Colon),
            ";" => Some(TokenKind::Semicolon),
            "=" => Some(TokenKind::Equal),
            "**=" => Some(TokenKind::StarStarEqual),
            "*=" => Some(TokenKind::StarEqual),
            "/=" => Some(TokenKind::SlashEqual),
            "%=" => Some(TokenKind::PercentEqual),
            "+=" => Some(TokenKind::PlusEqual),
            "-=" => Some(TokenKind::MinusEqual),
            ".=" => Some(TokenKind::DotEqual),
            "<<=" => Some(TokenKind::LessThanLessThanEqual),
            ">>=" => Some(TokenKind::GreaterThanGreaterThanEqual),
            "&=" => Some(TokenKind::AmpersandEqual),
            "^=" => Some(TokenKind::CaratEqual),
            "|=" => Some(TokenKind::BarEqual),
            "," => Some(TokenKind::Comma),
            "@" => Some(TokenKind::At),
            "::" => Some(TokenKind::ColonColon),
            "=>" => Some(TokenKind::EqualGreaterThan),
            "==>" => Some(TokenKind::EqualEqualGreaterThan),
            "?->" => Some(TokenKind::QuestionMinusGreaterThan),
            "..." => Some(TokenKind::DotDotDot),
            "$$" => Some(TokenKind::DollarDollar),
            "|>" => Some(TokenKind::BarGreaterThan),
            "/>" => Some(TokenKind::SlashGreaterThan),
            "</" => Some(TokenKind::LessThanSlash),
            "<?" => Some(TokenKind::LessThanQuestion),
            "?>" => Some(TokenKind::QuestionGreaterThan),
            ":@" if is_hack => Some(TokenKind::ColonAt),
            _ => None,
        }
    }
}
