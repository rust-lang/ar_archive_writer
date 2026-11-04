use std::io::Cursor;

use ar_archive_writer::{ArchiveKind, NewArchiveMember};
use object::{Architecture, BinaryFormat, Endianness, write};

mod common;

/// Tests archives whose member offsets do not fit in 32 bits, by lowering the
/// threshold through `SYM64_THRESHOLD` for both `llvm-ar` and `ar_archive_writer`.
#[test]
fn sym64_threshold() {
    unsafe { std::env::set_var("SYM64_THRESHOLD", "100") };
    common::generate_archive_and_compare(
        "sym64_threshold",
        |architecture, subarch, endianness, binary_format| {
            let mut object1 = write::Object::new(binary_format, architecture, endianness);
            object1.set_sub_architecture(subarch);
            common::add_file_with_functions_to_object(
                &mut object1,
                b"file1.c",
                &[b"func1", b"func2"],
            );

            let mut object2 = write::Object::new(binary_format, architecture, endianness);
            object2.set_sub_architecture(subarch);
            common::add_file_with_functions_to_object(
                &mut object2,
                b"file2.c",
                &[b"func3", b"func4"],
            );

            vec![
                ("file_with_a_long_name_1.o", object1.write().unwrap()),
                ("file_with_a_long_name_2.o", object2.write().unwrap()),
            ]
        },
    );
}

/// Tests that an ARM64X archive, which needs both COFF symbol maps, is
/// rejected instead of being written as GNU64.
#[test]
fn sym64_threshold_arm64x() {
    unsafe { std::env::set_var("SYM64_THRESHOLD", "100") };
    let objects = [Architecture::Aarch64, Architecture::X86_64].map(|architecture| {
        let mut object = write::Object::new(BinaryFormat::Coff, architecture, Endianness::Little);
        common::add_file_with_functions_to_object(&mut object, b"file.c", &[b"sym"]);
        object.write().unwrap()
    });
    let members = objects
        .iter()
        .enumerate()
        .map(|(index, bytes)| {
            NewArchiveMember::new(
                bytes,
                &ar_archive_writer::DEFAULT_OBJECT_READER,
                format!("file_with_a_long_name_{index}.obj"),
            )
        })
        .collect::<Vec<_>>();
    let mut output = Cursor::new(Vec::new());
    let error = ar_archive_writer::write_archive_to_stream(
        &mut output,
        &members,
        ArchiveKind::Coff,
        false,
        Some(true),
    )
    .unwrap_err();
    assert_eq!(
        error.to_string(),
        "Archive is too large: ARM64X does not support archives larger than 4GB"
    );
    assert!(output.into_inner().is_empty());
}
