### Understanding Youki

[youki](https://github.com/youki-dev/youki) is a already production ready and respected container runtime built in rust. My task is to, not copy, but understand the project's structure on GitHub.

## First Time seeing Youki
This is my first time seeing youki, and I am surprised at how compact the project is. Unlike a php-laravel or nextjs app that I'm used to at work, this doesn't have a lot of fluff.

# `main.rs`
I found the `main.rs` in `youki/crates/youki/src`. Besides main there are 2 folders: `commands` and `workload`. 

Even though, I have yet to use it, I see it is using `clap` which is a `Rust` `cli` crate. I build mine with raw `args` and I will definitely be using clap in my project.

It parses the options, then has a nested match statment and matches it to standard or common with matches like `StandardCmd::create`, `::start`, `::kill`, `::delete`.

I kinda understand matches, so I'm glad to see the code isn't insanely complex like I though.

I dont understand what the code does alreadt, but I see a few references to sys_calls and the main runction returns a result: `cmd_result`.

# `commands`
I suppose here are the `cli` "commands" but are actually just `fn` that I suppose are called from the cli.

Each `.rs` has the primary function and helper functions. In addition, (I still dont know testing in Rust), there are tests in the `.rs` function file.

# `workload`
I am not sure what this is. There are a few functions, but are beyond my task to understand.

# Others
There are 2 othe files `observability.rs` and `rootpath.rs` who I suspect are helper functions of some sort.


## `youki` vs `runc`
Besides being faster, `youki` is reported to be safer than `runc`. `runc` is written in `Go` and does not benefit for the security benefits of `Rust`. Besides being heavily adopted and mainted, `runc` has a mixed `C` implementation. Therefore, is prone to some security vulnerabilities.