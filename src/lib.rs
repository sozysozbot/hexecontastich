#[macro_use]
extern crate lazy_static;

extern crate regex;

pub mod convert;
pub mod html;
pub mod scansion;

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Poem(Vec<Vec<Line>>);
use line::Line;
pub mod line;

impl Poem {
    #[must_use]
    pub fn count_syll<F>(&self, f: &F) -> usize
    where
        F: Fn(&line::syllabify::Syllable) -> bool,
    {
        self.0
            .iter()
            .map(|vec_line| {
                vec_line
                    .iter()
                    .map(|line| line.count_syll(&f))
                    .sum::<usize>()
            })
            .sum()
    }
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

pub fn count_syll<F>(poem_map: &HashMap<String, Poem>, f: &F) -> usize
where
    F: Fn(&line::syllabify::Syllable) -> bool,
{
    poem_map.iter().map(|(_, poem)| poem.count_syll(f)).sum()
}
