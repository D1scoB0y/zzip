use anyhow;
use clap::Parser;
use ignore::WalkBuilder;
use std::fs;
use std::path::{Path, PathBuf};
use zip::{ZipWriter, write::SimpleFileOptions};

#[derive(Parser)]
#[command(
    name = "zzip",
    version = "0.1.0",
    about = "ZIP archiver that honors .gitignore and .zipignore files"
)]
struct Args {
    /// Paths to content to archive
    #[arg(required = true)]
    inputs: Vec<PathBuf>,

    /// Respect .gitignore
    #[arg(short, long)]
    gitignore: bool,

    /// Output zip archive filename
    #[arg(short, long)]
    output: Option<PathBuf>,
}

const DEFAULT_ARCHIVE_NAME: &str = "Archive.zip";

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut canonicalized_inputs = Vec::with_capacity(args.inputs.len());

    for input in args.inputs {
        canonicalized_inputs.push(input.canonicalize()?);
    }

    let output = match args.output {
        Some(output) => output,

        None if canonicalized_inputs.len() == 1 => {
            let input = &canonicalized_inputs[0];

            if input == Path::new("/") {
                PathBuf::from(DEFAULT_ARCHIVE_NAME)
            } else if input.is_dir() {
                let dirname = input
                    .file_name()
                    .expect("directory should have a name")
                    .to_string_lossy();

                PathBuf::from(format!("{dirname}.zip"))
            } else {
                let filename = input
                    .file_name()
                    .expect("file should have a name")
                    .to_string_lossy();

                PathBuf::from(format!("{filename}.zip"))
            }
        }

        None => PathBuf::from(DEFAULT_ARCHIVE_NAME),
    };

    let f = fs::File::create(&output)?;
    let f = std::io::BufWriter::new(f);
    let mut archive = ZipWriter::new(f);

    let canonicalized_output = output.canonicalize().ok();

    for input in &canonicalized_inputs {
        if input.is_dir() {
            let base = input.parent().unwrap_or(input.as_path());

            let walker = WalkBuilder::new(input)
                .git_ignore(args.gitignore)
                .add_custom_ignore_filename(".archignore")
                .hidden(false)
                .build();

            for entry in walker {
                let entry = entry?;

                if !entry.file_type().is_some_and(|t| t.is_file()) {
                    continue;
                }

                if canonicalized_output
                    .as_ref()
                    .is_some_and(|p| p == entry.path())
                {
                    continue;
                }

                let relative = entry.path().strip_prefix(base)?;

                archive.start_file(relative.to_string_lossy(), SimpleFileOptions::default())?;

                let mut file = fs::File::open(entry.path())?;
                std::io::copy(&mut file, &mut archive)?;
            }
        } else if input.is_file() {
            archive.start_file(
                input.file_name().unwrap().to_string_lossy(),
                SimpleFileOptions::default(),
            )?;

            let mut file = fs::File::open(input)?;
            std::io::copy(&mut file, &mut archive)?;
        }
    }

    archive.finish()?;

    Ok(())
}
