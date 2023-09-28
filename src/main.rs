use arithmetic_problems_generator::run;
use clap::{Arg, ArgGroup, Command};

mod parser;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Define and match command-line parameters
    let mut command = Command::new("arithmetic-problems-generator")
    .version("0.1.0")
    .arg(
        Arg::new("num")
            .help("The number of generated questions")
            .short('n')
            .required(false),
    )
    .arg(
        Arg::new("range")
            .help("The range of numeric values (natural numbers, true fractions, and true fraction denominators) in the question")
            .short('r')
            .required(false),
    )
    .arg(
        Arg::new("exercise_file")
            .help("The file path where the exercise questions are stored")
            .short('e')
            .required(false),
    )
    .arg(
        Arg::new("answer_file")
            .help("The file path where the answers to the exercise questions are stored")
            .short('a')
            .required(false),
    )
    .group(
        ArgGroup::new("check")
            .args(["exercise_file", "answer_file"])
            .conflicts_with("range")
            .conflicts_with("num")
    )
    .group(
        ArgGroup::new("generate")
            .args(["num","range"])
            .conflicts_with("exercise_file")
            .conflicts_with("answer_file")
    );
    let matches = command.clone().get_matches();

    match parser::to_config(matches) {
        Ok(config) => {
            //TODO Exception handling may be required
            run(config);
        }
        Err(err) => {
            eprintln!("Argument parser error: {}", err);
            let _ = command.print_help();
            std::process::exit(1);
        }
    }
    Ok(())
}
