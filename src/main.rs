#![warn(clippy::pedantic, clippy::nursery, clippy::nursery)]
#![allow(clippy::non_ascii_literal, clippy::unicode_not_nfc)]

#[macro_use]
extern crate lazy_static;

extern crate regex;

mod convert;
mod scansion;

use log::info;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn generate_li(poem: &Poem, date: &str) -> (usize, String) {
    let how_many_lines = poem.line_count();
    let li = if how_many_lines == 60 {
        format!("    <li><a href=\"{}.html\">{}</a></li>", date, date)
    } else {
        format!(
            "    <li><a href=\"{}.html\">{}</a> (only the first {} lines are attested)</li>",
            date, date, how_many_lines
        )
    };
    (how_many_lines, li)
}

fn read_to_poem_map() -> Result<std::collections::HashMap<String, Poem>, Box<dyn Error>> {
    use std::collections::HashMap;
    let mut poem_map = HashMap::new();
    for entry in std::fs::read_dir("raw/")? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            let date = entry.file_name().into_string().unwrap();
            info!("Converting {}", date);
            let content = std::fs::read_to_string(format!("raw/{}", date))?;

            // to convert \r\n into \n
            let content = content.lines().collect::<Vec<_>>().join("\n");
            let cont = content.split("\n\n").collect::<Vec<_>>();

            let poem = Poem::new(&cont);

            poem_map.insert(date, poem);
        }
    }
    Ok(poem_map)
}

fn main() -> Result<(), Box<dyn Error>> {
    use itertools::Itertools;
    use std::collections::HashMap;

    // Starting up stuffs
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Read
    let poem_map = read_to_poem_map()?;

    // Write each poem's IPA and scansion
    for (date, poem) in &poem_map {
        chapterize_and_write_file(
            poem,
            &mut File::create(format!("docs/{}.html", date))?,
            date,
            |line| convert::elide_initial_glottal_stop(&line.to_ipa()),
        )?;
        chapterize_and_write_file(
            poem,
            &mut File::create(format!("docs/{}-scansion.html", date))?,
            date,
            scansion::to_scanned,
        )?;
    }

    let mut map = HashMap::new();
    for (date, poem) in poem_map {
        map.insert(date.clone(), generate_li(&poem, &date));
    }

    let sorted = map.into_iter().sorted().collect::<Vec<_>>();
    let html = sorted
        .iter()
        .map(|(_date, (_, li))| li.to_owned())
        .collect::<Vec<_>>()
        .join("\n");

    info!("Writing index.html");
    let mut file = File::create("docs/index.html")?;
    write!(file, "<!DOCTYPE html><head><title>Hexecontastich</title></head><body><h2>Hexecontastich</h2><img src=\"img/hexecontastich.jpg\" width=\"300\">\n<ul>\n{}\n</ul>\n</body>", html)?;

    let mut total_lines = 0;
    let progress = sorted
        .iter()
        .map(|(date, (num_of_lines, _li))| {
            let date = date.split('-').collect::<Vec<_>>()[0..=2].join("/");
            total_lines += num_of_lines;
            format!("{}\t{}", date, total_lines)
        })
        .collect::<Vec<_>>()
        .join("\n");

    info!("Writing progress.tsv");
    let mut file = File::create("progress.tsv")?;
    writeln!(file, "{}", progress)?;
    info!("Processed the total of {} lines.", total_lines);
    Ok(())
}

struct Poem(Vec<Vec<Line>>);
use line::Line;
pub mod line;

impl Poem {
    pub fn new(content: &[&str]) -> Self {
        Self(
            content
                .iter()
                .map(|a| a.lines().map(Line::new).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
    }

    pub fn line_count(&self) -> usize {
        let Self(poem) = &self;
        poem.iter().map(|chapter| chapter.iter().count()).sum()
    }
}

fn chapterize_and_write_file<F>(
    poem: &Poem,
    file: &mut File,
    date: &str,
    mut lambda: F,
) -> Result<(), Box<dyn Error>>
where
    F: FnMut(&Line) -> String,
{
    let mut line_index = 0;
    let Poem(poem) = &poem;
    let content = poem
        .iter()
        .enumerate()
        .map(|(chap_num, chap)| {
            format!(
                "{}. <div id=\"res{}\" class=\"poem_block\">\n{}</div><br>\n",
                chap_num + 1,
                chap_num + 1,
                chap.iter()
                    .filter_map(|line| {
                        if line.is_empty() {
                            None
                        } else {
                            let a = lambda(line);
                            line_index += 1;
                            if line_index % 5 == 0 {
                                Some(format!(
                                    "<p>{}<span class=\"line_number\">{}</span></p>",
                                    a, line_index
                                ))
                            } else {
                                Some(format!("<p>{}</p>", a))
                            }
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        })
        .collect::<Vec<_>>();

    write!(
        file,
        r#"<!DOCTYPE html>
<meta charset="utf-8">
<link href="poem.css" rel="stylesheet">
<a href="index.html">back to top</a> <a href="{}.html">main text</a> <a href="{}-scansion.html">scansion</a><br><br>
{}
"#,
        date,
        date,
        content.join("\n"),
    )?;

    Ok(())
}
