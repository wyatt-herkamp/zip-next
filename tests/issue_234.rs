use zip_next::result::ZipError;

const BUF: &[u8] = &[
    0, 80, 75, 1, 2, 127, 120, 0, 3, 3, 75, 80, 232, 3, 0, 0, 0, 0, 0, 0, 3, 0, 1, 0, 7, 0, 0, 0,
    0, 65, 0, 1, 0, 0, 0, 4, 0, 0, 224, 255, 0, 255, 255, 255, 255, 255, 255, 20, 39, 221, 221,
    221, 221, 221, 221, 205, 221, 221, 221, 42, 221, 221, 221, 221, 221, 221, 221, 221, 38, 34, 34,
    219, 80, 75, 5, 6, 0, 0, 0, 0, 5, 96, 0, 1, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 234, 236, 124,
    221, 221, 37, 221, 221, 221, 221, 221, 129, 4, 0, 0, 221, 221, 80, 75, 1, 2, 127, 120, 0, 4, 0,
    0, 2, 127, 120, 0, 79, 75, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0,
    234, 0, 0, 0, 3, 8, 4, 232, 3, 0, 0, 0, 255, 255, 255, 255, 1, 0, 0, 0, 0, 7, 0, 0, 0, 0, 3, 0,
    221, 209, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 58, 58, 42, 75, 9, 2, 127,
    120, 0, 99, 99, 99, 99, 99, 99, 94, 7, 0, 0, 0, 0, 0, 0, 213, 213, 213, 213, 213, 213, 213,
    213, 213, 7, 0, 0, 211, 211, 211, 211, 124, 236, 99, 99, 99, 94, 7, 0, 0, 0, 0, 0, 0, 213, 213,
    213, 213, 213, 213, 213, 213, 213, 7, 0, 0, 211, 211, 211, 211, 124, 236, 234, 0, 0, 0, 3, 8,
    0, 0, 0, 12, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 0, 0, 58, 58, 58, 42, 175, 221, 253, 221,
    221, 221, 221, 221, 80, 75, 9, 2, 127, 120, 0, 99, 99, 99, 99, 99, 99, 94, 7, 0, 0, 0, 0, 0, 0,
    213, 213, 213, 213, 213, 213, 213, 213, 213, 7, 0, 0, 211, 211, 211, 211, 124, 236, 221, 221,
    221, 221, 221, 80, 75, 9, 2, 127, 120, 0, 99, 99, 99, 99, 99, 99, 94, 7, 0, 0, 0, 0, 0, 0, 213,
    213, 213, 213, 213, 213, 213, 213, 213, 7, 0, 0, 211, 211, 211, 211, 124, 236,
];

#[test]
fn invalid_header() {
    let reader = std::io::Cursor::new(&BUF);
    let archive = zip_next::ZipArchive::new(reader);
    match archive {
        Err(ZipError::InvalidArchive(_)) => {}
        value => panic!("Unexpected value: {value:?}"),
    }
}
