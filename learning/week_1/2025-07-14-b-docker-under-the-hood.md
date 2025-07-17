### Docker Under the Hood

## Architecture Flow:

**CLI** → **Daemon** (via REST API) → **containerd**→ **runc** → actual **namespaces**/**cgroups**
Docker orchestrates; runc does the low-level container creation work.

**Key Insight**: A container is fundamentally a process with "blinders and limitations" - these are namespaces (isolation) and cgroups (resource limits).

## Namespaces

**Purpose**: Provide process isolation by creating "invisible fences" - giving containers the illusion of running on separate systems.
Types:

- **PID** – Process isolation. Container gets its own process tree starting from PID 1
- **NET** – Network isolation. Own network interfaces, routing tables, firewall rules
- **IPC** – Inter-Process Communication isolation. Separate message queues, semaphores, shared memory
- **MNT** – Mount isolation. Container gets its own filesystem view and mount points
- **UTS** – Hostname/domain isolation. Container can have its own hostname
- **USER** – User ID isolation (you missed this one - maps container users to host users)

## cgroups

**Purpose**: Resource management and limits enforcement (not just sharing).

Key cgroups:

- memory, cpu, cpuset, blkio, net_cls, devices, freezer

## Docker Components

**Docker Client (CLI)**: Interface for user commands
**Docker Daemon (dockerd)**: REST API server managing containers, images, networks
**containerd**: High-level container runtime managing container lifecycle
**runc**: Low-level runtime that actually creates namespaces/cgroups

## Summary:

Docker is a user-friendly orchestration layer over Linux kernel isolation primitives.

## References

https://docs.docker.com/get-started/docker-overview/
