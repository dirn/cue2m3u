use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cue2m3u", about = "Generate playlists for disc-based games.")]
enum Cli {
    #[structopt(name = "generate", about = "Generate playlists")]
    Generate {
        #[structopt(long, short, help = "Scan subfolders of `input`")]
        recursive: bool,
        #[structopt(
            required = true,
            help = "Location of the games to generate playlists for"
        )]
        source: PathBuf,
    },
}

fn dispatch() -> Result<(), ()> {
    match Cli::from_args() {
        Cli::Generate { source, recursive } => println!(
            "generate playlists for {:?}{}",
            source,
            if recursive { " recursively" } else { "" }
        ),
    }

    Ok(())
}

fn main() {
    process::exit(match dispatch() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("error: {:?}", e);
            1
        }
    });
}
