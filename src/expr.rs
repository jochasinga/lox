use crate::lexer::Token;

pub trait Visitor<R> {
    fn visit_binary_expr(&mut self, expr: &Expr) -> R;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> R;
    fn visit_literal_expr(&mut self, expr: &Expr) -> R;
    fn visit_unary_expr(&mut self, expr: &Expr) -> R;

    fn print(&mut self, expr: &Expr) -> R;
    fn parenthesize(&mut self, name: &str, exprs: Vec<&Expr>) -> R;
}

#[derive(Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Option<Token>),
    Unary(Token, Box<Expr>),
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        use Expr::*;
        match self.clone() {
            Binary(left, operator, right) => {
                visitor.visit_binary_expr(&Binary(left, operator, right))
            }

            Grouping(expression) => visitor.visit_grouping_expr(&Grouping(expression)),

            Literal(value) => visitor.visit_literal_expr(&Literal(value)),

            Unary(operator, right) => visitor.visit_unary_expr(&Unary(operator, right)),
        }
    }
}

struct AstPrinter;
impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Binary(left, operator, right) = expr {
            self.parenthesize(&operator.lexeme, vec![left, right])
        } else {
            panic!("Expected Binary expression");
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Grouping(expression) = expr {
            self.parenthesize("group", vec![expression])
        } else {
            panic!("Expected Grouping expression");
        }
    }

    fn visit_literal_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Literal(value) = expr {
            match value {
                Some(token) => token.lexeme.to_string(),
                None => "nil".to_string(),
            }
        } else {
            panic!("Expected Literal expression");
        }
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> String {
        if let Expr::Unary(operator, right) = expr {
            self.parenthesize(&operator.lexeme, vec![right])
        } else {
            panic!("Expected Unary expression");
        }
    }

    fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self as &mut dyn Visitor<String>)
    }

    fn parenthesize(&mut self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut builder = String::new();
        builder.push_str("(");
        builder.push_str(name);
        for expr in exprs {
            builder.push_str(" ");
            builder.push_str(&expr.accept(self as &mut dyn Visitor<String>));
        }
        builder.push_str(")");
        builder
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Token, TokenType};

    use super::*;

    #[test]
    fn test_pretty_printer() {
        let left = Expr::Unary(
            Token::new(TokenType::Minus, "-".to_string(), 1),
            Box::new(Expr::Literal(Some(Token::new(
                TokenType::Number(123.0),
                "123".to_string(),
                1,
            )))),
        );

        let operator = Token::new(TokenType::Star, "*".to_string(), 1);

        let right = Expr::Grouping(Box::new(Expr::Literal(Some(Token::new(
            TokenType::Number(45.67),
            "45.67".to_string(),
            1,
        )))));

        let expression = Expr::Binary(Box::new(left), operator, Box::new(right));

        let s = AstPrinter.print(&expression);
        assert_eq!(s, "(* (- 123) (group 45.67))".to_string());
    }
}
