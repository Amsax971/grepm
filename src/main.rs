use std::env::args;
use std::fs::*;
use std::io::*;
use std::process;
use colored::*;

struct FindInFile;

impl FindInFile {
    fn find_string() -> Result<()> {



        let argc: Vec<String> = args().collect();
        
        if argc[1] == "-h" {
            println!("Grepm, a remake of the grep command on linux, used to find something in a file\nSyntax : grepm <File> <Match> [OPTION]\nTo know the options, execute grepm --help")
        } else if argc[1] == "--help" {
            println!("Grepm, a remake of the grep command on linux, used to find something in a file\nSyntax : grepm <File> <Match>\nOptions:\n-q: quiet")
        }

        let mut file = File::open(argc[1].trim())?;
        let mut filecontent = String::new();
        let what: &str = &argc[2];

        file.read_to_string(&mut filecontent)?;

        file.read_to_string(&mut filecontent)?;

        let finding = filecontent.find(what);

        if argc.len() == 4 {
            let option = &argc[3];

            if option == "-q" {
            if finding == None {
                println!("{}", "No similarities has been found in this file");
                process::exit(1)
            } else {
                println!("''{}''{}{}", what," has been found on byte : ", finding.unwrap())
            }
            process::exit(1)
        }
        
        }
        
        

        println!("{} {} {}", what.red().bold(), "->".purple().bold(), argc[1].trim().truecolor( 79, 255, 112 ).bold());

        println!("{}", "Searching...".bold());

        
        if finding == None {
            println!("{}", "No similarities has been found in this file".red().bold());
            process::exit(1)
        } else {
            println!("''{}''{}{}", what.truecolor( 79, 255, 112 ).bold()," has been found on byte : ".truecolor( 79, 255, 112 ).bold(), finding.unwrap())
        }

        Ok(())
    }

}

fn main() {
    let _find = FindInFile::find_string();
}
