# LLVM Reference Files

These are a copy of the relevant LLVM files that were ported to Rust from the
last time that this project was "synced" with LLVM.

Currently that sync point is 15.0.0-rc3, commit [8ef3e895a](https://github.com/llvm/llvm-project/tree/3d3ef9d073e1e27ea57480b371b7f5a9f5642ed2).

These files were originally located at:
* `llvm/include/llvm/Support/Alignment.h`
* `llvm/include/llvm/Object/Archive.h`
* `llvm/include/llvm/Object/ArchiveWriter.h`
* `llvm/lib/Object/ArchiveWriter.cpp`

When syncing, make sure to update these files and the commit above.

Additionally, `ar_archive_writer` has removed some options, so you can assume:
* `deterministic` is always `true`.
* `write_symtab` is always `true`.
