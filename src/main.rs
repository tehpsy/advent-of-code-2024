mod day1;
mod day2;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <day>");
        return;
    }

    match args[1].as_str() {
        "1" => day1::run(),
        "2" => day2::run(),
        _ => eprintln!("Day not implemented"),
    }
}
