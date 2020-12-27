#[macro_use]
extern crate lazy_static;

extern crate regex;

mod convert;
mod scansion;

use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    write_files(
        "2020-09-26",
        &vec![
            "
'I:gakinA:sebi'AntegonI:si:
se:sUperi:'a:gAntasorI:'a
'a:kasinE:raga'A:ri:ni:sEko
ge:te:rImba'asAkabe:nI:
dEhtonagi:rAse'a:ka:sUhte
sa:magarIntisisAkare:nI:
sIssatinA:gabigEntegonI:si:
se:ramirA:gambOhteganI:
si:si:'Anteraka:sarinIsse
'a:kagipIssekAre:rimanI:
dAnkatina:sAhtantibinEsse
'a:ka:kAssiritAhteganI:
bi:rine'a:rAkiga:na:'Ihti
'IhtoresAntanisUhtabi'Onda",
            "
mi:si:'a:tInatankeronAhte
kantetinIssi'akAsene:mI:
nihtina'a:gAse'a:rina'Ihte
kAssaritIndasakAsere:mI:
bi:sakinAnte'a'a:gasirIhte
'intasenAhti'Ana:gisemI:
ni:rise'Andiki'A:samarIhti
'i:ri:bAsena:'AssinanI:
dihtire'Ahpisira:ka:se:nAsi
ni:bise'Ahtana'Irome:nI:",
            "
ne:sIgi'assAre'a:ka:nAnte
ti:rekirI:rekanAsene:mI:
kuhtina'IndesirA:nisomInde
nIsi'amAse:ratinIsige:mI:
mi:miserAntani'Asse'orI:re
dihtesinImbanibAsene:mI:
nindisomAhterana:gaserIhti
ni:ni:sAndire'AssabagOhta",
            "
'a:sUpana:'a:nIhtinasEndi
se:se:pOturi:'AssagarI:ra
ne:be:'a:tInasi:rakurAhti
'inkOpeni:ni:sAntegorI:na
'o:nOhte:'a:gAntari'Asse
nOhtari'I:nakorIsare:sI:
ti:gInane:ri:'AhtinarIhti
mEhtako'i:rIse'Agone:sI:
'o:nOhte:mi:'AnteginIssa
de:re:sAturi:nAsome:nI:
ni:'Ameka:ta:gAssatinAhta
si:si:'AhteribIndabinAhta",
            "
sa:'a:re:gAsi'a:gasenInti
me:ne:tOgusiri'Asene:mI:
ti:nIsema:na:bEntasinAhte
na:rInase:ni:nAsene:mI:
si:sitinAhtiri'AndasinAhte
ni:ni:sa:tIrinIsige:mI:
kOhtorisImbasebEhtina'Indi
be:mi:rIhtasinAhtegomI:
'angasinIntemigIhteribIhti
so:sUri'a:ga:sAtine:mI:
se:be:'AssirinIndasinAnde
'Isima:'a:bIsesApire:mI:
mEssirinIntami'AnderemIssi
ma:nApari:ni:rUhteribImba",
            "
nampasinIhtera'a:nisirIngi
'a:ka:nIsige:'AhtabamI:",
        ],
    )
}

fn write_files(date: &str, data: &[&str]) -> Result<(), Box<dyn Error>> {
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
fn write_file(file: &mut File, converted: &[String], date: &str) -> Result<(), Box<dyn Error>> {
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
    Ok(())
}
