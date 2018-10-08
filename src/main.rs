extern crate getopts;
extern crate tokio;

mod server;

use getopts::Matches;
use getopts::Options;
use std::env;

const DEFAULT_HOST: &'static str = "0.0.0.0";

fn usage(opts: &Options, args: &Vec<String>) {
    let executable = args[0].clone();
    let brief = format!("Usage: {} [-h host] -p PORT -d DIRNAME", executable);
    print!("{}", opts.usage(&brief));
}

fn parse_args(matches: Matches) -> Result<(String, u16, String), String> {
    let host: String = match matches.opt_str("h") {
        Some(x) => x,
        None => String::from(DEFAULT_HOST),
    };

    let port: u16 = match matches.opt_str("p") {
        Some(val) => {
            let pnumber = match val.parse::<u16>() {
                Ok(p) => p,
                Err(err) => {
                    let msg = format!("Cannot parse the port number {}", err.to_string());
                    return Err(msg);
                }
            };
            pnumber
        }
        None => {
            return Err(String::from("You must set the PORT"));
        }
    };

    let dirname: String = match matches.opt_str("d") {
        Some(val) => val,
        None => {
            return Err(String::from("You must set the directory name"));
        }
    };

    Ok((host, port, dirname))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("h", "host", "socket host", "HOST")
        .optopt("p", "port", "port number", "PORT")
        .optopt("d", "dir", "directory name", "DIRNAME");

    let matches = match opts.parse(&args[1..]) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Cannot parse CLI options: {}", err.to_string());
            usage(&opts, &args);
            return;
        }
    };

    let (host, port, dirname) = match parse_args(matches) {
        Ok(val) => val,
        Err(msg) => {
            eprintln!("Cannot parse args due to error: {}", msg);
            usage(&opts, &args);
            return;
        }
    };

    server::run_server(host, port, dirname);
}
