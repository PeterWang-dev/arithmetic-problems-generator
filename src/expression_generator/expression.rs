use super::error::ErrorKind;
use super::operand::Operand;
use rand::Rng;
use std::fmt::Display;

#[derive(Debug)]
pub struct Expression {
    left: Operand,
    right: Operand,
    operator: char,
}

// TODO: This function may be splitted into two functions
//      moved to a seperate module called constraint.rs.
//      As two checks are included in this function:
//      1. Check if the expression obays arithmatic rules.
//      2. Check if the expression obey user defined constraints like non-negative result.
fn arithmatic_check(left: u32, operator: char, right: u32) -> Result<(), ErrorKind> {
    // Arithmatic rules:
    match operator {
        '+' => Ok(()), // 1. '+' is always valid
        '-' => {
            // 2. '-' is valid if left >= right
            if left >= right {
                Ok(())
            } else {
                Err(ErrorKind::UnsupportExpression) // Negative result, u32 cannot be negative.
            }
        }
        '*' => Ok(()), // 3. '*' is always valid
        '/' => {
            // 4. '/' is valid if right is not zero and left can be divided by right without remainder
            if right == 0 {
                return Err(ErrorKind::ArithmaticViolation); // Divided by zero.
            }

            if left % right == 0 {
                // Fractional result, u32 cannot be fractional.
                Ok(())
            } else {
                Err(ErrorKind::UnsupportExpression)
            }
        }
        _ => panic!("Invalid operator: {}", operator),
    }
}

impl Expression {
    pub fn new_unchecked(left: Operand, operator: char, right: Operand) -> Self {
        Self {
            left,
            right,
            operator,
        }
    }

    // TODO: a new() or from that checks if inputs obay the constraints and propagate the error
    //      and if expression duplicate

    pub fn from_u32(left: u32, operator: char, right: u32) -> Result<Self, ErrorKind> {
        // Check if inputs obay the constraints and propagate the error.
        arithmatic_check(left, operator, right)?;

        // TODO: Check if expression duplicates.

        Ok(Self::new_unchecked(
            Operand::Unit(left),
            operator,
            Operand::Unit(right),
        ))
    }

