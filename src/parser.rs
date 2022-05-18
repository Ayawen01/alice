use crate::{token::{Token, TokenType, Literal}, error::AliceError, ast::{Stmt, Expr, AliceObject}};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    #[inline]
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    #[inline]
    pub fn parse(&mut self) -> Result<Vec<Stmt>, AliceError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => return Err(e)
            }
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, AliceError> {
        if self.matches(&[TokenType::Let]) {
            return self.var_declaration();
        }

        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, AliceError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let mut initializer: Option<Expr> = None;

        if self.matches(&[TokenType::Equal]) {
            initializer = match self.expression() {
                Ok(expr) => Some(expr),
                Err(e) => return Err(e)
            };
        }

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var { name, initializer })
    }

    #[inline]
    fn expression(&mut self) -> Result<Expr, AliceError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, AliceError> {
        let expr = self.or()?;

        if self.matches(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            match expr {
                Expr::Variable { name } => return Ok(Expr::Assign { name, value: Box::new(value) }),
                _ => return Err(AliceError::ParseError("Invalid assignment target.".into() ,equals.line))
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, AliceError> {
        let mut expr = self.and()?;

        while self.matches(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical { left: Box::new(expr), operator, right: Box::new(right) }
        }
        
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, AliceError> {
        let mut expr = self.equality()?;

        while self.matches(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical { left: Box::new(expr), operator, right: Box::new(right) }
        }
        
        Ok(expr)
    }

    fn statement(&mut self) -> Result<Stmt, AliceError> {
        if self.matches(&[TokenType::If]) {
            self.if_statement()
        }
        else if self.matches(&[TokenType::For]) {
            self.for_statement()
        }
        else if self.matches(&[TokenType::Println]) {
            self.print_statement()
        }
        else if self.matches(&[TokenType::LeftBrace]) {
            let statements = self.block()?;
            Ok(Stmt::Block { statements })
        }
        else {
            self.expression_statement()
        }
    }

    fn if_statement(&mut self) -> Result<Stmt, AliceError> {
        //self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        //self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        
        let mut else_branch: Option<Box<Stmt>> = None;
        if self.matches(&[TokenType::Else]) {
            else_branch = match self.statement() {
                Ok(stmt) => Some(Box::new(stmt)),
                Err(e) => return Err(e)
            };
        }

        Ok(Stmt::If { condition, then_branch, else_branch })
    }

    fn for_statement(&mut self) -> Result<Stmt, AliceError> {
        let value = if let Ok(Expr::Variable { name }) = self.expression() {
            name
        } else {
            return Err(AliceError::ParseError("Expect variable.".into(), self.peek().line))
        };

        self.consume(TokenType::In, "Expext 'in' after variable.")?;

        let expr = self.expression();

        let expression = if let Ok(Expr::Array { value }) = expr {
            Expr::Array { value }
        } else if let Ok(Expr::Variable { name }) = expr {
            Expr::Variable { name }
        } else {
            return Err(AliceError::ParseError("Expect [...].".into(), self.peek().line))
        };

        if self.matches(&[TokenType::LeftBrace]) {
            let block = self.block()?;
            Ok(Stmt::For { value, expression, body: block }) 
        } else {
            return Err(AliceError::ParseError("Expect '{' after [...].".into(), self.peek().line));
        }
    }

    fn block(&mut self) -> Result<Vec<Stmt>, AliceError> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Stmt, AliceError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Println { expression: expr })
    }

    fn expression_statement(&mut self) -> Result<Stmt, AliceError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression { expression: expr })
    }

    fn equality(&mut self) -> Result<Expr, AliceError> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, AliceError> {
        let mut expr = self.term()?;

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, AliceError> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, AliceError> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, AliceError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                value: Box::new(right)
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, AliceError> {
        if self.matches(&[TokenType::False]) {
            return Ok(Expr::Literal {
                value: AliceObject::Boolean(false)
            });
        }
        if self.matches(&[TokenType::True]) {
            return Ok(Expr::Literal {
                value: AliceObject::Boolean(true)
            });
        }
        if self.matches(&[TokenType::Nil]) {
            return Ok(Expr::Literal {
                value: AliceObject::Nil
            });
        }

        if self.matches(&[TokenType::Identifier]) {
            return Ok(Expr::Variable { 
                name: self.previous()
            });
        }

        if self.matches(&[TokenType::String, TokenType::F64, TokenType::I64]) {
            let literal = self.previous().literal.unwrap();

            if let Literal::String(str) = literal {
                return Ok(Expr::Literal {
                    value: AliceObject::String(str)
                });
            }
            
            if let Literal::F64(num) = literal {
                return Ok(Expr::Literal { 
                    value: AliceObject::F64(num)
                });
            }

            if let Literal::I64(num) = literal {
                return Ok(Expr::Literal { 
                    value: AliceObject::I64(num)
                });
            }
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            if let Err(e) = self.consume(TokenType::RightParen, "Expect ')' after expression.") {
                return Err(e);
            };
            return Ok(Expr::Grouping {
                expression: Box::new(expr)
            });
        }

        if self.matches(&[TokenType::LeftSquare]) {
            let mut list = Vec::new();

            if self.matches(&[TokenType::RightSquare]) {
                return Ok(Expr::Array { value: list })
            }

            while !self.matches(&[TokenType::RightSquare]) {
                let expr = self.expression()?;
                if let Err(e) = self.consume(TokenType::Comma, "Expect ',' after expression.") {
                    if self.peek().r#type != TokenType::RightSquare {
                        return Err(e);
                    }
                };
                list.push(expr);
            }

            return Ok(Expr::Array { value: list })
        }

        Err(AliceError::ParseError("Expect expression.".into(), self.peek().line))
    }
}

impl Parser {
    fn matches(&mut self, types: &[TokenType]) -> bool {
        for &t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, t: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().r#type == t
    }

    #[inline]
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.peek().r#type == TokenType::Eof
    }

    #[inline]
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn consume<'a>(&'a mut self, t: TokenType, msg: &'a str) -> Result<Token, AliceError> {
        if self.check(t) {
            return Ok(self.advance());
        }
        Err(AliceError::ParseError(msg.into(), self.peek().line))
    }
}