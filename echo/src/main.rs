use std::env;

fn main() {
    let arg1 = std::env::args().nth(1).expect("no arguments");
    if arg1.starts_with("$"){
        if let Some(var_name) = arg1.strip_prefix("$"){
            match env::var(var_name) {
                Ok(val) => {
                    println!("{}", val);
                }
                Err(_) => {
                    println!("Environment variable {} not found", var_name)
                }
        }
    }
}
    println!("{}", arg1);
}