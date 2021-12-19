use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::process;
use structopt::StructOpt;
use walkdir::WalkDir;

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
        } => generate_playlists(source, recursive, overwrite),
    }
}

fn find_cue_files(source: &PathBuf, recursive: bool) -> io::Result<Vec<PathBuf>> {
    let mut cue_files = vec![];

    let mut walker = WalkDir::new(source).follow_links(true).sort_by_file_name();
    if !recursive {
        walker = walker.max_depth(1);
    }
    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let name = entry.file_name().to_string_lossy();
        if name.ends_with(".cue") {
            cue_files.push(entry.path().to_owned());
        }
    }

    Ok(cue_files)
}

fn generate_playlists(source: PathBuf, recursive: bool, overwrite: bool) -> Result<(), String> {
    println!(
        "generate playlists for {:?}{}, {} existing",
        source,
        if recursive { " recursively" } else { "" },
        if overwrite { "overwrite" } else { "skip" }
    );
    if let Ok(cue_files) = find_cue_files(&source, recursive) {
        let cue_files = make_relative_paths(&source, cue_files);
        let cue_files_by_folder = group_files_by_folder(&cue_files);
        println!("found {:?}", cue_files_by_folder);
    } else {
        return Err("Error finding cue files".to_owned());
    }

    Ok(())
}

fn group_files_by_folder(files: &Vec<PathBuf>) -> HashMap<&Path, Vec<&PathBuf>> {
    let mut grouped_files = HashMap::new();

    for file in files {
        let prefix = file.parent().unwrap_or(Path::new(""));
        grouped_files.entry(prefix).or_insert(vec![]).push(file);
    }

    grouped_files
}

fn make_relative_paths(source: &PathBuf, absolute_paths: Vec<PathBuf>) -> Vec<PathBuf> {
    absolute_paths
        .into_iter()
        .map(|f| f.strip_prefix(&source).unwrap().to_owned())
        .collect()
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
