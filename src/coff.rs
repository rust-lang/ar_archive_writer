// Derived from code in LLVM, which is:
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[repr(u16)]
#[allow(clippy::upper_case_acronyms)]
pub enum MachineTypes {
    AMD64 = 0x8664,
    ARMNT = 0x1C4,
    ARM64 = 0xAA64,
    ARM64EC = 0xA641,
    ARM64X = 0xA64E,
    I386 = 0x14C,
}

impl From<MachineTypes> for u16 {
    fn from(val: MachineTypes) -> Self {
        val as u16
    }
}

pub fn is_arm64ec(machine: MachineTypes) -> bool {
    machine == MachineTypes::ARM64EC || machine == MachineTypes::ARM64X
}

pub fn is_any_arm64(machine: MachineTypes) -> bool {
    machine == MachineTypes::ARM64 || is_arm64ec(machine)
}

pub fn is_64_bit(machine: MachineTypes) -> bool {
    machine == MachineTypes::AMD64 || is_any_arm64(machine)
}

#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(u16)]
pub enum ImportType {
    /// An executable code symbol.
    Code = 0,
    /// A data symbol.
    Data = 1,
    /// A constant value.
    Const = 2,
}

impl From<ImportType> for u16 {
    fn from(val: ImportType) -> Self {
        val as u16
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
#[repr(u16)]
pub enum ImportNameType {
    /// Import is by ordinal. This indicates that the value in the Ordinal/Hint
    /// field of the import header is the import's ordinal. If this constant is
    /// not specified, then the Ordinal/Hint field should always be interpreted
    /// as the import's hint.
    Ordinal = 0,
    /// The import name is identical to the public symbol name
    Name = 1,
    /// The import name is the public symbol name, but skipping the leading ?,
    /// @, or optionally _.
    NameNoprefix = 2,
    /// The import name is the public symbol name, but skipping the leading ?,
    /// @, or optionally _, and truncating at the first @.
    NameUndecorate = 3,
    /// The import name is specified as a separate string in the import library
    /// object file.
    NameExportas = 4,
}

impl From<ImportNameType> for u16 {
    fn from(val: ImportNameType) -> Self {
        val as u16
    }
}
