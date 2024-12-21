struct CliArgs {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern: String = std::env::args().nth(1).expect( "no pattern given");
    let path: String = std::env::args().nth(2).expect("no path given");

    let args: CliArgs = CliArgs {
        pattern,
        path: std::path::PathBuf::from(path),
    };

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}


