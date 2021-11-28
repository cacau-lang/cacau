use std::io::Write;

use crate::{
    ast::{Assignment, CacauProgram, Expression, FunctionCall, HighLevelItem},
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

    fn define_struct(&self) {
        todo!()
    }

    fn define_enum(&self) {
        todo!()
    }

    fn define_function(&self) {
        todo!()
    }

    fn eval_expr(&mut self, expr: &Expression) -> Value {
        use Expression::*;
        match expr {
            FunctionCall(call) => self.eval_function_call(call),
            IntegerLiteral(integer) => Value::Integer(*integer),
            StringLiteral(string) => Value::String(String::from(*string)),
            Assignment(assign) => self.eval_assignment(assign),
            Identifier(name) => self.eval_identifier(name),
            _ => todo!(),
        }
    }

    fn eval_if(&self) {
        todo!()
    }

    // TODO assignment returns the assigned value?
    // TODO scope rules
    fn eval_assignment(&mut self, assign: &Assignment) -> Value {
        let val = self.eval_expr(&assign.expression);
        self.symbol_table.create_var(assign.name, val);

        Value::Void
    }

    fn eval_function_call(&mut self, call: &FunctionCall) -> Value {
        // HACK println is hardcoded here
        if call.name == "println" && call.params.len() == 1 {
            match self.eval_expr(&call.params[0]) {
                Value::String(ref str) => {
                    let _bytes_written = self.stdout.write(str.as_bytes()).unwrap();
                    let _bytes_written = self.stdout.write(b"\n").unwrap();
                }
                Value::Integer(val) => {
                    let str = format!("{}\n", val);
                    let _bytes_written = self.stdout.write(str.as_bytes()).unwrap();
                }
                _ => todo!(),
            }
            Value::Void
        } else if call.name == "assert" && call.params.len() == 1 {
            todo!();
        } else {
            // TODO function not found
            Value::Void
        }
    }

    fn eval_identifier(&self, name: &str) -> Value {
        if let Some(value) = self.symbol_table.get_value(name) {
            value.clone()
        } else {
            // TODO error instead
            Value::Void
        }
    }
}
