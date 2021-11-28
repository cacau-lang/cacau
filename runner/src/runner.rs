use std::io::Write;

use crate::{
    ast::{
        ArithmeticOperation, Assignment, BooleanOperation, CacauProgram, ComparisonOperation,
        Expression, FunctionCall, HighLevelItem,
    },
    mem::{SymbolTable, Value},
};

pub struct Runner<'a> {
    stdout: &'a mut dyn Write,
    symbol_table: SymbolTable,
}

impl<'a> Runner<'a> {
    pub fn run(program: &CacauProgram, stdout: &'a mut dyn Write) {
        let mut runner = Runner {
            stdout,
            symbol_table: SymbolTable::default(),
        };

        for item in program.items.iter() {
            use HighLevelItem::*;
            match item {
                Expr(ref expr) => {
                    runner.eval_expr(expr);
                }
                _ => todo!(),
            }
        }
    }

    fn eval_expr(&mut self, expr: &Expression) -> Value {
        use Expression::*;
        match expr {
            FunctionCall(call) => self.eval_function_call(call),
            IntegerLiteral(integer) => Value::Integer(*integer),
            FloatLiteral(float) => Value::Float(*float),
            BooleanLiteral(boolean) => Value::Boolean(*boolean),
            CharLiteral(char) => Value::Char(*char),
            StringLiteral(string) => Value::String(String::from(*string)),
            Assignment(assign) => self.eval_assignment(assign),
            Identifier(name) => self.eval_identifier(name),
            CompOperation(comp) => self.eval_comparison_oper(comp),
            ArithOperation(arith) => self.eval_arithmetic_oper(arith),
            BoolOperation(boolean) => self.eval_boolean_oper(boolean),
            Not(expr) => eval_not(self.eval_expr(expr)),
            Minus(expr) => eval_minus(self.eval_expr(expr)),
        }
    }

    // TODO assignment returns the assigned value?
    // TODO scope rules
    fn eval_assignment(&mut self, assign: &Assignment) -> Value {
        let val = self.eval_expr(&assign.expression);
        self.symbol_table.create_var(assign.name, val);

        Value::Void
    }

    fn eval_comparison_oper(&mut self, comp: &ComparisonOperation) -> Value {
        use crate::ast::ComparisonOperator::*;

        let left = self.eval_expr(&comp.left);
        let right = self.eval_expr(&comp.right);

        match comp.op {
            Equals => eval_equals(left, right),
            NotEquals => eval_not_equals(left, right),
            Less => eval_less(left, right),
            LessEquals => eval_less_equals(left, right),
            Greater => eval_greater(left, right),
            GreaterEquals => eval_greater_equals(left, right),
        }
    }

    fn eval_arithmetic_oper(&mut self, arith: &ArithmeticOperation) -> Value {
        use crate::ast::ArithmeticOperator::*;

        let left = self.eval_expr(&arith.left);
        let right = self.eval_expr(&arith.right);

        match arith.op {
            Add => eval_add(left, right),
            Subtract => eval_subtract(left, right),
            Multiply => eval_multiply(left, right),
            Divide => eval_divide(left, right),
            Power => eval_power(left, right),
            Modulo => eval_modulo(left, right),
        }
    }

    fn eval_boolean_oper(&mut self, boolean: &BooleanOperation) -> Value {
        use crate::ast::BooleanOperator::*;

        let left = self.eval_expr(&boolean.left);
        let right = self.eval_expr(&boolean.right);

        match boolean.op {
            Or => eval_or(left, right),
            And => eval_and(left, right),
        }
    }

    fn eval_function_call(&mut self, call: &FunctionCall) -> Value {
        if call.name == "println" && call.params.len() == 1 {
            self.eval_println(call)
        } else if call.name == "assert" && call.params.len() == 1 {
            self.eval_assert(call)
        } else {
            panic!(
                "Function {} could not be found or has invalid number of args",
                call.name
            )
        }
    }

    fn eval_assert(&mut self, call: &FunctionCall) -> Value {
        match self.eval_expr(&call.params[0]) {
            Value::Boolean(assert_ok) => {
                if !assert_ok {
                    // TODO panic only the runtime
                    // TODO show expression that failed
                    panic!("Assert failed");
                } else {
                    Value::Void
                }
            }
            _ => Value::Void,
        }
    }

