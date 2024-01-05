use std::{env, net::IpAddr, str::FromStr}; // env is a module that provides access to environment variables
use std::str::process; // process is a module that provides access to the current process

struct Arguments { // Arguments is a struct that contains the flag, ipaddr, and threads fields
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments { // impl is a keyword that defines an implementation block
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        // args is a slice of String
        // &[String] is a slice of String
        // &str is a string slice
        // &'static str is a string slice that lives for the entire duration of the program
        // Result is an enum that returns either Ok or Err
        // Ok is a variant of Result that indicates success
        // Err is a variant of Result that indicates failure
        // Result<Arguments, &'static str> is a Result that returns either Arguments or &'static str

        if args.len() < 2 {
            // args.len() returns the number of elements in the slice
            // args.len() < 2 returns true if the number of elements in the slice is less than 2
            // args.len() < 2 returns false if the number of elements in the slice is greater than or equal to 2
            // if args.len() < 2 returns true, then the program will return Err("not enough arguments")
            // if args.len() < 2 returns false, then the program will continue to the next line
            return Err("not enough arguments");
        } else if args.len() > 4 {
            // args.len() returns the number of elements in the slice
            // args.len() > 4 returns true if the number of elements in the slice is greater than 4
            // args.len() > 4 returns false if the number of elements in the slice is less than or equal to 4
            // if args.len() > 4 returns true, then the program will return Err("too many arguments")
            // if args.len() > 4 returns false, then the program will continue to the next line
            return Err("too many arguments");
        }

        let flag = args[1].clone(); 
        // args[1] is the first argument passed to the program
        // args[1].clone() returns a copy of the first argument passed to the program
        // args[1].clone() is assigned to the flag variable

        if let Ok(ipaddr) = IpAddr::from_str(&flag) {
            // IpAddr::from_str(&flag) returns an IpAddr if the flag is a valid IP address
            return Ok(Arguments { flag: String::from(""), ipaddr, threads: 4 }); 
            // return an Arguments struct with the ipaddr field set to the IP address
        } else {
            let flag = args[1].clone(); // args[1] is the first argument passed to the program
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                // flag.contains("-h") returns true if the flag contains "-h"
                // flag.contains("-help") returns true if the flag contains "-help"
                // args.len() == 2 returns true if the number of elements in the slice is equal to 2
                // if flag.contains("-h") || flag.contains("-help") && args.len() == 2 returns true, then the program will print the help message
                // if flag.contains("-h") || flag.contains("-help") && args.len() == 2 returns false, then the program will continue to the next line
                println!("Usage: -j to select how many threads you want\r\n -h or -help to show this help message");
                return Err("help");
        } else if flag.contains("-h") || flag.contains("-help")  { 
            // flag.contains("-h") returns true if the flag contains "-h"
            return Err("too many arguments");
        } else if flag.contains("-j") { 
            // flag.contains("-j") returns true if the flag contains "-j"
            let ipaddr =  match IpAddr::from_str(&args[3]) { 
                // IpAddr::from_str(&args[3]) returns an IpAddr if the flag is a valid IP address
                Ok(s) => s,
                Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6"),
            };
            let threads = match args[2].parse::<u16>() { 
                // args[2].parse::<u16>() returns a u16 if the flag is a valid number
                Ok(s) => s,
                Err(_) => return Err("failed to parse thread number"),
            };
            return Ok(Arguments { threads, flag, ipaddr }); 
            // return an Arguments struct with the threads, flag, and ipaddr fields set to the number of threads, flag, and IP address
        } else {
            return Err("invalid syntax");
        }
    }
}
}

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() returns a vector of the arguments passed to the program

    let program = args[0].clone(); 
    // args[0] is the name of the program  
    // args[0].clone() returns a copy of the name of the program

    let arguments = Arguments::new(&args).unwrap_or_else( 
        // Arguments::new(&args) returns an Arguments struct
        |err| { 
            // |err| is a closure that takes an error as an argument
            if err.contains("help") { // err.contains("help") returns true if the error contains "help"
                process::exit(0);
            } else { // err.contains("help") returns false if the error does not contain "help"
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0); // process::exit(0) exits the program
            }
        }
    );
}
