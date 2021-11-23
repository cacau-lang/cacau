use std::io::Write;

use crate::{
    ast::{CacauProgram, Expression, FunctionCall, HighLevelItem},
    mem::{SymbolTable, Value},
};

pub struct Runner<'a> {
    stdout: &'a mut dyn Write,
    symbol_table: SymbolTable,
}

impl<'a> Runner<'a> {
    pub fn run(program: &CacauProgram, stdout: &mut dyn Write) {
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

    fn define_struct(&mut self) {
        todo!()
    }

    fn define_enum(&mut self) {
        todo!()
    }

    fn define_function(&mut self) {
        todo!()
    }

    fn eval_expr(&mut self, expr: &Expression) -> Value {
        use Expression::*;
        match expr {
            FunctionCall(call) => self.eval_function_call(call),
            Integer(integer) => Value::Integer(*integer),
            Char(string) => Value::String(String::from(*string)),
            _ => Value::Void,
        }
    }

    fn eval_if(&mut self) {
        todo!()
    }

    fn eval_assignment(&mut self) {
        todo!()
    }

    fn eval_function_call(&mut self, call: &FunctionCall) -> Value {
        // HACK println is hardcoded here
        if call.name == "println" && call.params.len() == 1 {
            match self.eval_expr(&call.params[0]) {
                Value::String(ref str) => {
                    let _bytes_written = self.stdout.write(str.as_bytes()).unwrap();
                    let _bytes_written = self.stdout.write(b"\n").unwrap();
                }
                _ => todo!(),
            }
            Value::Void
        } else {
            todo!()
        }
    }
}
