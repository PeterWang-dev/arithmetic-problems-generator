use std::path::PathBuf;

pub enum Config {
    Generator(GeneratorConfig),
    Checker(CheckerConfig),
}

pub struct GeneratorConfig {
    number: u32,
    range: (u32, u32),
}

pub struct CheckerConfig {
    exercise_path: PathBuf,
    answer_path: PathBuf,
}

impl GeneratorConfig {
    pub fn new(number: u32, range: (u32, u32)) -> Self {
        Self { number, range }
    }

    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn range(&self) -> (u32, u32) {
        self.range
    }
}

impl CheckerConfig {
    pub fn new(exercise_path: PathBuf, answer_path: PathBuf) -> Self {
        Self {
            exercise_path,
            answer_path,
        }
    }

    pub fn from_str(exercise_path: &str, answer_path: &str) -> Self {
        Self {
            exercise_path: PathBuf::from(exercise_path),
            answer_path: PathBuf::from(answer_path),
        }
    }

    pub fn exercise_path(&self) -> &PathBuf {
        &self.exercise_path
    }

    pub fn answer_path(&self) -> &PathBuf {
        &self.answer_path
    }
}

// TODO: run
pub fn run(config: Config) {
    match config {
        Config::Generator(_) => {
            println!("Generator");
        }
        Config::Checker(_) => {
            println!("Checker");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_config_new() {
        let config = GeneratorConfig::new(1, (1, 10));

        assert_eq!(config.number(), 1);
        assert_eq!(config.range(), (1, 10));
    }

    #[test]
    fn test_generator_config_number() {
        let config = GeneratorConfig::new(1, (1, 10));

        assert_eq!(config.number(), 1);
    }

    #[test]
    fn test_generator_config_range() {
        let config = GeneratorConfig::new(1, (1, 10));

        assert_eq!(config.range(), (1, 10));
    }

    #[test]
    fn test_checker_config_new() {
        let config = CheckerConfig::new(PathBuf::from("exercise"), PathBuf::from("answer"));

        assert_eq!(config.exercise_path(), &PathBuf::from("exercise"));
        assert_eq!(config.answer_path(), &PathBuf::from("answer"));
    }

    #[test]
    fn test_checker_config_from_str() {
        let config = CheckerConfig::from_str("Exercises.txt", "Answer.txt");

        assert_eq!(config.exercise_path(), &PathBuf::from("Exercises.txt"));
        assert_eq!(config.answer_path(), &PathBuf::from("Answer.txt"));
    }

    #[test]
    fn test_config() {
        let generator_config = GeneratorConfig::new(0, (0, 0));
        let checker_config = CheckerConfig::new(PathBuf::new(), PathBuf::new());

        let config = Config::Generator(generator_config);
        match config {
            Config::Generator(_) => {
                assert!(true)
            }
            Config::Checker(_) => {
                assert!(false)
            }
        }

        let config = Config::Checker(checker_config);
        match config {
            Config::Generator(_) => {
                assert!(false)
            }
            Config::Checker(_) => {
                assert!(true)
            }
        }
    }
}
