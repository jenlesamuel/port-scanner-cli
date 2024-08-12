# Port Scanner CLI
A command-line application written in Rust that scans the ports of a provided IP address to determine if they are open. The application is highly configurable, allowing you to specify the IP address, the range of ports to scan, and the number of threads to use for the scan.

## Features
- **Multi-threaded scanning**: Specify the number of threads to speed up the scanning process.

- **Customizable**: Provide the IP address and port range to scan via command-line arguments.

- **Help menu**: Display usage instructions with the -h flag.

## Installation
#### Clone the repository
```shell
git clone https://github.com/jenlesamuel/port-scanner-cli
cd port-scanner-cli
```

#### Build the project
Make sure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).
```shell
cargo build --release
```

#### Run the executable
```shell
./target/release/port-scanner-cli
```

## Usage

#### Scan ports
To scan ports use the **-n** flag to specify the number of threads, specify the **ip** to scan as an argument and specify the **scan** command:
```shell
./port-scanner-cli -n 75 127.0.0.1 scan
```

#### Help Menu
To see the help menu, use the -h flag
```shell
./port-scanner-cli -h
```
