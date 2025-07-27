extern crate nix;
use nix::sched::{unshare, CloneFlags};
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, write}};
use std::process;

fn main() {
    println!("About to create PID namespace...");
    println!("My pid BEFORE unshare is {}", process::id());
    unshare(CloneFlags::CLONE_NEWPID).expect("Expected a new PID");
    
        match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Continuing execution in parent process, new child has pid: {}", child);
            waitpid(child, None).unwrap();        
        }
        Ok(ForkResult::Child) => {
            write(std::io::stdout(), "I'm a new child process\n".as_bytes()).ok();        
            println!("Child PID {}", process::id());
            process::exit(0);
        }
        Err(_) => println!("Fork failed"),
    }
}
