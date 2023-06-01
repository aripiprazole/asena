use crate::ast::node::TokenKind::*;
use crate::ast::node::TreeKind::*;

use super::error::ParseError;
use super::Parser;

impl<'a> Parser<'a> {
    /// File = Decl*
    pub fn file(&mut self) {
        let mark = self.open();

        while !self.eof() {
            self.decl();
        }

        self.close(mark, File)
    }

    /// Decl = DeclSignature
    pub fn decl(&mut self) {
        self.decl_signature()
    }

    /// DeclSignature = Global ':' TypeExpr
    pub fn decl_signature(&mut self) {
        let mark = self.open();

        self.global();
        self.at(Colon);
        self.type_expr();

        self.close(mark, DeclSignature);
    }

    /// TypeExpr = Expr
    pub fn type_expr(&mut self) {
        let mark = self.open();
        self.expr();
        self.close(mark, Type)
    }

    /// Expr = ExprGroup | ExprBinary | ExprAccessor
    ///      | ExprApp | ExprDsl | ExprArray
    ///      | ExprLam | ExprLet | ExprGlobal
    ///      | ExprLocal | ExprLit | ExprAnn
    ///      | ExprQual | ExprPi | ExprSigma | ExprHelp
    pub fn expr(&mut self) {
        self.expr_binary()
    }

    /// ExprBinary = ExprAccessor (Symbol ExprAccessor)*
    pub fn expr_binary(&mut self) {
        let mark = self.open();

        self.expr_ann();

        // simplify by returning the lhs symbol directly
        if self.at(Symbol) {
            while self.at(Symbol) {
                self.advance();
                self.expr_ann();
            }

            self.close(mark, ExprBinary)
        } else {
            self.ignore(mark)
        }
    }

    /// ExprAnn = ExprQual (':' ExprQual)*
    pub fn expr_ann(&mut self) {
        let mark = self.open();

        self.expr_qual();

        // simplify by returning the lhs symbol directly
        if self.at(Colon) {
            while self.at(Colon) {
                self.advance();
                self.expr_qual();
            }

            self.close(mark, ExprAnn)
        } else {
            self.ignore(mark)
        }
    }

    /// ExprQual = ExprAnonymousPi ('=>' ExprAnonymousPi)*
    pub fn expr_qual(&mut self) {
        let mark = self.open();

        self.expr_anonymous_pi();

        // simplify by returning the lhs symbol directly
        if self.at(DoubleArrow) {
            while self.at(DoubleArrow) {
                self.advance();
                self.expr_anonymous_pi();
            }

            self.close(mark, ExprQual)
        } else {
            self.ignore(mark)
        }
    }

    /// ExprAnonymousPi = ExprAccessor ('->' ExprAccessor)*
    pub fn expr_anonymous_pi(&mut self) {
        let mark = self.open();

        self.expr_accessor();

        // simplify by returning the lhs symbol directly
        if self.at(RightArrow) {
            while self.at(RightArrow) {
                self.advance();
                self.expr_accessor();
            }

            self.close(mark, ExprPi)
        } else {
            self.ignore(mark)
        }
    }

    /// ExprAccessor = ExprApp ('.' ExprApp)*
    pub fn expr_accessor(&mut self) {
        let mark = self.open();

        self.expr_app();

        // simplify by returning the lhs symbol directly
        if self.at(Dot) {
            while self.at(Dot) {
                self.advance();
                self.expr_app();
            }

            self.close(mark, ExprAcessor)
        } else {
            self.ignore(mark)
        }
    }

    /// ExprApp = Primary Primary*
    pub fn expr_app(&mut self) {
        self.primary();
    }

