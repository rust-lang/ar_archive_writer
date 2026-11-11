// Derived from code in LLVM, which is:
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn get_arm64ec_mangled_function_name(name: &str) -> Result<Option<String>, ()> {
    let first_char = name
        .chars()
        .next()
        .expect("get_arm64ec_mangled_function_name requires non-empty name");
    if first_char != '?' {
        // For non-C++ symbols, prefix the name with "#" unless it's already
        // mangled.
        if first_char == '#' {
            return Ok(None);
        }
        return Ok(Some(format!("#{name}")));
    }

    // If the name contains $$h, then it is already mangled.
    if name.contains("$$h") {
        return Ok(None);
    }

    // Handle MD5 mangled names, which use a slightly different rule from
    // other C++ manglings.
    //
    // A non-Arm64EC function:
    //
    // ??@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa@
    //
    // An Arm64EC function:
    //
    // ??@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa@$$h@
    if name.starts_with("??@") && name.ends_with('@') {
        return Ok(Some(format!("{name}$$h@")));
    }

    // Ask the demangler where we should insert "$$h".
    // TODO: In LLVM, this uses the Microsoft Demangler to demangle the name to
    // find the insertion point. The demangler is massive, so rather than
    // porting it we're going to require that the name is already mangled.
    Err(())
}

pub fn get_arm64ec_demangled_function_name(name: &str) -> Option<String> {
    // For non-C++ names, drop the "#" prefix.
    let first_char = name.chars().next().unwrap();
    if first_char == '#' {
        return Some(name[1..].to_string());
    }
    if first_char != '?' {
        return None;
    }

    // MD5 mangled name; see comment in get_arm64ec_mangled_function_name.
    if name.starts_with("??@") && name.ends_with("@$$h@") {
        return Some(name[..name.len() - 4].to_string());
    }

    // Drop the ARM64EC "$$h" tag.
    match name.split_once("$$h") {
        Some((first, second)) if !second.is_empty() => Some(format!("{first}{second}")),
        _ => None,
    }
}
