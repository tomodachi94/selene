use full_moon::ast::Expression;

pub fn strip_parentheses(expression: &Expression) -> &Expression {
    match expression {
        Expression::Parentheses { expression, .. } => expression,
        _ => expression,
    }
}
