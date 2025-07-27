### Syscalls in rust
To find out why I should be  rust for making syscalls, I turned to the [youki](https://github.com/youki-dev/youki) GitHub repo.

To make containers, system calls (syscalls) to the Linux kernel are necessary. 

According to `youki` this is easier done in Rust vs Go. Additionally, it is better making syscalls in Rust over C, for example, because of Rust's enforced memory safety.

From what I can gather, Rust is a great systems programming language, and has use of the `nix` crate

## nix
[nix](https://docs.rs/nix/latest/nix/sys/socket/index.html) and the [nix crate](https://docs.rs/nix/latest/nix/) allows for syscalls in rust for using syscalls within normal rust functions. It seems easy and benefits from Rust's memory management.

Rust is also pretty fast.

Refs:
https://docs.rs/nix/latest/nix/sys/socket/index.html
https://github.com/youki-dev/youki
https://docs.rs/nix/latest/nix/
https://www.linkedin.com/pulse/mastering-unix-system-calls-rusts-nix-crate-luis-soares-m-sc-/
