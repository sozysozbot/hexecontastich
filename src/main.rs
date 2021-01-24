#![warn(clippy::pedantic, clippy::nursery, clippy::nursery)]
#![allow(clippy::non_ascii_literal, clippy::unicode_not_nfc)]

#[macro_use]
extern crate lazy_static;

extern crate regex;

mod convert;
mod html;
mod scansion;

use log::info;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn read_to_poem_map() -> Result<HashMap<String, Poem>, Box<dyn Error>> {
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

    //----------------------------------------------------------------
    // Starting up stuffs
    //----------------------------------------------------------------
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    //----------------------------------------------------------------
    // Read
    //----------------------------------------------------------------
    let poem_map = read_to_poem_map()?;

    //----------------------------------------------------------------
    // Write each poem's IPA and scansion
    //----------------------------------------------------------------
    for (date, poem) in &poem_map {
        html::chapterize_and_write_file(
            poem,
            &mut File::create(format!("docs/{}.html", date))?,
            date,
            |line| convert::elide_initial_glottal_stop(&line.to_ipa()),
        )?;
        html::chapterize_and_write_file(
            poem,
            &mut File::create(format!("docs/{}-scansion.html", date))?,
            date,
            scansion::to_scanned,
        )?;
    }

    //----------------------------------------------------------------
    // Write index.html
    //----------------------------------------------------------------
    html::write_index(&poem_map)?;

    //----------------------------------------------------------------
    // Write progress.tsv
    //----------------------------------------------------------------
    let mut total_lines = 0;
    let progress = poem_map
        .iter()
        .sorted()
        .map(|(date, poem)| {
            let date = date.split('-').collect::<Vec<_>>()[0..=2].join("/");
            total_lines += poem.line_count();
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

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Poem(Vec<Vec<Line>>);
use line::Line;
pub mod line;

impl Poem {
    #[must_use]
    pub fn new(content: &[&str]) -> Self {
        Self(
            content
                .iter()
                .map(|a| a.lines().map(Line::new).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
    }

    #[must_use]
    pub fn line_count(&self) -> usize {
        let Self(poem) = &self;
        poem.iter().map(|chapter| chapter.iter().count()).sum()
    }
}
