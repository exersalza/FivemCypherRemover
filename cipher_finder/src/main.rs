use clap::Parser;

use crate::file_op::ScannedFile;

pub mod de_obfs;
pub mod file_op;
pub mod os;
pub mod utils;

// todo:
//  scan modes: // defines how many patterns will be used to scan one file
//   aggressive // all
//   passive // main patterns, hex detection

#[derive(Parser, Debug)]
#[clap(name = "FivemCipherFinder", about = "FivemCipherFinder finds ciphers in your scripts.", long_about = None)]
struct Args {
    #[clap(short = 'm', long = "mode", default_value = "aggressive")]
    /// Scan mode
    mode: String,

    #[clap(short = 'p', long = "path", default_value = ".")]
    /// Paht to the Directory where your server is located
    path: String,

    #[clap(short = 'e', long = "exclude", default_value = "")]
    /// Exclude given Paths from Search. Syntax: foo,bar,foobar
    exclude: String,

    #[clap(long = "include-git", default_value = "false")]
    /// includes content of .gitignore files. Maybe increases the time it needs to filter out files
    include_git: bool,
}

fn main() -> std::io::Result<()> {
    // i kissed a girl and i liked it https://images.app.goo.gl/ynuCJ85rmxJFVNBs5
    let opt = Args::parse();
    let all_paths = os::get_all_files(opt.path, Some(utils::format_dir_str(opt.exclude)));
    let paths = utils::filter_viables(all_paths);

    for i in paths {
        let infected = ScannedFile::new(i);
        println!("{:?}", infected?.get_infected());
    }

    Ok(())
}
