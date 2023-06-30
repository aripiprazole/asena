use asena_leaf::token::{kind::TokenKind::*, Token};

use crate::{HighlightColor, SyntaxHighlight};

impl SyntaxHighlight for Token {
    fn highlight(&self) -> crate::HighlightColor {
        match self.kind {
            Error => HighlightColor::Error,
            Nat => HighlightColor::Number,
            InKeyword => HighlightColor::SoftKeyword,
            EqualSymbol => HighlightColor::Assign,
            HashSymbol => HighlightColor::Attribute,
            Symbol => HighlightColor::Operator,
            // TODO: use a better way to highlight builtin types using semantic >>>
            Identifier if self.text == "String" => HighlightColor::BuiltinType,
            Identifier if self.text == "Unit" => HighlightColor::BuiltinType,
            Identifier if self.text == "Maybe" => HighlightColor::BuiltinType,
            Identifier if self.text == "Just" => HighlightColor::BuiltinFunction,
            Identifier if self.text == "Nothing" => HighlightColor::BuiltinFunction,
            Identifier if self.text == "todo" => HighlightColor::BuiltinFunction,
            Identifier if self.text == "panic" => HighlightColor::BuiltinFunction,
            // <<<
            Identifier => HighlightColor::LocalReference,
            Str => HighlightColor::String,
            Eof => HighlightColor::Eof,
            LetKeyword | TrueKeyword | FalseKeyword | IfKeyword | ElseKeyword | ThenKeyword
            | TypeKeyword | RecordKeyword | ReturnKeyword | EnumKeyword | TraitKeyword
            | ClassKeyword | InstanceKeyword | CaseKeyword | WhereKeyword | MatchKeyword
            | UseKeyword => HighlightColor::HardKeyword,
            Int8 | UInt8 | Int16 | UInt16 | Int32 | UInt32 | Int64 | UInt64 | Int128 | UInt128
            | Float32 | Float64 => HighlightColor::Number,
            LeftBracket | RightBracket | LeftBrace | RightBrace | LeftParen | RightParen
            | Comma | Semi | Colon | Dot => HighlightColor::Delimitator,
            LambdaUnicode | ForallUnicode | PiUnicode | SigmaUnicode => {
                HighlightColor::GlobalFunction
            }
            HelpSymbol | DoubleArrow | RightArrow | LeftArrow => HighlightColor::Operator,
            FunKeyword => HighlightColor::HardKeyword,
            SelfKeyword => HighlightColor::SoftKeyword,
        }
    }
}
