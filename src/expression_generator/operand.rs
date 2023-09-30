use super::expression::Expression; // Warning: A recursive dependency exists.
use std::fmt::Display;

#[derive(Debug)]
pub enum Operand {
    Unit(u32),
    Expression(Box<Expression>),
}

impl Operand {
    pub fn is_unit(&self) -> bool {
        match self {
            Operand::Unit(_) => true,
            _ => false,
        }
    }

    pub fn is_expression(&self) -> bool {
        match self {
            Operand::Expression(_) => true,
            _ => false,
        }
    }
}

impl PartialEq for Operand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Operand::Unit(a), Operand::Unit(b)) => a == b,
            // WARNING: A recursive call to PartialEq::eq() may cause a stack overflow or infinite loop.
            (Operand::Expression(a), Operand::Expression(b)) => a == b,
            _ => panic!("Can not compare Operand::Unit with Operand::Expression."),
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Unit(num) => write!(f, "{}", num),
            // WARNING: A recursive call to Display::fmt() may cause a stack overflow or infinite loop.
            Operand::Expression(expr) => write!(f, "( {} )", expr),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_operand_is_unit() {
        // Test Operand::Unit
        let operand = Operand::Unit(1);
        assert!(operand.is_unit());

        // Test Operand::Expression
        let expression = Expression::new_unchecked(Operand::Unit(1), '+', Operand::Unit(1));
        let operand = Operand::Expression(Box::new(expression));
        assert!(!operand.is_unit());

        // Test Operand::Expression with nested expression
        let expression = Expression::new_unchecked(
            Operand::Unit(1),
            '+',
            Operand::Expression(Box::new(Expression::new_unchecked(
                Operand::Unit(1),
                '+',
                Operand::Unit(1),
            ))),
        );
        let operand = Operand::Expression(Box::new(expression));
        assert!(!operand.is_unit());
    }

    #[test]
    fn test_operand_is_expression() {
        // Test Operand::Unit
        let operand = Operand::Unit(1);
        assert!(!operand.is_expression());

        // Test Operand::Expression
        let expression = Expression::new_unchecked(Operand::Unit(1), '+', Operand::Unit(1));
        let operand = Operand::Expression(Box::new(expression));
        assert!(operand.is_expression());

        // Test Operand::Expression with nested expression
        let expression = Expression::new_unchecked(
            Operand::Unit(1),
            '+',
            Operand::Expression(Box::new(Expression::new_unchecked(
                Operand::Unit(1),
                '+',
                Operand::Unit(1),
            ))),
        );
        let operand = Operand::Expression(Box::new(expression));
        assert!(operand.is_expression());
    }

    #[test]
    #[should_panic(expected = "Can not compare Operand::Unit with Operand::Expression.")]
    fn test_operand_eq() {
        // Test Operand::Unit
        let operand_1 = Operand::Unit(1);
        let operand_2 = Operand::Unit(1);
        assert_eq!(operand_1, operand_2);

        let operand_2 = Operand::Unit(2);
        assert_ne!(operand_1, operand_2);

        // Test Operand::Expression
        let expression_1 = Expression::new_unchecked(Operand::Unit(1), '+', Operand::Unit(1));
        let expression_2 = Expression::new_unchecked(Operand::Unit(1), '+', Operand::Unit(1));
        let operand_1 = Operand::Expression(Box::new(expression_1));
        let operand_2 = Operand::Expression(Box::new(expression_2));
        assert_eq!(operand_1, operand_2);

        let expression_2 = Expression::new_unchecked(Operand::Unit(1), '+', Operand::Unit(2));
        let operand_2 = Operand::Expression(Box::new(expression_2));
        assert_ne!(operand_1, operand_2);

        // Test Operand::Unit and Operand::Expression
        let operand_1 = Operand::Unit(1);
        let operand_2 = Operand::Expression(Box::new(Expression::new_unchecked(
            Operand::Unit(2),
            '+',
            Operand::Unit(3),
        )));
        assert_eq!(operand_1, operand_2) // This gonna panic
    }

    #[test]
    fn test_operand_display() {
        // Test Operand::Unit
        let operand = Operand::Unit(1);
        assert_eq!(format!("{}", operand), "1");

        // Test Operand::Expression
        let expression = Expression::new_unchecked(Operand::Unit(1), '+', Operand::Unit(1));
        let operand = Operand::Expression(Box::new(expression));
        assert_eq!(format!("{}", operand), "( 1 + 1 )");

        // Test Operand::Expression with nested expression
        let operand_1 = Operand::Unit(1);
        let operand_2 = Operand::Expression(Box::new(Expression::new_unchecked(
            Operand::Unit(2),
            '+',
            Operand::Unit(3),
        )));
        let expression = Expression::new_unchecked(operand_1, '*', operand_2);
        let operand = Operand::Expression(Box::new(expression));
        assert_eq!(format!("{}", operand), "( 1 * ( 2 + 3 ) )");
    }
}
