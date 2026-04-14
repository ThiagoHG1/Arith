use crate::lexe;

pub enum Expr {
    Number(i32),
    Add(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),
    Rest(Box<Expr>, Box<Expr>),
}

pub fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Minus(a, b) => eval(a) - eval(b),
        Expr::Multiply(a, b) => eval(a) * eval(b),
        Expr::Division(a, b) => eval(a) / eval(b),
        Expr::Rest(a, b) => eval(a) % eval(b),
    }
}

pub fn parse(tokens: &[lexe::Token]) -> Expr {
    let mut pos: usize = 0;
    return parse_expr(tokens, &mut pos);
}

fn parse_expr(tokens: &[lexe::Token], pos: &mut usize) -> Expr {
    let mut left  = parse_term(tokens, pos);

    while *pos < tokens.len() {
        match tokens[*pos] {
            lexe::Token::Plus => {
                *pos += 1;
                let right = parse_term(tokens, pos);
                left = Expr::Add(Box::new(left), Box::new(right));
            },

            lexe::Token::Minus => {
                *pos += 1;
                let right = parse_term(tokens, pos);
                left = Expr::Minus(Box::new(left), Box::new(right));
            },

            _ => break,
        }
    }
    left
}

fn parse_term(tokens: &[lexe::Token], pos: &mut usize) -> Expr {
    let mut left = parse_factor(tokens, pos);

    while *pos < tokens.len() {
        match tokens[*pos] {
            lexe::Token::Multiply => {
                *pos += 1;
                let right = parse_factor(tokens, pos);
                left = Expr::Multiply(Box::new(left), Box::new(right));
            },

            lexe::Token::Division => {
                *pos += 1;
                let right = parse_factor(tokens, pos);
                left = Expr::Division(Box::new(left), Box::new(right));
            },

            lexe::Token::Rest => {
                *pos += 1;
                let right = parse_factor(tokens, pos);
                left = Expr::Rest(Box::new(left), Box::new(right));
            },

            _ => break,
        }
    }
    left
}

fn parse_factor(tokens: &[lexe::Token], pos: &mut usize) -> Expr {
    match tokens[*pos] {
        lexe::Token::Minus => {
            *pos += 1;
            if *pos < tokens.len() {
                if let lexe::Token::Minus = tokens[*pos] {
                    *pos += 1;
                    return parse_factor(tokens, pos);
                }
            }

            let expr = parse_factor(tokens, pos);
            Expr::Multiply(Box::new(Expr::Number(-1)), Box::new(expr))
        },

        lexe::Token::Plus => {
            *pos += 1;
            let expr = parse_factor(tokens, pos);
            expr
        },

        lexe::Token::Number(n) => {
            *pos += 1;
            Expr::Number(n)
        },

        lexe::Token::ParenthesisOpen => {
            *pos += 1;
            let expr = parse_expr(tokens, pos);
            *pos += 1;
            expr
        },

        _ => panic!("Token invalido no factor"),
    }
}




