### What Even IS a Container?

## VMs vs. Containers

Both **VM**s and **containers** are ways of achieving virtualization and isolation.

**VMs** = hardware virtualization

- full guest OS. Heavier, but more isolation.

- Hardware/hypervisor
- `Virtualizing hardware` to create **virtual machines**. Separated by amounts of ram, CPU utilities and such.
- **Isolation**: appears like separate machines

**Containers** = OS-level virtualization.

- `hardware/ kernel` (hardware and os communication)/os/containers
- **Isolation**:
  - `Process isolation` - containers can only see what they are given and nothing else...

**Namespaces**-allow for appearance and customization that each container is a separate os
**cgroups**-monitoring and metering resources and enforce limits.

Containers use namespaces (isolation) + cgroups (resource limits).
Process isolation vs full machine isolation.

## Containers vs Container Images

Images are the container blueprint/template (similar to a class)
Container is a running instance (like an object of a class)

Key terms:
**Isolation**: from isolate - cause (a person or place) to be or remain alone or apart from others.
**VMs** = hardware virtualization
**Containers** = OS-level virtualization.
**Image** = container blueprints
**Process isolation** = containers can only see what they are given and nothing else
**Namespaces**- = allow for appearance and customization that each container is a separate os
**cgroups** = monitoring and metering resources

Summary:
`VM`s virtualize hardware to appear like seperate machines. Containers use namespaces and cgroups (from the linux kernel) to produce os-like isolation.

References:
https://youtu.be/cjXI-yxqGTI?si=OqmFkPBC7y5XU8mu