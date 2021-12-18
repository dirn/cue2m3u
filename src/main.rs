use std::ffi::OsStr;
use std::fs::read_dir;
use std::io;
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
        #[structopt(long, short, help = "Overwrite existing playlists")]
        overwrite: bool,
        #[structopt(
            required = true,
            help = "Location of the games to generate playlists for"
        )]
        source: PathBuf,
    },
}

fn dispatch() -> Result<(), String> {
    match Cli::from_args() {
        Cli::Generate {
            source,
            recursive,
            overwrite,
        } => {
            println!(
                "generate playlists for {:?}{}, {} existing",
                source,
                if recursive { " recursively" } else { "" },
                if overwrite { "overwrite" } else { "skip" }
            );
            if let Ok(cue_files) = find_cue_files(source) {
                println!("found {:?}", cue_files);
            } else {
                return Err("Error finding cue files".to_owned());
            }
        }
    }

    Ok(())
}

fn find_cue_files(source: PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut cue_files = vec![];

    for path in read_dir(source)? {
        let path = path?.path();
        if let Some("cue") = path.extension().and_then(OsStr::to_str) {
            cue_files.push(path.to_owned());
        }
    }

    Ok(cue_files)
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
