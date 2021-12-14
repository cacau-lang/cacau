use std::{env, fs, io::BufWriter};

fn main() {
    let file = env::args_os().nth(1).expect("no file supplied");

    let contents = fs::read_to_string(file).unwrap();

    let parsed = parser_lalrpop::parse(contents.as_str());

    let mut writer = BufWriter::new(std::io::stdout());

    runner::Runner::run(&parsed, &mut writer);
}
