use dark_forge::mpk::{Mpak, Error, FileInfo};
use clap::Clap;
use std::io::{self, Write};

#[derive(Clap)]
#[clap(author = "Ben Falk <benjamin.falk@yahoo.com>")]
struct Opts {
    /// The file to work with
    file: String,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// List the contents of the mpak file
#[derive(Clap)]
struct ListContents {
}

/// Copy contents to stdout
#[derive(Clap)]
struct CatFiles {
    files: Vec<String>,
}

/// Unzip contents to files
#[derive(Clap)]
struct Unzip {
    /// Optional directory to unpack files to
    dir: Option<String>
}

#[derive(Clap)]
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
    fn run(self, mpak: Mpak) {
        let mut mpak = mpak;

        for file in self.files {
            match mpak.file_contents(&file) {
                None => eprintln!("File {} Not Found", &file),
                Some(data) => {
                    io::stdout().write_all(&data).unwrap();
                }
            }
        }
    }
}

impl MpakCommand for Unzip {
    fn run(self, mpak: Mpak) {
        let mut mpak = mpak;
        let dir = self.dir.unwrap_or(mpak.name.clone());
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
            }
        }
    }
}
