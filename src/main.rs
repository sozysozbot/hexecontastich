#![warn(clippy::pedantic, clippy::nursery, clippy::nursery)]
#![allow(clippy::non_ascii_literal, clippy::unicode_not_nfc)]
use hexecontastich::{convert, count_syll, html, line, scansion, Poem};
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

    //----------------------------------------------------------------
    // Write stat.html
    //----------------------------------------------------------------
    write_stat(&poem_map)
}

#[allow(clippy::cast_precision_loss)]
fn write_stat(poem_map: &HashMap<String, Poem>) -> Result<(), Box<dyn Error>> {
    use line::syllabify::Coda::{Long, Nasal, H};
    use line::syllabify::Onset::{B, G, K, M, N, P, Q, R, S, T};
    use line::syllabify::Vowel::{A, E, I, O, U};
    let mut file = File::create("docs/stat.html")?;
    let syll_total = count_syll(poem_map, &|_| true);

    let onset_stat = vec![P, B, M, T, R, N, S, K, G, Q]
        .into_iter()
        .map(|onset| {
            let count = count_syll(poem_map, &|syll| syll.onset == onset);
            stat_table_row(
                &format!("/{}/", onset.to_representative_ipa()),
                count,
                count as f64 / (syll_total as f64) * 100.0,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let main_vowel_stat = vec![I, E, A, O, U]
        .into_iter()
        .map(|vowel| {
            let count = count_syll(poem_map, &|syll| syll.vowel == vowel);
            stat_table_row(
                &format!("/{}/", vowel.to_representative_ipa()),
                count,
                count as f64 / (syll_total as f64) * 100.0,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let rhyme_stat = vec![
        (I, None),
        (I, Some(Nasal)),
        (I, Some(H)),
        (I, Some(Long)),
        (E, None),
        (E, Some(Nasal)),
        (E, Some(H)),
        (E, Some(Long)),
        (A, None),
        (A, Some(Nasal)),
        (A, Some(H)),
        (A, Some(Long)),
        (O, None),
        (O, Some(Nasal)),
        (O, Some(H)),
        (O, Some(Long)),
        (U, None),
        (U, Some(Nasal)),
        (U, Some(H)),
        (U, Some(Long)),
    ]
    .into_iter()
    .map(|(vowel, coda)| {
        let count = count_syll(poem_map, &|syll| syll.vowel == vowel && syll.coda == coda);
        stat_table_row(
            &format!(
                "/{}{}/",
                vowel.to_representative_ipa(),
                match coda {
                    Some(coda) => coda.to_representative_ipa(),
                    None => "",
                }
            ),
            count,
            count as f64 / (syll_total as f64) * 100.0,
        )
    })
    .collect::<Vec<_>>()
    .join("\n");
    writeln!(
        file,
        "<!DOCTYPE html>
<head><title>statistics</title></head>\n<body style=\"font-family: Arial;\">
<h2>onsets</h2>\n<table>\n{}</table>\n
<h2>main vowel</h2>\n<table>\n{}</table>\n
<h2>rhyme</h2>\n<table>\n{}</table>\n",
        onset_stat, main_vowel_stat, rhyme_stat
    )?;
    Ok(())
}

fn stat_table_row(content: &str, count: usize, ratio: f64) -> String {
    format!(
        "<tr><td>{}</td><td style=\"text-align: right; font-family: monospace\">{}</td><td style=\"text-align: right; font-family: monospace\">{:.2}%</td></tr>",
        content, count, ratio
    )
}
