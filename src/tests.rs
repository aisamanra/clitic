use crate::*;

#[test]
fn basic_file_ref_test() {
    let mut table = FileTable::new();
    let f1 = table.add_file("sample.txt", "foo\nbar\n".to_string());
    let f2 = table.add_file("other.txt", "whatever\n".to_string());
    let f3 = table.add_repl_line("stuff\n".to_string());
    let f4 = table.add_repl_line("other stuff\n".to_string());

    assert_eq!(table.get_content(f1), "foo\nbar\n");
    assert_eq!(table.get(f1).source, FileSource::File("sample.txt".into()));

    assert_eq!(table.get_content(f2), "whatever\n");
    assert_eq!(table.get(f2).source, FileSource::File("other.txt".into()));

    assert_eq!(table.get_content(f3), "stuff\n");
    assert_eq!(table.get(f3).source, FileSource::Repl(0));

    assert_eq!(table.get_content(f4), "other stuff\n");
    assert_eq!(table.get(f4).source, FileSource::Repl(1));
}

#[test]
fn show_span_test() {
    let mut table = FileTable::new();
    let f = table.add_file(
        "sample.txt",
        "foo\nwhatever, that's just like, your opinion, man\nbaz\n".to_string(),
    );

    let l = Loc::new(Span::new(4, 12), f);
    assert_eq!(
        table.get_line(l).unwrap(),
        [
            "  2 |whatever, that's just like, your opinion, man",
            "     ^^^^^^^^",
        ]
        .join("\n"),
    );
}
