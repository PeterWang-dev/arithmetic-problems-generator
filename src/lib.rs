pub enum Config {
    Generator(GeneratorConfig),
    Checker(CheckerConfig),
}

pub struct GeneratorConfig {}

pub struct CheckerConfig {}

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