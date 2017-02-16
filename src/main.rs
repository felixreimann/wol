extern crate wol;
extern crate getopts;

use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [-h] [-4|-6]  MAC", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("",
                "mac",
                "the MAC address of the remote system",
                "00:00:00:00:00:00");
    opts.optflag("4", "ipv4", "use IPv4");
    opts.optflag("6", "ipv6", "use IPv6 (default)");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("Illegal argument: {}", f.to_string());
            return;
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let mac_str = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        println!("No MAC address given");
        print_usage(&program, opts);
        return;
    };
    let mac = wol::parse_mac(mac_str);
    match mac {
        Err(err) => {
            println!("Error during parsing of MAC address: {}", err);
            print_usage(&program, opts);
            return;
        },
        Ok(mac) => if matches.opt_present("4") {
            wol::send_magic_packet_v4(mac).unwrap_or_else(|err| {
                println!("Error during sending: {}", err);
                print_usage(&program, opts);
                return;
            });
        } else {
            wol::send_magic_packet_v6(mac).unwrap_or_else(|err| {
                println!("Error during sending: {}", err);
                print_usage(&program, opts);
                return;
            });
        },
    }
}
