use alloc::rc::Rc;
use core::ops::{Add, Sub};
use core::borrow::Borrow;
use crate::renderer::js::ast::{Node, Program};

#[derive(Debug, Clone)]
pub struct JsRuntime {}

impl JsRuntime {
    pub fn new() -> JsRuntime {
        Self {}
    }

    pub fn execute(&mut self, program: &Program) {
        for node in program.body() {
            self.eval(&Some(node.clone()));
        }
    }

    fn eval(
        &mut self,
        node: &Option<Rc<Node>>
    ) -> Option<RuntimeValue> {
        let node = match node {
            Some(n) => n,
            None => return None,
        };

        match node.borrow() {
            Node::ExpressionStatement(expr) => self.eval(&expr),
            Node::AdditiveExpression {
                operator,
                left,
                right
            } => {
                let left_value = match self.eval(&left) {
                    Some(value) => value,
                    None => return None,
                };
                let right_value = match self.eval(&right) {
                    Some(value) => value,
                    None => return None,
                };

                if operator == &'+' {
                    Some(left_value + right_value)
                } else if operator == &'-' {
                    Some(left_value - right_value)
                } else {
                    None
                }
            }
            Node::AssignmentExpression {
                operator: _,
                left: _,
                right: _,
            } => {
                None
            }
            Node::MemberExpression {
                object: _,
                property: _,
            } => {
                None
            },
            Node::NumericLiteral(value) => Some(RuntimeValue::Number(*value)),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Number(u64),
}

impl Add<RuntimeValue> for RuntimeValue {
    type Output = RuntimeValue;

    fn add(self, rhs: RuntimeValue) -> Self::Output {
        let (RuntimeValue::Number(left_num), RuntimeValue::Number(right_num)) = (&self, &rhs);
        RuntimeValue::Number(left_num + right_num)
    }
}

impl Sub<RuntimeValue> for RuntimeValue {
    type Output = RuntimeValue;

    fn sub(self, rhs: RuntimeValue) -> Self::Output {
        let (RuntimeValue::Number(left_num), RuntimeValue::Number(right_num)) = (&self, &rhs);
        RuntimeValue::Number(left_num - right_num)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use crate::renderer::js::ast::JsParser;
    use crate::renderer::js::token::JsLexer;
    use super::*;

    #[test]
    fn test_num() {
        let js = "42".to_string();
        let lexer = JsLexer::new(js);
        let mut parser = JsParser::new(lexer);
        let ast = parser.parse_ast();
        let mut runtime = JsRuntime::new();
        let expected = [Some(RuntimeValue::Number(42))];
        let mut i = 0;

        for node in ast.body() {
            let result = runtime.eval(&Some(node.clone()));
            assert_eq!(expected[i], result);
            i += 1;
        }
    }

    #[test]
    fn test_add_nums() {
        let js = "1 + 2".to_string();
        let lexer = JsLexer::new(js);
        let mut parser = JsParser::new(lexer);
        let ast = parser.parse_ast();
        let mut runtime = JsRuntime::new();
        let expected = [Some(RuntimeValue::Number(3))];
        let mut i = 0;

        for node in ast.body() {
            let result = runtime.eval(&Some(node.clone()));
            assert_eq!(expected[i], result);
            i += 1;
        }
    }

    #[test]
    fn test_sub_nums() {
        let js = "2 - 1".to_string();
        let lexer = JsLexer::new(js);
        let mut parser = JsParser::new(lexer);
        let ast = parser.parse_ast();
        let mut runtime = JsRuntime::new();
        let expected = [Some(RuntimeValue::Number(1))];
        let mut i = 0;

        for node in ast.body() {
            let result = runtime.eval(&Some(node.clone()));
            assert_eq!(expected[i], result);
            i += 1;
        }
    }
}