use std::{env, process};

use simple_payment_engine::process_csv_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("please specify an input file");
        process::exit(1);
    }

    match process_csv_file(&args[1]) {
        Err(error) => {
            eprintln!("{}", error);
        }
        Ok(accounts) => {
            println!("client,available,held,total,locked");
            accounts.iter().for_each(|account| {
                let available = account.1.available;
                let held = account.1.held;
                let total = available + held;
                let locked = account.1.locked;
                println!("{},{},{},{},{}", account.0, available, held, total, locked);
            })
        }
    }
}
