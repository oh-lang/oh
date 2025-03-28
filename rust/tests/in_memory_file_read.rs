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

#[test]
fn test_read_empty_file() {
    let path = FilePath::try_from("tests/empty-file.txt").expect("ok");
    let file = InMemoryFile::open(path).expect("ok");
    assert_eq!(&file.lines[..], []);
}

#[test]
fn test_read_file_no_empty_lines() {
    let path = FilePath::try_from("tests/non-empty-file.txt").expect("ok");
    let file = InMemoryFile::open(path).expect("ok");
    assert_eq!(
        &file.lines[..],
        [
            FileLine::try_from("we start with one line").expect("ok"),
            FileLine::try_from("and we go to the next").expect("ok"),
            FileLine::try_from("but no where do we live").expect("ok"),
            FileLine::try_from("but in between them").expect("ok"),
        ]
    );
}
