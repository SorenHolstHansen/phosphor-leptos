use clap::Parser;

mod update;

#[derive(Parser)]
enum Command {
    /// Re-generate the icon components from the original SVG files
    Update,
}

impl Command {
    fn run(&self) {
        match self {
            Self::Update => update::run(),
        }
    }
}

fn main() {
    Command::parse().run()
}
