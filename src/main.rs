use clap::{Parser, Subcommand};
use std::{path::PathBuf, process::ExitCode};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// The canonical, absolute, symlink resolved version of this path
    Canonicalize { path: PathBuf },

    /// Get the nth component of this path.
    /// If the `n` is negative, count back from the last component. (-1 being the last component)
    Component { path: PathBuf, n: i32 },

    /// Whether the path exists
    Exists { path: PathBuf },

    /// Return the extension of the file name
    Extension { path: PathBuf },

    /// Get the filename
    Filename { path: PathBuf },

    /// Whether the path is an absolute path
    IsAbsolute { path: PathBuf },

    /// Whether the path is a directory
    IsDir { path: PathBuf },

    /// Whether the path is a file
    IsFile { path: PathBuf },

    /// Whether the path is a relative path
    IsRelative { path: PathBuf },

    /// Whether the path is a symlink
    IsSymlink { path: PathBuf },

    /// Join multiple paths
    Join { paths: Vec<PathBuf> },

    /// Return the parent of the input
    Parent { path: PathBuf },

    /// The path that the symlink points to
    ResolveLink { path: PathBuf },

    /// Get the filename excluding the extension
    Stem { path: PathBuf },

    /// With a different filename in the same directory
    WithFileName { path: PathBuf, filename: String },

    /// Get the filename with a different suffix
    WithSuffix { path: PathBuf, suffix: String },
}

fn process(command: Commands) -> Option<()> {
    match command {
        Commands::Stem { path } => {
            println!("{}", path.as_path().file_stem()?.to_string_lossy());
        }
        Commands::IsAbsolute { path } => {
            path.as_path().is_absolute().then_some(())?;
        }
        Commands::Parent { path } => {
            println!("{}", path.as_path().parent()?.to_string_lossy());
        }
        Commands::Filename { path } => {
            println!("{}", path.as_path().file_name()?.to_string_lossy());
        }
        Commands::WithSuffix { path, suffix } => {
            println!(
                "{}",
                path.as_path().with_extension(suffix).to_string_lossy()
            );
        }
        Commands::IsRelative { path } => {
            path.as_path().is_relative().then_some(())?;
        }
        Commands::IsDir { path } => {
            path.as_path().is_dir().then_some(())?;
        }
        Commands::Canonicalize { path } => {
            println!("{}", path.as_path().canonicalize().ok()?.display());
        }
        Commands::Component { path, n } => {
            let components: Vec<_> = path.as_path().components().collect();
            if components.is_empty() {
                return None;
            }
            let n = n.rem_euclid(components.len() as i32) as usize;
            println!("{}", components[n].as_os_str().to_string_lossy());
        }
        Commands::Exists { path } => {
            path.as_path().exists().then_some(())?;
        }
        Commands::Extension { path } => {
            println!("{}", path.as_path().extension()?.to_string_lossy());
        }
        Commands::IsFile { path } => {
            path.as_path().is_file().then_some(())?;
        }
        Commands::IsSymlink { path } => {
            path.as_path().is_symlink().then_some(())?;
        }
        Commands::Join { paths } => {
            let joined = paths.into_iter().reduce(|p1, p2| p1.join(p2))?;
            println!("{}", joined.display());
        }
        Commands::ResolveLink { path } => {
            println!("{}", path.read_link().ok()?.display());
        }
        Commands::WithFileName { path, filename } => {
            println!("{}", path.with_file_name(filename).display());
        }
    };
    Some(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match process(cli.command) {
        Some(_) => ExitCode::SUCCESS,
        None => ExitCode::FAILURE,
    }
}
