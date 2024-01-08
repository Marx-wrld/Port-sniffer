use std::{env, str::FromStr}; // env is a module that provides access to environment variables
use std::net::{IpAddr, TcpStream}; // net is a module that provides networking primitives
use std::io::{self, Write}; // io is a module that provides input and output
use std::process; // process is a module that provides access to the current process
use std::sync::mpsc::{Sender, channel}; // mpsc is a module that provides multiple producer, single consumer channels
use std::thread; // thread is a module that provides the spawn function

const MAX: u16 = 65535; // The maximum ports that we can sniff

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


fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16){
    // Sender<u16> is a Sender that sends a u16
    // start_port is the starting port
    // addr is the IP address
    // num_threads is the number of threads

    let mut port: u16 = start_port + 1; // start_port + 1 is assigned to the port variable
    loop {
        match TcpStream::connect((addr, port)) { 
            // std::net::TcpStream::connect((addr, port)) returns a TcpStream if the port is open
            Ok(_) => {
                // Ok(_) is a variant of Result that indicates success
                // if std::net::TcpStream::connect((addr, port)) returns Ok(_), then the port is open
                println!("{} is open", port);
                io::stdout().flush().unwrap(); // io::stdout().flush() flushes the standard output stream
                tx.send(port).unwrap_or_else(|error| eprintln!("{}", error));
                // tx.send(port) sends the port to the Sender
                // tx.send(port).unwrap_or_else(|error| eprintln!("{}", error)) returns an error if the port cannot be sent to the Sender
            }
            Err(_) => {} // Err(_) is a variant of Result that indicates failure
        }
        if (MAX - port) <= num_threads {
            // if (MAX - port) <= num_threads returns true, then the program will break out of the loop
            // if (MAX - port) <= num_threads returns false, then the program will continue to the next line
            break;
        }
        port += num_threads;
        // port += num_threads increments the port by the number of threads
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

    let num_threads = arguments.threads; // arguments.threads is the number of threads
    let (tx, rx) = channel(); // channel() returns a Sender and Receiver
    for i in 0..num_threads { // for i in 0..num_threads iterates through the number of threads
        let tx = tx.clone(); // tx.clone() returns a copy of the Sender
        thread::spawn(move || { // thread::spawn() spawns a thread
            scan(tx, i, arguments.ipaddr, num_threads); // scan() scans the IP address
        });
    }

    let mut out = vec![]; // vec![] returns a vector
    drop(tx); // drop(tx) drops the Sender
    for p in rx { // for p in rx iterates through the Receiver
        out.push(p); // out.push(p) pushes the port to the vector
    }

    println!("");
    out.sort(); // out.sort() sorts the vector
    for v in out { // for v in out iterates through the vector
        println!("{} is open", v);
    }
}
