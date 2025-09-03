use std::iter::Peekable;

use crate::token::{Keyword, Operator, Token, TokenKind};

#[derive(Debug)]
pub struct Ast {
    statements: Vec<Statement>,
}

impl Ast {
    pub fn get_stmts(self) -> Vec<Statement> {
        self.statements
    }
}

#[derive(Debug)]
pub enum Statement {
    VarDecl(VarDecl),
    Expr(Expr),
    Comment,
}

#[derive(Debug)]
pub struct VarDecl {
    pub identifier: String,
    pub expr: Expr,
}

    fn new(identifier: String, expr: Expr) -> Self {
        Self { identifier, expr }
impl VarDecl {
    }
}

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    Number(i32),
    Boolean(bool),
    Ident(String),
    Grouping(Box<Expr>),
}

impl Ast {
    pub fn new() -> Self {
        return Self {
            statements: Vec::new(),
        };
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekable<I>,
}
type ParseResult<T> = Result<T, ParseError>;

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(tokens: I) -> Self {
        return Self {
            tokens: tokens.peekable(),
        };
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub fn parse_program(&mut self) -> ParseResult<Ast> {
        let mut ast = Ast::new();
        while let Some(_) = self.peek() {
            let statement = self.parse_statement();
            ast.statements.push(statement?);
        }
        Ok(ast)
    }

    fn parse_statement(&mut self) -> ParseResult<Statement> {
        if let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::Keyword(t) => match t {
                    Keyword::Var => {
                        let stmt = self.parse_var_decl()?;
                        return Ok(Statement::VarDecl(stmt));
                    }
                    _ => {
                        return Err(ParseError {
                            message: "parse_statement: keyword not supported yet.".to_string(),
                            line: token.line,
                            column: token.column,
                        });
                    }
                },
                TokenKind::Comment => {
                    self.next();
                    Ok(Statement::Comment)
                }
                _ => {
                    let expr = self.parse_expression()?;
                    return Ok(Statement::Expr(expr));
                }
            }
        } else {
            return Err(ParseError {
                message: "parse_statement: expected a token, found none".to_string(),
                // TODO: need to get token but there is no next token
                line: 0,
                column: 0,
            });
        }
    }

    fn parse_var_decl(&mut self) -> ParseResult<VarDecl> {
        // we get here because a let was found so we can bump
        self.next();
        // then we expect an identifier
        let ident = if let Some(token) = self.next() {
            match token.kind {
                TokenKind::Identifier(ident) => ident,
                _ => {
                    return Err(ParseError {
                        message: "parse_var_decl: unexpected token".to_string(),
                        line: token.line,
                        column: token.column,
                    });
                }
            }
        } else {
            return Err(ParseError {
                message: "parse_var_decl: expected a token, found none".to_string(),
                // TODO: need to get token but there is no next token
                line: 0,
                column: 0,
            });
        };
                        line: token.line,
                        column: token.column,
                    });
                }
            }
        } else {
            return Err(ParseError {
                message: "parse_var_decl: expected a token, found none".to_string(),
                // TODO: need to get token but there is no next token
                line: 0,
                column: 0,
            });
        };
        // then we expect an =
        if let Some(token) = self.next() {
            match token.kind {
                TokenKind::Operator(op) => match op {
                    Operator::Assign => {}
                    _ => {
                        return Err(ParseError {
                            message: "parse_var_decl: expected assignment".to_string(),
                            line: token.line,
                            column: token.column,
                        });
                    }
                },
                _ => {
                    return Err(ParseError {
                        message: "parse_var_decl: expected an =".to_string(),
                        line: token.line,
                        column: token.column,
                    });
                }
            }
        }
        let expr = self.parse_expression()?;
        let letdecl = VarDecl::new(ident, var_type, expr);
        Ok(letdecl)
    }

    fn parse_expression(&mut self) -> ParseResult<Expr> {
        // first parse left term
        let left = Box::new(self.parse_term()?);
        // then we expect a + or a -
        // we expect some token
        if let Some(token) = self.peek() {
            match token.kind {
                TokenKind::Operator(op) => {
                    if let Operator::Plus | Operator::Minus = op {
                        self.next();
                        // then the right
                        let right = Box::new(self.parse_expression()?);
                        return Ok(Expr::Binary { left, op, right });
                    }
                }
                _ => {}
            };
        }
        Ok(*left)
    }

    fn parse_term(&mut self) -> ParseResult<Expr> {
        // we first expect an atom
        let left = Box::new(self.parse_atom()?);

        // then we expect a * or /
        // we expect some token
        if let Some(token) = self.peek() {
            match token.kind {
                TokenKind::Operator(op) => {
                    if let Operator::Multply | Operator::Divide = op {
                        self.next();
                        // then a right term
                        let right = Box::new(self.parse_expression()?);
                        return Ok(Expr::Binary { left, op, right });
                    }
                }
                _ => {}
            };
        }
        Ok(*left)
    }

    fn parse_atom(&mut self) -> ParseResult<Expr> {
        if let Some(token) = self.next() {
            match token.kind {
                TokenKind::BooleanLiteral(bl) => {
                    return Ok(Expr::Boolean(bl));
                }
                TokenKind::NumberLiteral(nl) => {
                    return Ok(Expr::Number(nl));
                }
                TokenKind::Identifier(id) => {
                    return Ok(Expr::Ident(id));
                }
                _ => {
                    let msg = format!("parse_atom: expected an atom, found {:?}", token);
                    return Err(ParseError {
                        message: msg.to_string(),
                        line: token.line,
                        column: token.column,
                    });
                }
            }
        } else {
            return Err(ParseError {
                message: "parse_atom: expected a token, found none".to_string(),
                // TODO: need to get token but there is no next token
                line: 0,
                column: 0,
            });
        };
    }
}
