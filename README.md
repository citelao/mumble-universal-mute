# Mumble mute plugin

Written in Rust

## Develop

Setup:

```powershell
# Install LLVM/Clang
winget install llvm

# Patch Rust bindings plugin
# TBD

# Download the Mumble source code & symlink to it.
# The Rust bindings are generated fresh from the plugins/ dir.
# https://github.com/mumble-voip/mumble
New-Item -ItemType SymbolicLink mumble_sources -Value C:\Users\Path\To\mumble\
```

Inner loop:

```powershell
rust build
```

### Tips

* Make sure you have 64-bit Mumble!
* Enable Developer console (or use SysInternals DebugView)