    // TODO: Only support simple expressions with one operator.
    //      Support more complex expressions.
    pub fn from_range(range: (u32, u32)) -> Self {
        let (min, max) = range;

        // WARNING: If unfortunate, this loop will consume a lot of time and memory.
        loop {
            let operator = match rand::thread_rng().gen_range(0..4) {
                0 => '+',
                1 => '-',
                2 => '*',
                3 => '/',
                _ => unreachable!(),
            };

            let left = rand::thread_rng().gen_range(min..=max);
            let right = rand::thread_rng().gen_range(min..=max);

            match Self::from_u32(left, operator, right) {
                Ok(expr) => return expr,
                Err(_) => continue,
            }
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // WARNING: A recursive call to Display::fmt() may cause a stack overflow or infinite loop.
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        // WARNING: This is not a complete check for duplicate expressions.
        //      It only checks if the two expressions are the same.
        self.left == other.left && self.right == other.right && self.operator == other.operator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_new_unchecked() {
        // Test expression with two Operand::Units
        let expression = Expression::new_unchecked(Operand::Unit(1), '+', Operand::Unit(2));
        assert_eq!(expression.left, Operand::Unit(1));
        assert_eq!(expression.operator, '+');
        assert_eq!(expression.right, Operand::Unit(2));

        // Test expression with one Operand::Unit and one Operand::Expression
        let expression = Expression::new_unchecked(
            Operand::Unit(1),
            '+',
            Operand::Expression(Box::new(Expression::new_unchecked(
                Operand::Unit(2),
                '-',
                Operand::Unit(3),
            ))),
        );
        assert_eq!(expression.left, Operand::Unit(1));
        assert_eq!(expression.operator, '+');
        assert_eq!(
            expression.right,
            Operand::Expression(Box::new(Expression::new_unchecked(
                Operand::Unit(2),
                '-',
                Operand::Unit(3)
            )))
        );

        // Test expression with one Operand::Expressions and one Operand::Unit
        let expression = Expression::new_unchecked(
            Operand::Expression(Box::new(Expression::new_unchecked(
                Operand::Unit(1),
                '+',
                Operand::Unit(2),
            ))),
            '-',
            Operand::Unit(3),
        );
        assert_eq!(
            expression.left,
            Operand::Expression(Box::new(Expression::new_unchecked(
                Operand::Unit(1),
                '+',
                Operand::Unit(2)
            )))
        );
        assert_eq!(expression.operator, '-');
        assert_eq!(expression.right, Operand::Unit(3));

        // Test expression with two Operand::Expressions
        let expression = Expression::new_unchecked(
            Operand::Expression(Box::new(Expression::new_unchecked(
                Operand::Unit(1),
                '+',
                Operand::Unit(2),
            ))),
            '-',
            Operand::Expression(Box::new(Expression::new_unchecked(
                Operand::Unit(3),
                '*',
                Operand::Unit(4),
            ))),
        );
        assert_eq!(
            expression.left,
            Operand::Expression(Box::new(Expression::new_unchecked(
                Operand::Unit(1),
                '+',
                Operand::Unit(2)
            )))
        );
        assert_eq!(expression.operator, '-');
        assert_eq!(
            expression.right,
            Operand::Expression(Box::new(Expression::new_unchecked(
                Operand::Unit(3),
                '*',
                Operand::Unit(4)
            )))
        );
    }

    #[test]
    fn test_expression_from_u32() {
        // Test valid expression
        let expression = Expression::from_u32(1, '+', 2).unwrap();
        assert_eq!(expression.left, Operand::Unit(1));
        assert_eq!(expression.right, Operand::Unit(2));
        assert_eq!(expression.operator, '+');

        // TODO: Test valid complex expressions

        // Test invalid expression
        let expression = Expression::from_u32(1, '/', 2);
        assert!(expression.is_err());
        let expression = Expression::from_u32(1, '-', 2);
        assert!(expression.is_err());
        let expression = Expression::from_u32(1, '/', 0);
        assert!(expression.is_err());

        // TODO: Test invalid complex expressions
    }

    fn expression_valid_check(expression: &Expression) -> bool {
        // There are two possible cases of a randomized expression:
        //  1. expression with two units or
        //  2. expression with one unit and one expression.
        expression.left.is_unit() && expression.right.is_unit()
            || expression.left.is_unit() && expression.right.is_expression()
            || expression.left.is_expression() && expression.right.is_unit()
    }

    fn operand_range_check(operand: &Operand, range: (u32, u32)) -> bool {
        let (min, max) = range;

        match operand {
            Operand::Unit(num) => num >= &min && num <= &max,
            Operand::Expression(expr) => {
                // WARNING: Check both operands recursively
                operand_range_check(&expr.left, range) && operand_range_check(&expr.right, range)
            }
        }
    }

    #[test]
    fn test_expression_from_range() {
        let range: (u32, u32) = (0, 10);
        let expression = Expression::from_range(range);

        // Check if expression is valid
        assert!(expression_valid_check(&expression));

        // Check if operands are in range
        assert!(operand_range_check(&expression.left, range));
        assert!(operand_range_check(&expression.right, range));

        // TODO: Check if operands obay constraints
        // assert!(arithmatic_check(&expression.left));
        // assert!(arithmatic_check(&expression.right));

        // TODO: Check complex expressions
    }

    #[test]
    fn test_expression_eq() {
        // Test expression with two Operand::Units
        let expression_1 = Expression::new_unchecked(Operand::Unit(1), '+', Operand::Unit(1));
        let expression_2 = Expression::new_unchecked(Operand::Unit(1), '+', Operand::Unit(1));
        assert_eq!(expression_1, expression_2);

        let expression_2 = Expression::new_unchecked(Operand::Unit(1), '+', Operand::Unit(2));
        assert_ne!(expression_1, expression_2);

        // TODO:
        //      1. Test expression with one Operand::Unit and one Operand::Expression
        //      2. Test expression with one Operand::Expressions and one Operand::Unit
        //      3. Test expression with two Operand::Expressions

        // TODO: Test duplicate expressions
    }

    #[test]
    fn test_expression_display() {
        // Test expression with two Operand::Units
        let expression = Expression::new_unchecked(Operand::Unit(1), '+', Operand::Unit(2));
        assert_eq!(format!("{}", expression), "1 + 2");

        // Test expression with one Operand::Unit and one Operand::Expression
        let expression = Expression::new_unchecked(
            Operand::Unit(1),
            '*',
            Operand::Expression(Box::new(Expression::new_unchecked(
                Operand::Unit(2),
                '+',
                Operand::Unit(3),
            ))),
        );
        assert_eq!(format!("{}", expression), "1 * ( 2 + 3 )");

        // Test expression with one Operand::Expressions and one Operand::Unit
        let expression = Expression::new_unchecked(
            Operand::Expression(Box::new(Expression::new_unchecked(
                Operand::Unit(1),
                '+',
                Operand::Unit(2),
            ))),
            '/',
            Operand::Unit(3),
        );
        assert_eq!(format!("{}", expression), "( 1 + 2 ) / 3");

        // Test expression with two Operand::Expressions
        let expression = Expression::new_unchecked(
            Operand::Expression(Box::new(Expression::new_unchecked(
                Operand::Unit(1),
                '+',
                Operand::Unit(2),
            ))),
            '-',
            Operand::Expression(Box::new(Expression::new_unchecked(
                Operand::Unit(3),
                '*',
                Operand::Unit(4),
            ))),
        );
        assert_eq!(format!("{}", expression), "( 1 + 2 ) - ( 3 * 4 )");
    }
}
