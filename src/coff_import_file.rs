// Derived from code in LLVM, which is:
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub(crate) const IMPORT_DESCRIPTOR_PREFIX: &[u8] = b"__IMPORT_DESCRIPTOR_";
pub(crate) const NULL_IMPORT_DESCRIPTOR_SYMBOL_NAME: &[u8] = b"__NULL_IMPORT_DESCRIPTOR";
pub(crate) const NULL_THUNK_DATA_PREFIX: &[u8] = b"\x7f";
pub(crate) const NULL_THUNK_DATA_SUFFIX: &[u8] = b"_NULL_THUNK_DATA";
