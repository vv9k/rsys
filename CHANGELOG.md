# 0.6.0
- rename linux `kernel_version` to `kernel_release`
- add `Error::NixSyscallError`
- functions that returned `Error::FfiError` now return unified `Error::NixSyscallError`
- remove `Memory` struct from linux impl and all associated functions
- add `SysInfo` struct that replaces `Memory`
