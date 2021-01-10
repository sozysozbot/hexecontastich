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

            map.insert(date, li);

            total_lines += how_many_lines;
        }
    }

    println!("Processed the total of {} lines.", total_lines);

    let sorted = map
        .into_iter()
        .sorted()
        .map(|(_date, li)| li)
        .collect::<Vec<_>>()
        .join("\n");

    println!("Writing index.html");
    let mut file = File::create(format!("docs/index.html"))?;
    write!(file, "<!DOCTYPE html>\n<ul>\n{}\n</ul>\n", sorted)?;

    Ok(())
}

// returns how many lines there are
fn write_files(date: &str, data: &[&str]) -> Result<i32, Box<dyn Error>> {
    {
        let mut file = File::create(format!("docs/{}.html", date))?;
        let converted = data
            .into_iter()
            .map(|a| convert::convert(a.clone()))
            .collect::<Vec<_>>();

        write_file(&mut file, &converted, date)?;
    }
    {
        let mut file = File::create(format!("docs/{}-scansion.html", date))?;
        let scansion = data
            .into_iter()
            .map(|a| scansion::scansion(a.clone()))
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
                .filter(|l| *l != "")
                .map(|a| {
                    line_index += 1;
                    if line_index % 5 == 0 {
                        return format!(
                            "<p>{}<span class=\"line_number\">{}</span></p>",
                            a, line_index
                        );
                    } else {
                        return format!("<p>{}</p>", a);
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
