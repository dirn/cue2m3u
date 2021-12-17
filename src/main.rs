use std::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cue2m3u", about = "Generate playlists for disc-based games.")]
enum Cli {
    #[structopt(name = "generate", about = "Generate playlists")]
    Generate {},
}

fn dispatch() -> Result<(), ()> {
    match Cli::from_args() {
        Cli::Generate {} => println!("generate playlists"),
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
