use oh::core::*;

#[test]
fn test_read_file() {
    let path = FilePath::try_from("tests/sample-file.txt").expect("ok");
    let file = InMemoryFile::open(path).expect("ok");
    assert_eq!(
        &file.lines[..],
        [
            FileLine::try_from("").expect("ok"),
            FileLine::try_from("This is a sample test file").expect("ok"),
            FileLine::try_from("").expect("ok"),
            FileLine::try_from("It will be used to verify that file-reading works.").expect("ok"),
            FileLine::try_from("That is pretty much it.").expect("ok"),
            FileLine::try_from("").expect("ok"),
        ]
    );
}
