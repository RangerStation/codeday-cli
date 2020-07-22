mod string_reverser;

use structopt::StructOpt;

fn main() {
    let args = CliArgs::from_args();
    match args {
        CliArgs::Hello => {
            println!("Hello");
        },
        CliArgs::Goodbye => {
            println!("Goodbye");
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "mycli", rename_all = "kebab-case")]
pub enum CliArgs {
    #[structopt(name = "hello", about = "hello subcommand")]
    Hello,
    #[structopt(name = "goodbye", about = "goodbye subcommand")]
    Goodbye,
}