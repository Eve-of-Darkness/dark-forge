use dark_forge::mpk::{Mpak, Error};
use clap::Parser;
use std::io::{self, Write};

/// A simple tool to work with mpk files found in
/// the Dark Age of Camelot game client.  This is
/// a proprietary compression format which can
/// contain any number of files in a single archive.
///
/// This tools is designed to work with a single
/// file at a time and has sub-commands to perform
/// different actions.
#[derive(Parser)]
#[clap(author = "Ben Falk <benjamin.falk@yahoo.com>", version = "0.1.1")]
struct Opts {
    /// The file to work with
    file: String,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// List the contents of the mpak file
#[derive(Parser)]
struct ListContents {
}

/// Copy contents to stdout
#[derive(Parser)]
struct CatFiles {
    /// which file(s) to stream to stdout
    files: Vec<String>,
}

/// Unzip contents to files
#[derive(Parser)]
struct Unzip {
    /// Optional directory to unpack files to
    dir: Option<String>
}

#[derive(Parser)]
enum SubCommand {
    Ls(ListContents),
    Cat(CatFiles),
    Unzip(Unzip)
}

trait MpakCommand {
    fn run(self, mpak: Mpak);
}

impl MpakCommand for ListContents {
    fn run(self, mpak: Mpak) {
        let mut filenames = mpak.file_names();
        filenames.sort();
        for file in filenames {
            println!("{}", file);
        }
    }
}

impl MpakCommand for CatFiles {
    fn run(self, mut mpak: Mpak) {
        use io::ErrorKind::BrokenPipe;

        for file in self.files {
            match mpak.file_contents(&file) {
                None => eprintln!("File {} Not Found", &file),
                Some(data) => {
                    match io::stdout().write_all(&data) {
                        Ok(()) => (),
                        Err(error) => {
                            if error.kind() != BrokenPipe {
                                panic!("{:?}", error)
                            }
                        }
                    }
                }
            }
        }
    }
}

impl MpakCommand for Unzip {
    fn run(self, mut mpak: Mpak) {
        let dir = self.dir.unwrap_or_else(|| mpak.name.clone());
        std::fs::create_dir_all(&dir).unwrap();
        let mut names = vec![];

        for name in mpak.file_names() {
            names.push(name.clone());
        }

        for filename in names {
            let contents = mpak.file_contents(&filename).unwrap();
            let filename = format!("{}/{}", &dir, &filename);
            let mut file = std::fs::File::create(&filename).unwrap();
            file.write_all(&contents).unwrap();
        }
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    match Mpak::open(&opts.file) {
        Err(Error::IO(error)) =>
            eprintln!("{}", error),

        Ok(mpak) => {
            match opts.subcmd {
                SubCommand::Ls(cmd) => cmd.run(mpak),
                SubCommand::Cat(cmd) => cmd.run(mpak),
                SubCommand::Unzip(cmd) => cmd.run(mpak),
            }
        }
    }
}
