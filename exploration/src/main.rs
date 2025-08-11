use anyhow::Result;
use clap::{Parser, Subcommand, CommandFactory};
use dialoguer::{theme::ColorfulTheme, Input, Select};

#[derive(Parser, Debug)]
#[command(
    author, version,
    about = "Un CLI hybride : arguments + mode interactif",
    long_about = "This program help you compress (and later decompress) any file using Gzip. "
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
            println!("Compressing file: {}", file);
            // Call your compression function here
        }
        1 => {
            let file: String = Input::with_theme(&theme)
                .with_prompt("Enter the file to decompress")
                .interact_text()?;
            println!("Decompressing file: {}", file);
            // Call your decompression function here
        }
        _ => unreachable!(),    
    }
    Ok(())
}

fn run_command(cmd: Commands) -> Result<()> {
    match cmd {
        Commands::Compress { file } => {
            println!("Compressing file: {}", file);
            // Call your compression function here
        }
        Commands::Decompress { file } => {
            println!("Decompressing file: {}", file);
            
        }
    }
    Ok(())
}



fn main() -> Result<()> {
    let cli = Cli::parse();

    // Si l'utilisateur force --interactive ou ne passe aucune sous-commande, on ouvre le menu.
    match (cli.interactive, cli.command) {
        (true, _) | (false, None) => interactive_mode()?,
        (false, Some(cmd)) => run_command(cmd)?,
    }

    Ok(())
}