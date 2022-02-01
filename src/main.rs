use std::env;

use minigrep::Config;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)?;
    minigrep::print_begin_execution_msg(&config);
    minigrep::run(config)?;
    Ok(())
}
