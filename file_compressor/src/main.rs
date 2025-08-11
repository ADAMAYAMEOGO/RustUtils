mod gz;

use anyhow::Result;
use clap::{Parser, Subcommand, CommandFactory};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(
    author, version,
    about = "A hybrid CLI tool: arguments + interactive mode",
    long_about = "This program compresses and decompresses files using Gzip (streaming mode, no memory load)."
)]
struct Cli {
    #[arg(short, long)]
    interactive: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Compress {
        #[arg(short, long)]
        file: String,
    },
    Decompress {
        #[arg(short, long)]
        file: String,
    },
}

fn interactive_mode() -> Result<()> {
    let theme = ColorfulTheme::default();

    let choice = Select::with_theme(&theme)
        .with_prompt("Choose an action")
        .item("Compress a file")
        .item("Decompress a file")
        .default(0)
        .interact()?;

    match choice {
        0 => {
            let file: String = Input::with_theme(&theme)
                .with_prompt("Enter the file to compress")
                .interact_text()?;
            let out = gz::compress_file(Path::new(&file), None)?;
            println!("✅ Compressed to: {}", out.display());
        }
        1 => {
            let file: String = Input::with_theme(&theme)
                .with_prompt("Enter the .gz file to decompress")
                .interact_text()?;
            let out = gz::decompress_file(Path::new(&file), None)?;
            println!("✅ Decompressed to: {}", out.display());
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn run_command(cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Compress { file } => {
            let out = gz::compress_file(Path::new(&file), None)?;
            println!("✅ Compressed to: {}", out.display());
        }
        Commands::Decompress { file } => {
            let out = gz::decompress_file(Path::new(&file), None)?;
            println!("✅ Decompressed to: {}", out.display());
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match (cli.interactive, cli.command) {
        (true, _) | (false, None) => interactive_mode()?,
        (false, Some(cmd)) => run_command(cmd)?,
    }

    Ok(())
}
