### Rust Coding challenge

Day 9.5: Super Basic Rust Refresher

>Time: 15-20 minutes
>Goal: Get your Rust fingers moving again with container-themed practice.
>Simple Challenge:


```Rust
struct ProcessInfo{ 
    process_name: String,
    pid: i32,
    memory_limit: String
}

impl ProcessInfo {
    fn print_process_details(&self){
        println!("Process {} (PID: {}) has memory limit: {}", self.process_name, self.pid, self.memory_limit)
    }
}

fn main() {
    let process_1 =  ProcessInfo  {
        process_name: "bash".to_string(),
        pid: 1234,
        memory_limit: "100MB".to_string()
    };

    process_1.print_process_details();  

}
```