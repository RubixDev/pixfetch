---
name: New info type request
about: Request a new type of system information to show
title: "[info request] "
labels: enhancement
assignees: RubixDev

---

<!-- This template uses "Memory" as an example, replace the contents of every section with your requested type accordingly -->

**Name of information type**
Memory

**Description of information type**
The currently used and total RAM of the system

**Example value**
`2.50GB / 3.77GB`

<!-- The following section is not required, but desired -->
**Possible way to get the value in Rust**
Part of the already used `sysinfo` crate: [`sysinfo::SystemExt::used_memory`](https://docs.rs/sysinfo/latest/sysinfo/trait.SystemExt.html#tymethod.used_memory) and [`sysinfo::SystemExt::total_memory`](https://docs.rs/sysinfo/latest/sysinfo/trait.SystemExt.html#tymethod.total_memory) after refreshing [with memory](https://docs.rs/sysinfo/latest/sysinfo/struct.RefreshKind.html#method.with_memory)

<!-- Alternatively provide a Rust codeblock -->
