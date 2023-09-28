use arithmetic_problems_generator::{CheckerConfig, Config, GeneratorConfig};
use clap::ArgMatches;

pub fn to_config(matches: ArgMatches) -> Result<Config, Box<dyn std::error::Error>> {
    if matches.contains_id("range") {
        //check the value of range is valid
        let range = match matches.get_one::<String>("range").unwrap().parse::<u32>() {
            Ok(value) => value,
            Err(_) => return Err(format!("argument conversion exception").into()),
        };

        if matches.contains_id("num") {
            let num = match matches.get_one::<String>("num").unwrap().parse::<u32>() {
                Ok(value) => value,
                Err(_) => return Err(format!("argument conversion exception").into()),
            };
            return Ok(Config::Generator(GeneratorConfig::new(num, (0, range))));
        } else {
            return Ok(Config::Generator(GeneratorConfig::new(10, (0, range))));
        }
    } else if matches.contains_id("num") && !matches.contains_id("range") {
        let num = match matches.get_one::<String>("num").unwrap().parse::<u32>() {
            Ok(value) => value,
            Err(_) => return Err(format!("argument conversion exception").into()),
        };
        return Ok(Config::Generator(GeneratorConfig::new(num, (0, 100))));
    } else if matches.contains_id("exercise_file") && matches.contains_id("answer_file") {
        let exercise_file = matches.get_one::<String>("exercise_file").unwrap().clone();
        let answer_file = matches.get_one::<String>("answer_file").unwrap().clone();
        return Ok(Config::Checker(CheckerConfig::from_str(
            &exercise_file,
            &answer_file,
        )));
    } else {
        Err(format!("argument missing").into())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::to_config;
    use arithmetic_problems_generator::{CheckerConfig, Config};
    use clap::{Arg, ArgGroup, Command};

    //initial the command
    fn init() -> Command {
        Command::new("arithmetic-problems-generator")
        .version("0.1.0")
        .arg(
            Arg::new("num")
                .help("The number of generated problems")
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
                .multiple(true)
        )
        .group(
            ArgGroup::new("generate")
                .args(["num","range"])
                .conflicts_with("exercise_file")
                .conflicts_with("answer_file")
                .multiple(true)
        )
    }

    #[test]
    fn test_to_config_arg_range() {
        let command = init();

        let args = vec!["arithmetic-problems-generator", "-r", "10"];

        // Manually create a clap::ArgMatches object
        let matches = command.get_matches_from(&args);

        let range = match to_config(matches).unwrap() {
            Config::Generator(generator_config) => generator_config.range(),
            Config::Checker(_) => (0, 0),
        };
        assert_eq!(range, (0, 10));
    }

    #[test]
    fn test_to_config_arg_range_and_num() {
        let command = init();

        let args = vec!["arithmetic-problems-generator", "-n", "11", "-r", "10"];

        // Manually create a clap::ArgMatches object
        let matches = command.get_matches_from(&args);

        let range = match to_config(matches).unwrap() {
            Config::Generator(generator_config) => (
                generator_config.range().0,
                generator_config.range().1,
                generator_config.number(),
            ),
            Config::Checker(_) => (0, 0, 0),
        };
        assert_eq!(range, (0, 10, 11));
    }

    #[test]
    fn test_to_config_arg_num() {
        let command = init();

        let args = vec!["arithmetic-problems-generator", "-n", "11"];

        // Manually create a clap::ArgMatches object
        let matches = command.get_matches_from(&args);

        let range = match to_config(matches).unwrap() {
            Config::Generator(generator_config) => (
                generator_config.range().0,
                generator_config.range().1,
                generator_config.number(),
            ),
            Config::Checker(_) => (0, 0, 0),
        };
        assert_eq!(range, (0, 100, 11));
    }

    #[test]
    fn test_to_config_exercise_and_num() {
        let command = init();

        let args = vec![
            "arithmetic-problems-generator",
            "-n",
            "11",
            "-e",
            "test.txt",
        ];

        // Manually create a clap::ArgMatches object
        let matches = command.try_get_matches_from(&args);

        assert!(matches.is_err());
    }

    #[test]
    fn test_to_config_exercise_and_answer() {
        let command = init();

        let args = vec![
            "arithmetic-problems-generator",
            "-a",
            "answer.txt",
            "-e",
            "exercise.txt",
        ];

        // Manually create a clap::ArgMatches object
        let matches = command.get_matches_from(&args);

        let config = match to_config(matches).unwrap() {
            Config::Generator(_) => CheckerConfig::new(PathBuf::new(), PathBuf::new()),
            Config::Checker(config) => config,
        };

        assert_eq!(config.answer_path().to_str().unwrap(), "answer.txt");
        assert_eq!(config.exercise_path().to_str().unwrap(), "exercise.txt");
    }
}
