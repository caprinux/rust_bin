use std::io;
use std::io::BufRead;
use std::fs;
use std::path::Path;
use clap::Parser;

#[derive(Parser)]
#[clap(name="cat")]
#[clap(version)]
#[clap(about = "Concatenate FILE(s) to standard output.", long_about = None)]
struct CliArgs {

    /// display $ at end of each line
    #[clap(short='E', long="show-ends")]
    ends: bool,

    /// number all output lines
    #[clap(short, long)]
    number: bool,

    #[clap(multiple_occurrences=true)]
    files: Option<String>,
}


fn main() {
    let args = CliArgs::parse();

    if args.files == None {
        read_from_stdin(args.ends, args.number);
    } else {
        read_from_file(args.ends, args.number, args.files.unwrap());
    }
}

fn read_from_file(ends:bool, number:bool, file_name: String) {
    let mut num = 0;
    let file_exists: bool = Path::new(&file_name).is_file();
    if file_exists == false {
        println!("cat: {}: No such file or directory", file_name);
        std::process::exit(1);
    }

    let file = fs::File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let mut to_print = line.unwrap().trim().to_string();
        if ends {
            to_print.push_str("$");
        }
        if number {
            to_print = format!("     {}  {}", num, to_print);
        }
        println!("{}", to_print);
        num += 1;
    }
}

fn read_from_stdin(ends: bool, number: bool) {
    let mut num = 0;
    loop {
        // get input
        let mut input = String::new();
        _ = io::stdin().read_line(&mut input);
        input = input.trim().to_string();

        // process flags
        let mut to_print = input.clone();
        if ends {
            to_print.push_str("$");
        }
        if number {
            to_print = format!("     {}  {}", num, to_print);
        }
        println!("{}", to_print);
        num += 1;
    }

}
