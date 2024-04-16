# LLVM Reference Files

These are a copy of the relevant LLVM files that were ported to Rust from the
last time that this project was "synced" with LLVM.

Currently that sync point is 18.1.3, commit [ef6d1ec](https://github.com/llvm/llvm-project/tree/ef6d1ec07c693352c4a60dd58db08d2d8558f6ea).

These files were originally located at:
* `llvm/include/llvm/Object/Archive.h`
* `llvm/include/llvm/Object/ArchiveWriter.h`
* `llvm/include/llvm/Object/COFFImportFile.h`
* `llvm/include/llvm/Support/Alignment.h`
* `llvm/include/llvm/Support/MathExtras.h`
* `llvm/lib/Object/ArchiveWriter.cpp`

When syncing, make sure to update these files and the commit above.

Additionally, `ar_archive_writer` has removed some options, so you can assume:
* `deterministic` is always `true`.
* `write_symtab` is always `true`.
