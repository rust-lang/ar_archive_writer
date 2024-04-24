// Derived from code in LLVM, which is:
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn get_arm64ec_mangled_function_name(name: &str) -> Option<String> {
    let first_char = name.chars().next().unwrap();
    let is_cpp_fn = first_char == '?';
    if is_cpp_fn && name.contains("$$h") {
        return None;
    }
    if !is_cpp_fn && first_char == '#' {
        return None;
    }

    let mut prefix = "$$h";
    let insert_idx = if is_cpp_fn {
        match name.find("@@") {
            Some(two_at_signs_idx) if Some(two_at_signs_idx) != name.find("@@@") => {
                two_at_signs_idx + 2
            }
            _ => name.find('@').map_or(name.len(), |idx| idx + 1),
        }
    } else {
        prefix = "#";
        0
    };

    Some(format!(
        "{}{prefix}{}",
        &name[..insert_idx],
        &name[insert_idx..]
    ))
}

pub fn get_arm64ec_demangled_function_name(name: &str) -> Option<String> {
    let first_char = name.chars().next().unwrap();
    if first_char == '#' {
        return Some(name[1..].to_string());
    }
    if first_char != '?' {
        return None;
    }

    match name.split_once("$$h") {
        Some((first, second)) if !second.is_empty() => Some(format!("{first}{second}")),
        _ => None,
    }
}
