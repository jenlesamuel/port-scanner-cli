#![deny(clippy::all)]

use std::io::Write;
use std::net::{TcpStream, IpAddr};
use std::{io, thread, env};
use std::sync::mpsc;

const MAX_PORT: i32 = 65535;

struct OpenPorts(Vec<i32>);

impl OpenPorts {
    fn new()-> Self {
        OpenPorts(vec![])
    }

    fn display(&self) {
        println!();
        for port in &self.0 {
            println!("{} is open", port);
        }
    }

    fn add(&mut self, port: i32) {
        self.0.push(port);
    }
}

#[derive(Debug)]
struct Arguments {
    program_name: String,
    option: Option<String>,
    num_threads : Option<i32>,
    ip: Option<IpAddr>,
    command : Option<String>,
}

impl Arguments {
    fn print_help(program_name: &str) {
        println!("A port scanner");
        println!();
        println!("Usage: {} [OPTION] [ARG] [COMMAND]", program_name);
        println!();
        println!("Options:");
        println!("-n\t\tThe number of threads to concurrently scan ports, must be greater than 0 and less than 65536");
        println!("-h\t\tPrint help");
        println!("Commands:");
        println!("scan\t\tScan ports");
    }
    
    fn parse_cli()-> Result<Arguments, Box<dyn std::error::Error>> {
        let args: Vec<String> = env::args().collect();
        let program_name = &args[0];
        let args_len = args.len();
        let mut arguments = Arguments {
            program_name: program_name.clone(),
            option: None,
            num_threads: None,
            ip: None,
            command: None,
        };

        let err = |err: &str, program_name: &str| ->  Result<Arguments, Box<dyn std::error::Error>> {
            Arguments::print_help(program_name);
            Err(err.into())
        };

        if args_len == 1 {
            return err("Insufficient arguments", program_name);
        }

        if args_len == 2 {
            if &args[1] == "-h" {
                arguments.option = Some("-h".to_string());
                return Ok(arguments);
            } else {
                return err("Bad arguments", program_name);
            }
        }

        if args_len > 2 && args_len < 5 {
            return err("Insufficient arguments", program_name);
        }

        if let Ok(num_threads) = args[2].parse::<i32>() {
            if num_threads <= 0 || num_threads >= MAX_PORT {
                return err("Bad arguments", program_name);
            }
            arguments.num_threads = Some(num_threads);
        } else {
            return err("Bad arguments", program_name);
        }

        if let Ok(ip) = args[3].parse::<IpAddr>() {
            arguments.ip = Some(ip);
        } else {
            return err("Bad arguments", program_name);
        }

        if &args[4] == "scan" {
            arguments.command = Some(args[4].clone());
        } else {
            return err("Bad arguments", program_name); 
        }
    
        Ok(arguments)
    }

    fn exec(&self) {

        if self.command.is_none() {
            Arguments::print_help(&self.program_name);
        } else {
            let mut open_ports = OpenPorts::new();
            let (tx, rx) = mpsc::channel::<i32>();
            let num_threads = self.num_threads.unwrap();
            let ip = self.ip.unwrap();
            
            for mut port in 1..=num_threads{
                let tx_clone = tx.clone();
                thread::spawn( move|| {
                    loop {
                        let address = format!("{}:{}", ip, port);
                        
                        if TcpStream::connect(&address).is_ok() {
                            print!(".");
                            io::stdout().flush().unwrap();
                            tx_clone.send(port).unwrap();
                        }
                        port += num_threads;
                        if port > MAX_PORT {
                            break;
                        }
                    }
                });
            }

            drop(tx);
            
            for p in rx {
                open_ports.add(p);
            }

            open_ports.display();
        }
    }
}


fn main()-> Result<(), Box<dyn std::error::Error>> {

    let result = Arguments::parse_cli();
    if let Err(e) = result {
        Err(e)
    } else {
        let arguments = result.unwrap();
        arguments.exec();
        Ok(())
    }
}