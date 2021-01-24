use crate::{Line, Poem};
use log::info;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn generate_li(poem: &Poem, date: &str) -> String {
    let how_many_lines = poem.line_count();
    let li = if how_many_lines == 60 {
        format!("    <li><a href=\"{}.html\">{}</a></li>", date, date)
    } else {
        format!(
            "    <li><a href=\"{}.html\">{}</a> (only the first {} lines are attested)</li>",
            date, date, how_many_lines
        )
    };
    li
}

pub fn write_index(poem_map: &HashMap<String, Poem>) -> Result<(), Box<dyn Error>> {
    use itertools::Itertools;
    let html = poem_map
        .iter()
        .sorted()
        .map(|(date, poem)| generate_li(poem, date))
        .collect::<Vec<_>>()
        .join("\n");

    info!("Writing index.html");
    let mut file = File::create("docs/index.html")?;
    write!(file, "<!DOCTYPE html><head><title>Hexecontastich</title></head><body><h2>Hexecontastich</h2><img src=\"img/hexecontastich.jpg\" width=\"300\">\n<ul>\n{}\n</ul>\n</body>", html)?;
    Ok(())
}

pub fn chapterize_and_write_file<F>(
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
