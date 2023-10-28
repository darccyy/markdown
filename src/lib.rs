use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Markdown {
    pub meta: Meta,
    pub parts: Vec<Part>,
}

type Meta = HashMap<String, String>;

#[derive(Clone, Debug)]
pub enum Part {
    Section(Section),
    Content(Vec<Content>),
}

#[derive(Clone, Debug)]
pub struct Section {
    pub level: usize,
    pub header: String,
    pub content: Vec<Section>,
}

#[derive(Clone, Debug)]
pub enum Content {
    Paragraph(Lines),
    Unordered(Vec<NestedList>),
    Ordered(Vec<NestedList>),
    Quote(Lines),
    Horizontal,
}

#[derive(Clone, Debug)]
pub enum NestedList {
    More(Vec<NestedList>),
    Line(Line),
}

type Lines = Vec<Line>;
type Line = String;

pub fn parse(file: &str) -> Result<Markdown, ()> {
    let (meta, file) = split_meta(file);
    println!("{:#?}", meta);

    let lines = lex(&file).unwrap();
    println!("{:#?}", lines);
    todo!()
}

fn split_meta(file: &str) -> (Meta, String) {
    let mut meta: Option<Vec<&str>> = None;
    let mut rest: Option<Vec<&str>> = None;

    let mut lines = file.lines().peekable();
    while let Some(line) = lines.peek() {
        if line.trim().is_empty() {
            lines.next();
            continue;
        }
        if line == &"---" {
            match &meta {
                Some(_) => {
                    lines.next();
                    rest = Some(lines.collect());
                    break;
                }
                None => meta = Some(Vec::new()),
            }
        } else {
            match &mut meta {
                Some(meta) => meta.push(line),
                None => break,
            }
        }
        lines.next();
    }

    let file = match rest {
        Some(rest) => rest.join("\n"),
        None => file.to_string(),
    };

    let meta = meta.unwrap_or_default();
    let mut meta_map = Meta::new();
    for line in meta {
        let (key, value) = match line.find(':') {
            Some(index) => {
                let (key, value) = line.split_at(index);
                let mut value = value.chars();
                value.next();
                (key.trim(), value.as_str().trim())
            }
            None => (line, ""),
        };
        meta_map.insert(key.to_string(), value.to_string());
    }

    (meta_map, file)
}

#[derive(Debug)]
enum LineKind {
    Empty,
    Header(usize, String),
    Plain(String),
    Unordered(usize, String),
    Ordered(usize, String),
    Quote(String),
    Horizontal,
}

fn lex(file: &str) -> Result<Vec<LineKind>, ()> {
    let mut md_lines = Vec::new();

    for line in file.lines() {
        let mut words = line.trim().split_whitespace();
        let Some(first_word) = words.next() else {
            md_lines.push(LineKind::Empty);
            continue;
        };

        if first_word == "---" {
            md_lines.push(LineKind::Horizontal);
            continue;
        }

        if first_word.chars().all(|ch| ch == '#') {
            let level = first_word.len();
            let rest = words.collect::<Vec<_>>().join(" ");
            md_lines.push(LineKind::Header(level, rest));
            continue;
        }

        if "-+*".contains(first_word) {
            let level = get_list_level(line);
            let rest = words.collect::<Vec<_>>().join(" ");
            md_lines.push(LineKind::Unordered(level, rest));
            continue;
        }

        if is_ordered_ident(first_word) {
            let level = get_list_level(line);
            let rest = words.collect::<Vec<_>>().join(" ");
            md_lines.push(LineKind::Ordered(level, rest));
            continue;
        }

        if first_word == ">" {
            let rest = words.collect::<Vec<_>>().join(" ");
            md_lines.push(LineKind::Quote(rest));
            continue;
        }

        md_lines.push(LineKind::Plain(line.trim().to_string()));
    }

    Ok(md_lines)
}

fn is_ordered_ident(word: &str) -> bool {
    let mut chars = word.chars();
    if !chars.next_back().is_some_and(|ch| ch == '.') {
        return false;
    }
    chars.as_str().parse::<usize>().is_ok()
}

fn get_list_level(line: &str) -> usize {
    const INDENT_SIZE: usize = 4;
    line.chars().position(|ch| ch != ' ').unwrap_or(0) / INDENT_SIZE
}
