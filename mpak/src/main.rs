use dark_forge::mpk::{Mpak, Error};
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

#[derive(Clap)]
enum SubCommand {
    Ls(ListContents),
    Cat(CatFiles),
}

trait MpakCommand {
    fn run(self, mpak: Mpak);
}

impl MpakCommand for ListContents {
    fn run(self, mpak: Mpak) {
        let mut keys: Vec<&String> = mpak.file_info.keys().collect();
        keys.sort();
        for key in keys {
            println!("{}", key);
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
