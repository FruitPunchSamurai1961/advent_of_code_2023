use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_delimiter = ',', num_args = 1..)]
    days: Vec<i32>
}



fn main() {
    let args =  Args::parse();
    println!("Args is {:#?}", args);
}
