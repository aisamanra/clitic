#![cfg(test)]
mod tests;

/// A location in a source file
#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Span {
        Span { start, end }
    }

    pub fn empty() -> Span {
        Span {
            start: u32::MAX,
            end: u32::MAX,
        }
    }

    pub fn exists(&self) -> bool {
        self.start != u32::MAX && self.end != u32::MAX
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Loc {
    pub span: Span,
    pub file: FileRef,
}

impl Loc {
    pub fn new(span: Span, file: FileRef) -> Loc {
        Loc { span, file }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Located<T> {
    pub item: T,
    pub loc: Loc,
}

impl<T> Located<T> {
    pub fn new(item: T, file: FileRef, span: Span) -> Located<T> {
        Located {
            loc: Loc { file, span },
            item,
        }
    }
}

impl<T: Clone> Located<T> {
    pub fn map<R>(&self, func: impl FnOnce(T) -> R) -> Located<R> {
        Located {
            item: func(self.item.clone()),
            loc: self.loc,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FileSource {
    File(std::path::PathBuf),
    Repl(u32),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FileRef {
    idx: usize,
}

pub struct File {
    pub source: FileSource,
    pub content: String,
}

pub struct FileTable {
    files: Vec<File>,
    last_repl_line: u32,
}

impl FileTable {
    pub fn new() -> FileTable {
        FileTable {
            files: Vec::new(),
            last_repl_line: 0,
        }
    }

    pub fn add_file(&mut self, path: impl Into<std::path::PathBuf>, content: String) -> FileRef {
        self.add_file_from_source(FileSource::File(path.into()), content)
    }

    pub fn add_repl_line(&mut self, content: String) -> FileRef {
        let source = FileSource::Repl(self.last_repl_line);
        self.last_repl_line += 1;
        self.add_file_from_source(source, content)
    }

    fn add_file_from_source(&mut self, source: FileSource, content: String) -> FileRef {
        let idx = self.files.len();
        self.files.push(File { source, content });
        FileRef { idx }
    }

    pub fn get(&self, rf: FileRef) -> &File {
        &self.files[rf.idx]
    }

    pub fn get_content(&self, rf: FileRef) -> &str {
        &self.get(rf).content
    }

    pub fn get_line(&self, loc: Loc) -> Option<String> {
        if !loc.span.exists() {
            return None;
        }

        let mut line_number = 1;
        let mut start_of_line = 0;
        let mut end_of_line = None;
        let file = &self.files[loc.file.idx];
        let span = loc.span;
        let src = &file.content;

        for (i, ch) in src.char_indices() {
            if ch == '\n' {
                if i < span.start as usize {
                    line_number += 1;
                    start_of_line = i + 1;
                }
                if i >= span.end as usize && end_of_line.is_none() {
                    end_of_line = Some(i);
                }
            }
        }
        let end_of_line = end_of_line.unwrap_or(src.len());

        // If this comes from the REPL, then we should add the REPL
        // offset to the line number.
        if let FileSource::Repl(offset) = file.source {
            line_number += offset;
        }

        let mut result = format!("{:3} |", line_number);
        result.push_str(&src[start_of_line..end_of_line]);
        result.push_str("\n     ");
        for _ in start_of_line..(span.start as usize) {
            result.push(' ');
        }
        for _ in span.start..span.end {
            result.push('^');
        }
        Some(result)
    }
}

impl Default for FileTable {
    fn default() -> Self {
        FileTable::new()
    }
}
