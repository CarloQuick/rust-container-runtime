### Week 1 Reflection

## Containers if explained to a 5th grader
Write a 1-paragraph explanation of containers for someone who's never heard of them
Use your own analogies (Michael Scott, Dwight, office building)

Consider The Office episode where Dwight takes Michael Scott to the wilderness and leaves him to fend for himself. Once left, Michael thinks he's alone, and completely exposed to the outdoors. But we then see that Dwight, who is an outdoor expert, is always very close, and constantly observes him. He has the power to end Michael's perceinved isolation, and eventually does.

This is similar to how a container works. We can have a parent namespace (with cgroup) that spawns a child process. The parent can see everything inside the child process, but the child thinks only it exists. It thinks it's PID 1, and that its filesystem is completely isolated. It can see itself but not the parent process. Dwight = parent and Michael = child.

Additionally, the parent can control the resources given to the container. Another Office example is when Dwight buys the building Dunder Mifflin rents. He, being cheap and stingy, cuts corners by limiting resources to those working in the office. He limits the ply of toilet paper, locks the AC control unit and limits the amount of electricty allowed. Those in the office are resource limited because of the parent (Dwight). Again, this is similar to a cgroup which is a way of managing resources for processes. Want to limit how much memory the cgroup can have? The parent process, has the ability. The child doesnt have control and has to abide by the rules given to its cgroup.

## How does everything fit together?
Draw or write how all the pieces fit together: VMs → containers → namespaces → cgroups → Docker

VMs virtualize hardware. They are percieved as entirely isolated machines. These are resource heavy. Containers, as described above, virtualize the operating system. They are lighter then vms. 

Containers use namespaces to give child namespaces percieved isolation and cgroups to manage their resources.

Docker can utilize all of these. Containers are native to the linux kernel, so Docker just works on linux. On windows or mac, a linux vm coordinates the management of containers.

Usually there is a the docker cli, that allows developers to interact with Docker Desktop. The desktop is a Docker Daemon, dockerd, that is a server for communicating via REST API. (It's needed to download images and packages).

## Confidence Assessment
What concepts feel solid vs still confusing?
 - I am not confused by anything really, but want to know how the kernel can do all of this. Especially, how it can spin of namespaces and the "magic filesystem" when managing cgroups.
What questions do you still have?
- n/a
Are you excited to keep going?
- better fucking believe it