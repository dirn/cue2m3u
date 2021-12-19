use std::collections::HashMap;
use std::fmt;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process;
use structopt::StructOpt;
use walkdir::WalkDir;

struct Playlist {
    name: String,
    files: Vec<PathBuf>,
}

impl Playlist {
    fn to_contents(&self) -> Vec<String> {
        self.files
            .clone()
            .into_iter()
            .map(|f| f.to_string_lossy().to_string())
            .collect()
    }

    fn to_m3u(&self) -> String {
        format!("{}.m3u", self.name)
    }
}

impl fmt::Debug for Playlist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Playlist")
            .field("m3u", &self.to_m3u())
            .field("contents", &self.to_contents())
            .finish()
    }
}

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
    if let Ok(cue_files) = find_cue_files(&source, recursive) {
        let cue_files = make_relative_paths(&source, cue_files);
        let cue_files_by_folder = group_files_by_folder(&cue_files);
        let playlists = make_playlists(&source, cue_files_by_folder);
        for playlist in playlists {
            if let Err(e) = write_playlist(&source, playlist, overwrite) {
                return Err(format!("Error writing {}", e));
            }
        }
    } else {
        return Err("Error finding cue files".to_owned());
    }

    Ok(())
}

fn group_files_by_folder(files: &Vec<PathBuf>) -> HashMap<&Path, Vec<PathBuf>> {
    let mut grouped_files = HashMap::new();

    for file in files {
        let prefix = file.parent().unwrap_or(Path::new(""));
        grouped_files
            .entry(prefix)
            .or_insert(vec![])
            .push(file.to_owned());
    }

    grouped_files
}

fn make_playlists(source: &PathBuf, grouped_files: HashMap<&Path, Vec<PathBuf>>) -> Vec<Playlist> {
    let mut playlists: Vec<Playlist> = vec![];

    for (name, files) in grouped_files.into_iter() {
        let mut playlist_name = name.to_string_lossy().to_string();
        if playlist_name == "" {
            playlist_name = source.file_name().unwrap().to_string_lossy().to_string();
        }
        playlists.push(Playlist {
            name: playlist_name,
            files,
        });
    }

    playlists
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

fn write_playlist(source: &PathBuf, playlist: Playlist, overwrite: bool) -> io::Result<()> {
    let mut file = match OpenOptions::new()
        .write(true)
        .create_new(!overwrite)
        .open(source.join(playlist.to_m3u()))
    {
        Ok(file) => file,
        Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => return Ok(()),
        Err(e) => return Err(e),
    };
    for line in playlist.to_contents() {
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
    }
    Ok(())
}