    fn eval_println(&mut self, call: &FunctionCall) -> Value {
        match self.eval_expr(&call.params[0]) {
            Value::String(ref str) => {
                let _bytes_written = self.stdout.write(str.as_bytes()).unwrap();
                let _bytes_written = self.stdout.write(b"\n").unwrap();
            }
            Value::Integer(val) => {
                let str = format!("{}\n", val);
                let _bytes_written = self.stdout.write(str.as_bytes()).unwrap();
            }
            Value::Float(val) => {
                let fmt = format!("{:.5}\n", val);
                let _bytes_written = self.stdout.write(fmt.as_bytes()).unwrap();
            }
            Value::Boolean(val) => {
                let fmt = format!("{}\n", val);
                let _bytes_written = self.stdout.write(fmt.as_bytes()).unwrap();
            }
            Value::Char(val) => {
                let fmt = format!("{}\n", val);
                let _bytes_written = self.stdout.write(fmt.as_bytes()).unwrap();
            }
            _ => todo!(),
        }
        Value::Void
    }

    fn eval_identifier(&self, name: &str) -> Value {
        if let Some(value) = self.symbol_table.get_value(name) {
            value.clone()
        } else {
            // TODO panic inside runtime
            panic!("Could not find {}", name)
        }
    }
}

fn eval_or(left: Value, right: Value) -> Value {
    use crate::mem::Value::Boolean;
    match (&left, &right) {
        (Boolean(val1), Boolean(val2)) => Boolean(*val1 || *val2),
        _ => todo!(
            "Boolean OR not implemented for {:?} and {:?}",
            &left,
            &right
        ),
    }
}

fn eval_and(left: Value, right: Value) -> Value {
    use crate::mem::Value::Boolean;
    match (&left, &right) {
        (Boolean(val1), Boolean(val2)) => Boolean(*val1 && *val2),
        _ => todo!(
            "Boolean AND not implemented for {:?} and {:?}",
            &left,
            &right
        ),
    }
}

fn eval_not(value: Value) -> Value {
    use crate::mem::Value::Boolean;
    match &value {
        Boolean(value) => Boolean(!value),
        _ => todo!("Boolean NOT not implemented for {:?}", &value),
    }
}

fn eval_minus(value: Value) -> Value {
    use crate::mem::Value::*;
    match &value {
        Integer(val) => Integer(-val),
        Float(val) => Float(-val),
        _ => todo!("Unary minus not implemented for {:?}", &value),
    }
}

fn eval_add(left: Value, right: Value) -> Value {
    use crate::mem::Value::*;
    match (&left, &right) {
        (Integer(val1), Integer(val2)) => Integer(val1 + val2),
        (Float(val1), Float(val2)) => Float(val1 + val2),
        (Integer(val1), Float(val2)) => Float(*val1 as f64 + val2),
        (Float(val1), Integer(val2)) => Float(val1 + *val2 as f64),
        (String(val1), String(val2)) => String(val1.to_owned() + val2),
        _ => todo!("Addition of {:?} and {:?} not implemented", &left, &right),
    }
}

fn eval_subtract(left: Value, right: Value) -> Value {
    use crate::mem::Value::*;
    match (&left, &right) {
        (Integer(val1), Integer(val2)) => Integer(val1 - val2),
        (Float(val1), Float(val2)) => Float(val1 - val2),
        (Integer(val1), Float(val2)) => Float(*val1 as f64 - val2),
        (Float(val1), Integer(val2)) => Float(val1 - *val2 as f64),
        _ => todo!(
            "Subtraction of {:?} and {:?} not implemented",
            &left,
            &right
        ),
    }
}

fn eval_multiply(left: Value, right: Value) -> Value {
    use crate::mem::Value::*;
    match (&left, &right) {
        (Integer(val1), Integer(val2)) => Integer(val1 * val2),
        (Float(val1), Float(val2)) => Float(val1 * val2),
        (Integer(val1), Float(val2)) => Float(*val1 as f64 * val2),
        (Float(val1), Integer(val2)) => Float(val1 * *val2 as f64),
        _ => todo!(
            "Multiplication of {:?} and {:?} not implemented",
            &left,
            &right
        ),
    }
}

fn eval_divide(left: Value, right: Value) -> Value {
    use crate::mem::Value::*;
    // TODO check division by zero
    match (&left, &right) {
        (Integer(val1), Integer(val2)) => Integer(val1 / val2),
        (Float(val1), Float(val2)) => Float(val1 / val2),
        (Integer(val1), Float(val2)) => Float(*val1 as f64 / val2),
        (Float(val1), Integer(val2)) => Float(val1 / *val2 as f64),
        _ => todo!("Divisio of {:?} and {:?} not implemented", &left, &right),
    }
}

