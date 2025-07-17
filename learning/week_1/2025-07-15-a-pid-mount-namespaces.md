### Namespaces

**Namespaces** allow different aspects of the operating system to be independently modified: process tree, networking interfaces, mount points, IPC, and more.

# `unshare`:

**`unshare`** - creates new namespaces and executes programs within them

## PID Namespaces

How it works:

- Creates a new process tree with its own PID 1 process
- The process creating the namespace remains in the parent namespace
- Child becomes root of its own isolated process tree

# Isolation behavior:

- Child perspective: Sees itself as PID 1, believes it's alone in the system
- Parent perspective: Still sees everything, maintains full visibility and control

\*_Key asymmetry_: Child can't see parent, but parent sees child

## Your Michael Scott/Dwight Analogy for PID Namespaces:

# The Setup:

**Michael Scott** = process inside the PID namespace
**Dwight Schrute** = parent/host system watching from outside
"Surviving alone in the wild" = Michael thinks he's the only process (PID 1)

# What It Demonstrates:

- Michael's perspective (child namespace):
- Believes he's completely alone and in charge
- Thinks he's "PID 1" - the most important process
- Has no idea Dwight (or anyone else) exists
- Lives in blissful ignorance of the bigger world
- Dwight's perspective (parent namespace):
- Can see everything Michael is doing (full visibility)
- Knows Michael isn't actually alone or in charge
- Could intervene at any time (has control)
- Watching through a "sniper scope" = monitoring without being detected

# Technical Translation:

Container thinks it's the whole system, host knows it's just one isolated process among many.

## Mount Namespaces

Purpose: Creates virtual filesystem isolation
How it works:

- Mounts a virtual filesystem and assigns location as root
- Changes don't affect the host system (except shared filesystems)
- Creates temporary file structure for the process

Key insight: Mounting and unmounting operations are isolated from the rest of the system
Tools

`unshare` --help shows extensive options for creating namespaces and cgroups
