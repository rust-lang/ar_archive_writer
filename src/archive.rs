// Derived from code in LLVM, which is:
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Derived from https://github.com/llvm/llvm-project/blob/8ef3e895ad8ab1724e2b87cabad1dacdc7a397a3/llvm/include/llvm/Object/Archive.h

/// Size field is 10 decimal digits long
pub(crate) const MAX_MEMBER_SIZE: u64 = 9999999999;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArchiveKind {
    Gnu,
    Gnu64,
    Bsd,
    Darwin,
    Darwin64,
    Coff,
    AixBig,
}
