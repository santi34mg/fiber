use std::iter::Peekable;
use std::fmt;

use crate::parser::function::{FunctionBody, FunctionParameter, FunctionSignature};
use crate::parser::{Ast, Expr, Function, Statement, VarDecl};
use crate::token::{Keyword, Operator, Punctuation, Token, TokenKind};

#[derive(Debug)]
pub struct ParseError {
    pub filename: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub source_line: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Standard format: file:line:column
        writeln!(f, "{}:{}:{}:", self.filename, self.line, self.column)?;
        writeln!(f, "{}", self.message)?;
        writeln!(f, "\t{}", self.source_line)?;
        writeln!(f, "\t{}^", " ".repeat(self.column - 1))
    }
}

type ParseResult<T> = Result<T, ParseError>;

pub struct Parser<I>
where
    I: Iterator<Item = Token> + Clone,
{
    tokens: Peekable<I>,
    filename: String,
    source_lines: Vec<String>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token> + Clone,
{
    pub fn new(tokens: I, filename: String, source: String) -> Self {
        Self {
            tokens: tokens.peekable(),
            filename,
            source_lines: source.lines().map(|s| s.to_string()).collect(),
        }
    }

    fn error(&self, message: &str, line: usize, column: usize) -> ParseError {
        let source_line = self
            .source_lines
            .get(line.saturating_sub(1))
            .cloned()
            .unwrap_or_default();
        ParseError {
            filename: self.filename.clone(),
            message: message.to_string(),
            line,
            column,
            source_line,
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    /// Consume and return the next token, or error if none.
    fn expect_next(&mut self, msg: &str) -> ParseResult<Token> {
        self.next().ok_or_else(|| {
            let line = 0;
            let column = 0;
            self.error(msg, line, column)
        })
    }

    /// Consume and check the next token matches the predicate, or error.
    fn expect_token<F>(&mut self, pred: F, msg: &str) -> ParseResult<Token>
    where
        F: FnOnce(&Token) -> bool,
    {
        let token = self.expect_next(msg)?;
        if pred(&token) {
            Ok(token)
        } else {
            Err(self.error(msg, token.line, token.column))
        }
    }

    /// Consume the next token if it matches the predicate.
    fn consume_if<F>(&mut self, pred: F) -> Option<Token>
    where
        F: FnOnce(&Token) -> bool,
    {
        if let Some(token) = self.peek() {
            if pred(token) {
                return self.next();
            }
        }
        None
    }

    pub fn parse_program(&mut self) -> ParseResult<Ast> {
        let mut ast = Ast::new();
        while self.peek().is_some() {
            let statement = self.parse_statement();
            ast.statements.push(statement?);
        }
        Ok(ast)
    }

    fn parse_statement(&mut self) -> ParseResult<Statement> {
        let stmt = if let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::Keyword(Keyword::If) => {
                    self.next(); // consume 'if'
                        let condition = self.parse_expression()?;
                        // Parse then-branch using shared parse_body
                        let then_branch = self.parse_body()?;
                        // Check for optional else
                        let else_branch = if let Some(token) = self.peek() {
                            if matches!(token.kind, TokenKind::Keyword(Keyword::Else)) {
                                self.next(); // consume 'else'
                                let else_stmts = self.parse_body()?;
                                Some(else_stmts)
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        Statement::If {
                            condition,
                            then_branch,
                            else_branch,
                        }
                }
                TokenKind::Keyword(Keyword::Func) => {
                    let user_function = self.parse_function_declaration()?;
                    Statement::FunctionDeclaration(user_function)
                }
                TokenKind::Keyword(Keyword::Var) => {
                    let stmt = self.parse_var_decl()?;
                    Statement::VarDecl(stmt)
                }
                TokenKind::Keyword(Keyword::Return) => {
                    self.next(); // consume 'return'
                    // Optionally parse an expression after return
                    if let Some(token) = self.peek() {
                        // If next token is not a semicolon or block close, parse expression
                        match token.kind {
                            TokenKind::Punctuation(Punctuation::Semicolon)
                            | TokenKind::Punctuation(Punctuation::CloseCurly) => {
                                Statement::Return(None)
                            }
                            _ => {
                                let expr = self.parse_expression()?;
                                Statement::Return(Some(expr))
                            }
                        }
                    } else {
                        Statement::Return(None)
                    }
                }
                TokenKind::Identifier(_) => {
                    // Try to parse as assignment or increment/decrement
                    if self.is_assignment()? {
                        let stmt = self.parse_assignment()?;
                        Statement::Assignment {
                            identifier: stmt.0,
                            expr: stmt.1,
                        }
                    } else if self.is_increment_decrement()? {
                        let (identifier, op) = self.parse_increment_decrement()?;
                        // Represent as assignment: x++ => x = x + 1, x-- => x = x - 1
                        let expr = Expr::Binary {
                            left: Box::new(Expr::Ident(identifier.clone())),
                            op,
                            right: Box::new(Expr::Number(1)),
                        };
                        Statement::Assignment { identifier, expr }
                    } else {
                        let expr = self.parse_expression()?;
                        Statement::Expr(expr)
                    }
                }
                TokenKind::Keyword(Keyword::While) => {
                    // For simplicity, treat while as an expression statement for now
                    self.next(); // consume 'while'
                    let condition = self.parse_expression()?;
                    // Use shared parse_body to consume the block
                    let _body = self.parse_body()?;
                    // Represent while as a function call for now (to be implemented properly later)
                    let while_expr = Expr::Call {
                        callee: Box::new(Expr::Ident("while".to_string())),
                        args: vec![condition], // Incomplete representation
                    };
                    Statement::Expr(while_expr)
                }
                TokenKind::TypeIdentifier(_)
                | TokenKind::Keyword(Keyword::Else)
                | TokenKind::NumberLiteral(_)
                | TokenKind::BooleanLiteral(_)
                | TokenKind::CharLiteral(_)
                | TokenKind::Operator(_)
                | TokenKind::Punctuation(_)
                | TokenKind::Unkown(_) => {
                    let t = token.clone();
                    return Err(self.error("unsupported", t.line, t.column));
                }
            }
        } else {
            return Err(self.error("parse_statement: expected a token, found none", 0, 0));
        };

        // Optionally consume a semicolon if present
        self.consume_if(|t| matches!(t.kind, TokenKind::Punctuation(Punctuation::Semicolon)));
        Ok(stmt)
    }

    /// Checks if the next tokens represent an assignment (identifier followed by '=')
    fn is_assignment(&mut self) -> ParseResult<bool> {
        let mut iter = self.tokens.clone();
        if let Some(token) = iter.next() {
            if let TokenKind::Identifier(_) = token.kind {
                if let Some(next_token) = iter.next() {
                    if let TokenKind::Operator(Operator::Assign) = next_token.kind {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }

    /// Checks if the next tokens represent an increment or decrement (identifier followed by ++ or --)
    fn is_increment_decrement(&mut self) -> ParseResult<bool> {
        let mut iter = self.tokens.clone();
        if let Some(token) = iter.next() {
            if let TokenKind::Identifier(_) = token.kind {
                if let Some(next_token) = iter.next() {
                    if let TokenKind::Operator(Operator::Increment)
                    | TokenKind::Operator(Operator::Decrement) = next_token.kind
                    {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }

    /// Helper to parse '= expr'
    fn parse_initializer(&mut self) -> ParseResult<Expr> {
        self.expect_token(
            |t| matches!(t.kind, TokenKind::Operator(Operator::Assign)),
            "expected '='",
        )?;
        self.parse_expression()
    }

    /// Parses an assignment statement: identifier '=' expression
    fn parse_assignment(&mut self) -> ParseResult<(String, Expr)> {
        let ident_token = self.expect_token(
            |t| matches!(t.kind, TokenKind::Identifier(_)),
            "parse_assignment: expected identifier",
        )?;
        let identifier = if let TokenKind::Identifier(id) = ident_token.kind {
            id
        } else {
            unreachable!()
        };

        let expr = self.parse_initializer()?;
        Ok((identifier, expr))
    }

    /// Parses an increment or decrement statement: identifier ++ or identifier --
    fn parse_increment_decrement(&mut self) -> ParseResult<(String, Operator)> {
        let ident_token = self.expect_token(
            |t| matches!(t.kind, TokenKind::Identifier(_)),
            "parse_increment_decrement: expected identifier",
        )?;
        let identifier = if let TokenKind::Identifier(id) = ident_token.kind {
            id
        } else {
            unreachable!()
        };

        let op_token = self.expect_next("parse_increment_decrement: expected '++' or '--'")?;
        let op = match op_token.kind {
            TokenKind::Operator(Operator::Increment) => Operator::Plus,
            TokenKind::Operator(Operator::Decrement) => Operator::Minus,
            _ => {
                return Err(self.error(
                    "parse_increment_decrement: expected '++' or '--'",
                    op_token.line,
                    op_token.column,
                ));
            }
        };

        Ok((identifier, op))
    }

    fn parse_var_decl(&mut self) -> ParseResult<VarDecl> {
        self.expect_token(
            |t| matches!(t.kind, TokenKind::Keyword(Keyword::Var)),
            "parse_var_decl: expected 'var' keyword",
        )?;

        let ident_token = self.expect_token(
            |t| matches!(t.kind, TokenKind::Identifier(_)),
            "parse_var_decl: expected identifier",
        )?;
        let ident = if let TokenKind::Identifier(ident) = ident_token.kind {
            ident
        } else {
            unreachable!()
        };

        let type_token = self.expect_token(
            |t| matches!(t.kind, TokenKind::TypeIdentifier(_)),
            "parse_var_decl: expected type identifier",
        )?;
        let var_type = if let TokenKind::TypeIdentifier(t_ident) = type_token.kind {
            t_ident
        } else {
            unreachable!()
        };

        let expr = self.parse_initializer()?;
        Ok(VarDecl::new(ident, var_type, expr))
    }

    fn parse_expression(&mut self) -> ParseResult<Expr> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_logical_and()?;
        while let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::Operator(Operator::Or) => {
                    self.next();
                    let right = Box::new(self.parse_logical_and()?);
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: Operator::Or,
                        right,
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_equality()?;
        while let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::Operator(Operator::And) => {
                    self.next();
                    let right = Box::new(self.parse_equality()?);
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: Operator::And,
                        right,
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    /// Parse equality and comparison expressions (==, !=, >, <, >=, <=)
    fn parse_equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_comparison()?;

        while let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::Operator(op @ (Operator::Equals | Operator::Different)) => {
                    let op = *op;
                    self.next();
                    let right = Box::new(self.parse_comparison()?);
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op,
                        right,
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_additive()?;

        while let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::Operator(
                    op @ (Operator::GreaterThan
                    | Operator::LesserThan
                    | Operator::GreaterEqual
                    | Operator::LesserEqual),
                ) => {
                    let op = *op;
                    self.next();
                    let right = Box::new(self.parse_additive()?);
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op,
                        right,
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_additive(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_term()?;

        while let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::Operator(op @ (Operator::Plus | Operator::Minus)) => {
                    let op = *op;
                    self.next();
                    let right = Box::new(self.parse_term()?);
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op,
                        right,
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_unary()?;

        while let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::Operator(op @ (Operator::Multply | Operator::Divide)) => {
                    let op = *op;
                    self.next();
                    let right = Box::new(self.parse_unary()?);
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op,
                        right,
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    /// Parse unary expressions, including '!' for boolean negation.
    fn parse_unary(&mut self) -> ParseResult<Expr> {
        if let Some(_) = self.peek() {
            // If there's a '!' operator, consume it and parse unary recursively
            if let Some(_op_token) = self.consume_if(|t| matches!(t.kind, TokenKind::Operator(Operator::Not))) {
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: Operator::Not,
                    expr: Box::new(expr),
                })
            } else {
                self.parse_atom()
            }
        } else {
            self.parse_atom()
        }
    }

    fn parse_atom(&mut self) -> ParseResult<Expr> {
        let token = self.expect_next("parse_atom: expected a token, found none")?;
        let mut expr = match token.kind {
            TokenKind::BooleanLiteral(bl) => Expr::Boolean(bl),
            TokenKind::NumberLiteral(nl) => Expr::Number(nl),
            TokenKind::CharLiteral(c) => Expr::Char(c),
            TokenKind::Identifier(id) => Expr::Ident(id),
            TokenKind::Punctuation(Punctuation::OpenParen) => {
                let inner_expr = self.parse_expression()?;
                let _close = self.expect_token(
                    |t| matches!(t.kind, TokenKind::Punctuation(Punctuation::CloseParen)),
                    "parse_atom: expected ')'",
                )?;
                Expr::Grouping(Box::new(inner_expr))
            }
            _ => {
                return Err(self.error(
                    &format!("parse_atom: expected an atom, found {:?}", token.kind),
                    token.line,
                    token.column,
                ));
            }
        };

        // Parse function call if '(' follows
        while let Some(token) = self.peek() {
            if matches!(token.kind, TokenKind::Punctuation(Punctuation::OpenParen)) {
                self.next(); // consume '('
                let mut args = Vec::new();
                if let Some(token) = self.peek() {
                    if !matches!(token.kind, TokenKind::Punctuation(Punctuation::CloseParen)) {
                        loop {
                            args.push(self.parse_expression()?);
                            if let Some(token) = self.peek() {
                                if matches!(token.kind, TokenKind::Punctuation(Punctuation::Comma))
                                {
                                    self.next(); // consume ','
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
                self.expect_token(
                    |t| matches!(t.kind, TokenKind::Punctuation(Punctuation::CloseParen)),
                    "parse_atom: expected ')' after function call arguments",
                )?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_function_declaration(&mut self) -> ParseResult<Function> {
        self.expect_token(
            |t| matches!(t.kind, TokenKind::Keyword(Keyword::Func)),
            "parse_func_decl: expected 'func' keyword",
        )?;

        // Function name
        let name_token = self.expect_token(
            |t| matches!(t.kind, TokenKind::Identifier(_)),
            "parse_func_decl: expected function name",
        )?;
        let name = if let TokenKind::Identifier(n) = name_token.kind {
            n
        } else {
            unreachable!()
        };

        // Parameters
        self.expect_token(
            |t| matches!(t.kind, TokenKind::Punctuation(Punctuation::OpenParen)),
            "parse_func_decl: expected '('",
        )?;
        let mut args = Vec::new();
        while let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::Punctuation(Punctuation::CloseParen) => {
                    self.next();
                    break;
                }
                TokenKind::Identifier(_) => {
                    let param_name_token = self.expect_token(
                        |t| matches!(t.kind, TokenKind::Identifier(_)),
                        "parse_func_decl: expected parameter name",
                    )?;
                    let argument_name = if let TokenKind::Identifier(n) = param_name_token.kind {
                        n
                    } else {
                        unreachable!()
                    };

                    let param_type_token = self.expect_token(
                        |t| matches!(t.kind, TokenKind::TypeIdentifier(_)),
                        "parse_func_decl: expected parameter type",
                    )?;
                    let argument_type = if let TokenKind::TypeIdentifier(t) = param_type_token.kind
                    {
                        t
                    } else {
                        unreachable!()
                    };

                    args.push(FunctionParameter {
                        parameter_name: argument_name,
                        parameter_type: argument_type,
                    });

                    // Optional comma
                    self.consume_if(|t| {
                        matches!(t.kind, TokenKind::Punctuation(Punctuation::Comma))
                    });
                }
                _ => {
                    let line = token.line;
                    let column = token.column;
                    return Err(self.error(
                        "parse_func_decl: unexpected token in parameter list",
                        line,
                        column,
                    ));
                }
            }
        }

        // Optional return type
        let return_type = if let Some(ret_type_token) = self.consume_if(|t| matches!(t.kind, TokenKind::TypeIdentifier(_))) {
            if let TokenKind::TypeIdentifier(t) = ret_type_token.kind {
                Some(t)
            } else {
                None
            }
        } else {
            None
        };

        // Function body (use shared parse_body)
        let body = self.parse_body()?;

        Ok(Function {
            signature: FunctionSignature {
                name,
                parameters: args,
                return_type,
            },
            body: FunctionBody::UserDefinedBody(body),
        })
    }

    /// Parse a block body: expects '{' then parses statements until matching '}'.
    fn parse_body(&mut self) -> ParseResult<Vec<Statement>> {
        self.expect_token(
            |t| matches!(t.kind, TokenKind::Punctuation(Punctuation::OpenCurly)),
            "parse_body: expected '{'",
        )?;

        let mut stmts = Vec::new();
        while let Some(token) = self.peek() {
            if matches!(token.kind, TokenKind::Punctuation(Punctuation::CloseCurly)) {
                self.next();
                break;
            }
            stmts.push(self.parse_statement()?);
        }
        Ok(stmts)
    }
}
