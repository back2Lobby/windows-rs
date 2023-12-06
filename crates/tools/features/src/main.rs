fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 1 {
        println!(
            r#"Usage: windows-features.exe <name>
"#
        );
    } else {
        match windows_bindgen::features(&args[0]) {
            Ok(ok) => println!("{}", ok),
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }
    }
}
