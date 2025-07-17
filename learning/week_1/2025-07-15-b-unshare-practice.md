### Unshare (2025- 07-09)

## After learning about

**namespaces and cgroups**

I moved onto how **namespaces** are created on the linux kernel using the [unshare](https://man7.org/linux/man-pages/man1/unshare.1.html) command.
Unshare will run a program in a new namespace. It creates the namespace then executes the program. This namespace is ‘unshared’ from the parent namespace.

UTS namespace
To test this I follow a tutorial for creating a UTS (hostname and domain name) namespace.

I checked the hostname while at root. Then I unshared which created a new uts namespace and ran a shell. Here I renamed and verified the the hostname of the new namespace. After exiting, I checked that I was outside the child namespace by again checking the the host namespace hostname.
This taught me a very simple approach for creating a low risk, and visual first namespace. Next, I moved on testing a PID namespace.
Pid namespace

PID namespace

## References

I got a chance to run `unshare --pid --f --mount-proc /bin/bash`

After looking at the pid namespace from with in the pid, there were only two calls and the first being pid 1. From outside, I could see the same call but as pid 17. This shows that the parent could see the process as just another of its own. But the child thought it was on its own.

[unshare command doesn't create new PID namespace](https://stackoverflow.com/questions/68704803/unshare-command-doesnt-create-new-pid-namespace)
[unshare | man-page](https://man7.org/linux/man-pages/man1/unshare.1.html)
[What does echo $$ do?](https://unix.stackexchange.com/questions/729082/what-does-echo-do)
