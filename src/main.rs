#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::non_ascii_literal, clippy::unicode_not_nfc)]

#[macro_use]
extern crate lazy_static;

extern crate regex;

mod convert;
mod scansion;

use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    use itertools::Itertools;
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut total_lines = 0;
    for entry in std::fs::read_dir("raw/")? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            let date = entry.file_name().into_string().unwrap();

            println!("Converting {}", date);
            let content = std::fs::read_to_string(format!("raw/{}", date))?;

            // to convert \r\n into \n
            let content = content.lines().collect::<Vec<_>>().join("\n");

            let cont = content.split("\n\n").collect::<Vec<_>>();

            let how_many_lines = write_files(&date, &cont)?;

            let li = if how_many_lines == 60 {
                format!("    <li><a href=\"{}.html\">{}</a></li>", date, date)
            } else {
                format!("    <li><a href=\"{}.html\">{}</a> (only the first {} lines are attested)</li>", date, date, how_many_lines)
            };

            total_lines += how_many_lines;

            map.insert(date, (total_lines, li));
        }
    }

    println!("Processed the total of {} lines.", total_lines);

    let sorted = map.into_iter().sorted().collect::<Vec<_>>();
    let html = sorted
        .iter()
        .map(|(_date, (_tot_lines, li))| li.to_owned())
        .collect::<Vec<_>>()
        .join("\n");

    println!("Writing index.html");
    let mut file = File::create("docs/index.html")?;
    write!(file, "<!DOCTYPE html><head><title>Hexecontastich</title></head><body><h2>Hexecontastich</h2>\n<ul>\n{}\n</ul>\n</body>", html)?;

    let progress = sorted
        .iter()
        .map(|(date, (tot_lines, _li))| {
            let date = date.split('-').collect::<Vec<_>>()[0..=2].join("/");
            format!("{}\t{}", date, tot_lines)
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("Writing progress.tsv");
    let mut file = File::create("progress.tsv")?;
    writeln!(file, "{}", progress)?;

    Ok(())
}

// returns how many lines there are
fn write_files(date: &str, content: &[&str]) -> Result<i32, Box<dyn Error>> {
    {
        let mut file = File::create(format!("docs/{}.html", date))?;
        let converted = content
            .iter()
            .map(|a| convert::convert(a))
            .collect::<Vec<_>>();

        write_file(&mut file, &converted, date)?;
    }
    {
        let mut file = File::create(format!("docs/{}-scansion.html", date))?;
        let scansion = content
            .iter()
            .map(|a| scansion::scansion(a))
            .collect::<Vec<_>>();

        write_file(&mut file, &scansion, date)
    }
}

// returns how many lines there are
fn write_file(file: &mut File, converted: &[String], date: &str) -> Result<i32, Box<dyn Error>> {
    let mut res = vec![];

    let mut line_index = 0;
    for u in converted.iter() {
        res.push(
            u.lines()
                .filter_map(|a| {
                    if a.is_empty() {
                        None
                    } else {
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
                .join("\n"),
        );
    }

    let mut content = String::new();

    for (i, r) in res.iter().enumerate() {
        content += &format!(
            "{}. <div id=\"res{}\" class=\"poem_block\">\n{}</div><br>\n\n",
            i + 1,
            i + 1,
            r
        )
    }

    write!(
        file,
        r#"<!DOCTYPE html>
<meta charset="utf-8">
<link href="poem.css" rel="stylesheet">
<a href="index.html">back to top</a> <a href="{}.html">main text</a> <a href="{}-scansion.html">scansion</a><br><br>
{}"#,
        date, date, content,
    )?;

    Ok(line_index)
}