fn eval_power(left: Value, right: Value) -> Value {
    use crate::mem::Value::*;
    match (&left, &right) {
        // TODO int < 0
        // TODO int > u32
        (Integer(val1), Integer(val2)) => Integer(val1.pow(*val2 as u32)),
        (Float(val1), Float(val2)) => Float(val1.powf(*val2)),
        (Integer(val1), Float(val2)) => Float((*val1 as f64).powf(*val2)),
        (Float(val1), Integer(val2)) => Float(val1.powf(*val2 as f64)),
        _ => todo!(
            "Exponentiation of {:?} by {:?} not implemented",
            &left,
            &right
        ),
    }
}

fn eval_modulo(left: Value, right: Value) -> Value {
    use crate::mem::Value::*;
    // TODO div by zero
    match (&left, &right) {
        (Integer(val1), Integer(val2)) => Integer(val1 % val2),
        (Float(val1), Float(val2)) => Float(val1 % val2),
        (Integer(val1), Float(val2)) => Float(*val1 as f64 % val2),
        (Float(val1), Integer(val2)) => Float(val1 % *val2 as f64),
        _ => todo!("Modulo of {:?} by {:?} not implemented", &left, &right),
    }
}

fn eval_equals(left: Value, right: Value) -> Value {
    use crate::mem::Value::*;
    match (&left, &right) {
        (Integer(val1), Integer(val2)) => Boolean(val1 == val2),
        (String(val1), String(val2)) => Boolean(val1 == val2),
        (Char(val1), Char(val2)) => Boolean(val1 == val2),
        (Float(val1), Float(val2)) =>
        {
            #[allow(clippy::float_cmp)]
            Boolean(val1 == val2)
        }
        (Boolean(val1), Boolean(val2)) => Boolean(val1 == val2),
        _ => todo!("Comparison of {:?} and {:?} not implemented", &left, &right),
    }
}

fn eval_less(left: Value, right: Value) -> Value {
    use crate::mem::Value::*;
    match (&left, &right) {
        (Integer(val1), Integer(val2)) => Boolean(val1 < val2),
        (String(val1), String(val2)) => Boolean(val1 < val2),
        (Char(val1), Char(val2)) => Boolean(val1 < val2),
        (Float(val1), Float(val2)) => Boolean(val1 < val2),
        (Boolean(val1), Boolean(val2)) => Boolean(val1 < val2),
        _ => todo!("Comparison of {:?} and {:?} not implemented", &left, &right),
    }
}

fn eval_less_equals(left: Value, right: Value) -> Value {
    use crate::mem::Value::*;
    match (&left, &right) {
        (Integer(val1), Integer(val2)) => Boolean(val1 <= val2),
        (String(val1), String(val2)) => Boolean(val1 <= val2),
        (Char(val1), Char(val2)) => Boolean(val1 <= val2),
        (Float(val1), Float(val2)) => Boolean(val1 <= val2),
        (Boolean(val1), Boolean(val2)) => Boolean(val1 <= val2),
        _ => todo!("Comparison of {:?} and {:?} not implemented", &left, &right),
    }
}

fn eval_greater(left: Value, right: Value) -> Value {
    use crate::mem::Value::*;
    match (&left, &right) {
        (Integer(val1), Integer(val2)) => Boolean(val1 > val2),
        (String(val1), String(val2)) => Boolean(val1 > val2),
        (Char(val1), Char(val2)) => Boolean(val1 > val2),
        (Float(val1), Float(val2)) => Boolean(val1 > val2),
        (Boolean(val1), Boolean(val2)) => Boolean(val1 > val2),
        _ => todo!("Comparison of {:?} and {:?} not implemented", &left, &right),
    }
}

fn eval_greater_equals(left: Value, right: Value) -> Value {
    use crate::mem::Value::*;
    match (&left, &right) {
        (Integer(val1), Integer(val2)) => Boolean(val1 >= val2),
        (String(val1), String(val2)) => Boolean(val1 >= val2),
        (Char(val1), Char(val2)) => Boolean(val1 >= val2),
        (Float(val1), Float(val2)) => Boolean(val1 >= val2),
        (Boolean(val1), Boolean(val2)) => Boolean(val1 >= val2),
        _ => todo!("Comparison of {:?} and {:?} not implemented", &left, &right),
    }
}

fn eval_not_equals(left: Value, right: Value) -> Value {
    use crate::mem::Value::*;
    match (&left, &right) {
        (Integer(val1), Integer(val2)) => Boolean(val1 != val2),
        (String(val1), String(val2)) => Boolean(val1 != val2),
        (Char(val1), Char(val2)) => Boolean(val1 != val2),
        (Float(val1), Float(val2)) =>
        {
            #[allow(clippy::float_cmp)]
            Boolean(val1 != val2)
        }
        (Boolean(val1), Boolean(val2)) => Boolean(val1 != val2),
        _ => todo!("Comparison of {:?} and {:?} not implemented", &left, &right),
    }
}
