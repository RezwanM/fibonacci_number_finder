# nth_fibonacci_number

This repo contains code for a Fibonacci number finder where the user can find the nth Fibonacci number in the sequence.

The project was written in Rust.

## Preview

    $ cd <project_root> 
    $ cargo run
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
         Running `target\debug\fibonacci_number.exe`
    Welcome to the nth Fibonacci number finder app!
    Enter the position of the Fibonacci number in the sequence (index starts at 0):
    17
    The Fibonacci number in position 17 is: 2584

## Requirements

- rustc 1.84.0 (9fc6b4312 2025-01-07)

## Setup
The following instructions assume the user has Ubuntu as their local machine's OS. Most instructions should work for other Linux distributions as well, though mileage may vary.

### Step 1: Install Rust
Set up Rust (v1.84.0) on the local machine.

    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    
### Step 2: Clone the project
Clone this GitHub repository into the local machine.
    
    git clone --single-branch -b main <project_repo_url> <project_root> 
    
### Step 3: Resolve dependencies 
Install missing libraries required for running the project (if any).
    
    cargo build
    
### Step 4: Run the application
Run the application from the command-line.

## Usage
    
    cd <project_root>
    cargo run

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
