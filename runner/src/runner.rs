use std::io::Write;

use ast::{
    ArithExpr, CacauProgram, CmpExpr, Expr, FnCall, LogicExpr, Statement, Unary, VariableDecl,
};

use crate::mem::{SymbolTable, Value};

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
            use Statement::*;
            match item {
                Expr(ref expr) => {
                    runner.eval_expr(expr);
                }
                _ => todo!(),
            }
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Value {
        use Expr::*;
        match expr {
            FnCall(call) => self.eval_function_call(call),
            Lit(ast::Lit::Int(integer)) => Value::Integer(*integer),
            Lit(ast::Lit::Float(float)) => Value::Float(*float),
            Lit(ast::Lit::Bool(boolean)) => Value::Boolean(*boolean),
            Lit(ast::Lit::Char(char)) => Value::Char(*char),
            Lit(ast::Lit::String(string)) => Value::String(string.clone()),
            VarDecl(assign) => self.eval_assignment(assign),
            Id(name) => self.eval_identifier(name),
            Cmp(comp) => self.eval_cmp_expr(comp),
            Arith(arith) => self.eval_arith_expr(arith),
            Logic(boolean) => self.eval_logic_expr(boolean),
            Unary(ref unary) => self.eval_unary(unary),
            Paren(expr) => self.eval_expr(expr),
        }
    }

    fn eval_unary(&mut self, unary: &Unary) -> Value {
        match unary {
            Unary::Not(expr) => eval_not(self.eval_expr(expr)),
            Unary::Minus(expr) => eval_minus(self.eval_expr(expr)),
        }
    }

    // TODO assignment returns the assigned value?
    // TODO scope rules
    fn eval_assignment(&mut self, assign: &VariableDecl) -> Value {
        let val = self.eval_expr(&assign.expr);
        self.symbol_table.create_var(&assign.name, val);

        Value::Void
    }

    fn eval_cmp_expr(&mut self, comp: &CmpExpr) -> Value {
        use ast::CmpOp::*;

        let left = self.eval_expr(&comp.left);
        let right = self.eval_expr(&comp.right);

        match comp.op {
            EQ => eval_equals(left, right),
            NE => eval_not_equals(left, right),
            LT => eval_less(left, right),
            LE => eval_less_equals(left, right),
            GT => eval_greater(left, right),
            GE => eval_greater_equals(left, right),
        }
    }

    fn eval_arith_expr(&mut self, arith: &ArithExpr) -> Value {
        use ast::ArithOp::*;

        let left = self.eval_expr(&arith.left);
        let right = self.eval_expr(&arith.right);

        match arith.op {
            Add => eval_add(left, right),
            Sub => eval_subtract(left, right),
            Mul => eval_multiply(left, right),
            Div => eval_divide(left, right),
            Pow => eval_power(left, right),
            Mod => eval_modulo(left, right),
        }
    }

    fn eval_logic_expr(&mut self, boolean: &LogicExpr) -> Value {
        use ast::LogicOp::*;

        let left = self.eval_expr(&boolean.left);
        let right = self.eval_expr(&boolean.right);

        match boolean.op {
            Or => eval_or(left, right),
            And => eval_and(left, right),
        }
    }

    fn eval_function_call(&mut self, call: &FnCall) -> Value {
        if let Expr::Id(ref name) = call.callee {
            match (name.as_str(), call.params.len()) {
                ("print", 1) => return self.eval_print(call),
                ("println", 1) => return self.eval_println(call),
                ("assert", 1) => return self.eval_assert(call),
                ("string", 1) => return self.eval_string_cast(call),
                ("float", 1) => return self.eval_float_cast(call),
                ("int", 1) => return self.eval_int_cast(call),
                ("bool", 1) => return self.eval_bool_cast(call),
                _ => (),
            }
        }
        panic!(
            "Function {:?} could not be found or was given an invalid number of args",
            call.callee
        )
    }

    fn eval_string_cast(&mut self, call: &FnCall) -> Value {
        match self.eval_expr(&call.params[0]) {
            Value::Boolean(bool) => Value::String(bool.to_string()),
            Value::Float(float) => Value::String(format!("{:.5}", float)),
            Value::Integer(int) => Value::String(int.to_string()),
            Value::Char(char) => Value::String(char.to_string()),
            invalid => panic!("String conversion not implemented for {:?}", invalid),
        }
    }

    fn eval_float_cast(&mut self, call: &FnCall) -> Value {
        match self.eval_expr(&call.params[0]) {
            Value::Integer(int) => Value::Float(int as f64),
            Value::String(ref str) => Value::Float(
                str.parse()
                    .unwrap_or_else(|_| panic!("Could not parse {} to float", str)),
            ),
            invalid => panic!("Float conversion not implemented for {:?}", invalid),
        }
    }

    fn eval_int_cast(&mut self, call: &FnCall) -> Value {
        match self.eval_expr(&call.params[0]) {
            Value::Float(int) => Value::Integer(int as i64),
            Value::String(ref str) => Value::Integer(
                str.parse()
                    .unwrap_or_else(|_| panic!("Could not parse {} to integer", str)),
            ),
            invalid => panic!("Integer conversion not implemented for {:?}", invalid),
        }
    }

    fn eval_bool_cast(&mut self, call: &FnCall) -> Value {
        match self.eval_expr(&call.params[0]) {
            Value::Integer(int) => Value::Boolean(int != 0),
            Value::String(ref str) => Value::Boolean(
                str.parse()
                    .unwrap_or_else(|_| panic!("Could not parse {} to integer", str)),
            ),
            invalid => panic!("Integer conversion not implemented for {:?}", invalid),
        }
    }

    fn eval_assert(&mut self, call: &FnCall) -> Value {
        match self.eval_expr(&call.params[0]) {
            Value::Boolean(assert_ok) => {
                if !assert_ok {
                    // TODO panic only the runtime
                    // TODO show expression that failed
                    panic!("Assert failed: {:?}", call.params);
                } else {
                    Value::Void
                }
            }
            _ => Value::Void,
        }
    }

    fn eval_print(&mut self, call: &FnCall) -> Value {
        match self.eval_expr(&call.params[0]) {
            Value::String(ref str) => {
                let _bytes_written = self.stdout.write(str.as_bytes()).unwrap();
            }
            Value::Integer(val) => {
                let str = format!("{}", val);
                let _bytes_written = self.stdout.write(str.as_bytes()).unwrap();
            }
            Value::Float(val) => {
                let fmt = format!("{:.5}", val);
                let _bytes_written = self.stdout.write(fmt.as_bytes()).unwrap();
            }
            Value::Boolean(val) => {
                let fmt = format!("{}", val);
                let _bytes_written = self.stdout.write(fmt.as_bytes()).unwrap();
            }
            Value::Char(val) => {
                let fmt = format!("{}", val);
                let _bytes_written = self.stdout.write(fmt.as_bytes()).unwrap();
            }
            _ => todo!(),
        }
        Value::Void
    }

    fn eval_println(&mut self, call: &FnCall) -> Value {
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
        (String(string), Integer(multiplier)) => {
            String((0..*multiplier).map(|_| string.as_str()).collect())
        }
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
        (Integer(val1), Float(val2)) => Boolean(*val1 as f64 == *val2),
        (Float(val1), Integer(val2)) => Boolean(*val1 == *val2 as f64),
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
        (Integer(val1), Float(val2)) => Boolean((*val1 as f64) < *val2),
        (Float(val1), Integer(val2)) => Boolean(*val1 < *val2 as f64),
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
        (Integer(val1), Float(val2)) => Boolean(*val1 as f64 <= *val2),
        (Float(val1), Integer(val2)) => Boolean(*val1 <= *val2 as f64),
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
        (Integer(val1), Float(val2)) => Boolean(*val1 as f64 > *val2),
        (Float(val1), Integer(val2)) => Boolean(*val1 > *val2 as f64),
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
        (Integer(val1), Float(val2)) => Boolean(*val1 as f64 >= *val2),
        (Float(val1), Integer(val2)) => Boolean(*val1 >= *val2 as f64),
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
        (Integer(val1), Float(val2)) => Boolean(*val1 as f64 != *val2),
        (Float(val1), Integer(val2)) => Boolean(*val1 != *val2 as f64),
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