    /// Primary = Nat 'n'? | Int 'i8'? | Int 'u8'?
    ///         | Int 'i16'? | Int 'u16'? | Int ('u' | 'i32')?
    ///         | Int ('u' | 'u32')? | Int 'i64'? | Int 'u64'?
    ///         | Int 'i128'? | Int 'u128'? | Float 'f32'?
    ///         | Float 'f64'? | 'true' | 'false'
    pub fn primary(&mut self) -> bool {
        let token = self.peek();

        match token.value.kind {
            TrueKeyword => self.terminal(LitTrue),
            FalseKeyword => self.terminal(LitFalse),

            LetKeyword | IfKeyword | MatchKeyword => {
                // TODO: try to properly parse the expression
                self.report(ParseError::PrimaryMustBeSurrounded(token.value.kind))
            }

            ElseKeyword => self.report(ParseError::DanglingElse),
            CaseKeyword => self.report(ParseError::ReservedKeyword(CaseKeyword)),

            UseKeyword | TypeKeyword | RecordKeyword | ClassKeyword | TraitKeyword
            | InstanceKeyword => self.report(ParseError::DeclReservedKeyword(TypeKeyword)),

            ReturnKeyword => self.report(ParseError::StmtReservedKeyword(ReturnKeyword)),
            WhereKeyword => self.report(ParseError::StmtReservedKeyword(WhereKeyword)),
            InKeyword => self.report(ParseError::ReservedKeyword(InKeyword)),

            Lambda => self.report(ParseError::Unicode(Lambda, "lambda")),
            Forall => self.report(ParseError::Unicode(Lambda, "forall")),
            Pi => self.report(ParseError::Unicode(Lambda, "pi")),
            Sigma => self.report(ParseError::Unicode(Lambda, "sigma")),

            LeftBracket => self.report(ParseError::Unicode(LeftBracket, "left_bracket")),
            RightBracket => self.report(ParseError::Unicode(RightBracket, "right_bracket")),
            LeftBrace => self.report(ParseError::Unicode(LeftBrace, "left_brace")),
            RightBrace => self.report(ParseError::Unicode(RightBrace, "right_brace")),
            RightParen => self.report(ParseError::Unicode(RightParen, "right_paren")),

            Comma => self.report(ParseError::Unicode(Comma, "comma")),
            Semi => self.report(ParseError::Unicode(Semi, "semi")),
            Colon => self.report(ParseError::Unicode(Colon, "colon")),
            Dot => self.report(ParseError::Unicode(Dot, "dot")),
            Help => self.report(ParseError::Unicode(Help, "interrogation")),
            Equal => self.report(ParseError::Unicode(Equal, "equal")),

            DoubleArrow => self.report(ParseError::Unicode(DoubleArrow, "=>")),
            RightArrow => self.report(ParseError::Unicode(RightArrow, "->")),
            LeftArrow => self.report(ParseError::Unicode(LeftArrow, "<-")),

            Int8 => self.terminal(LitInt8),
            Int16 => self.terminal(LitInt16),
            Int32 => self.terminal(LitInt32),
            Int64 => self.terminal(LitInt64),
            Int128 => self.terminal(LitInt128),

            UInt8 => self.terminal(LitUInt8),
            UInt16 => self.terminal(LitUInt16),
            UInt32 => self.terminal(LitUInt32),
            UInt64 => self.terminal(LitUInt64),
            UInt128 => self.terminal(LitUInt128),

            Float32 => self.terminal(LitFloat32),
            Float64 => self.terminal(LitFloat64),

            Symbol => self.report(ParseError::Expected(Identifier)),
            Identifier => self.terminal(LitIdentifier),
            String => self.terminal(LitString),

            Eof => self.report(ParseError::CantParseDueToEof),

            // Parse group or named pi expressions
            // - Pi
            // - Group
            LeftParen => {
                todo!()
            }

            _ => self.report(ParseError::CantParsePrimary),
        }

        false
    }

    /// Global = <<Terminal>>
    pub fn global(&mut self) {
        self.terminal(LitSymbol);
    }

    /// Symbol = <<Terminal>>
    pub fn symbol(&mut self) {
        self.terminal(LitSymbol);
    }
}
