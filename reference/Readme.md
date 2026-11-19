# LLVM Reference Files

These are a copy of the relevant LLVM files that were ported to Rust from the
last time that this project was "synced" with LLVM.

Currently that sync point is 23.1.6, commit [5b10d2d](https://github.com/llvm/llvm-project/tree/5b10d2d74929eae78912771222c9855af97496f4).

These files were originally located at:
* `llvm/include/llvm/BinaryFormat/COFF.h`
* `llvm/include/llvm/BinaryFormat/GOFF.h`
* `llvm/include/llvm/Object/Archive.h`
* `llvm/include/llvm/Object/ArchiveWriter.h`
* `llvm/include/llvm/Object/COFFImportFile.h`
* `llvm/include/llvm/Support/Alignment.h`
* `llvm/include/llvm/Support/MathExtras.h`
* `llvm/lib/IR/Mangler.cpp`
* `llvm/lib/Object/ArchiveWriter.cpp`
* `llvm/lib/Object/COFFImportFile.cpp`
* `llvm/unittests/IR/ManglerTest.cpp`
* `llvm/lib/Support/ConvertEBCDIC.cpp`

When syncing, make sure to update these files and the commit above.

Additionally, `ar_archive_writer` has removed some options, so you can assume:
* `deterministic` is always `true`.
* `write_symtab` is always `true`.
