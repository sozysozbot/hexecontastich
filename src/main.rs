#[macro_use]
extern crate lazy_static;

extern crate regex;

mod convert;
mod scansion;

use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::create("docs/2020-09-26.html")?;

    let data = vec![
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
    ];

    let converted = data.into_iter().map(convert::convert).collect::<Vec<_>>();

    let text = r#"
<!DOCTYPE html>
<meta charset="utf-8">
<link href="poem.css" rel="stylesheet">
<a href="index.html">back to top</a> <a href="2020-09-26.html">main text</a> <a href="2020-09-26-scansion.html">scansion</a><br><br>
1. <div id="res1" class="poem_block"></div><br>
2. <div id="res2" class="poem_block"></div><br>
3. <div id="res3" class="poem_block"></div><br>
4. <div id="res4" class="poem_block"></div><br>
5. <div id="res5" class="poem_block"></div><br>
6. <div id="res6" class="poem_block"></div><br>

<script src="main.js"></script>
<script>
    const texts = [
        `
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
'IhtoresAntanisUhtabi'Onda
`, `
mi:si:'a:tInatankeronAhte
kantetinIssi'akAsene:mI:
nihtina'a:gAse'a:rina'Ihte
kAssaritIndasakAsere:mI:
bi:sakinAnte'a'a:gasirIhte
'intasenAhti'Ana:gisemI:
ni:rise'Andiki'A:samarIhti
'i:ri:bAsena:'AssinanI:
dihtire'Ahpisira:ka:se:nAsi
ni:bise'Ahtana'Irome:nI:
`, `
ne:sIgi'assAre'a:ka:nAnte
ti:rekirI:rekanAsene:mI:
kuhtina'IndesirA:nisomInde
nIsi'amAse:ratinIsige:mI:
mi:miserAntani'Asse'orI:re
dihtesinImbanibAsene:mI:
nindisomAhterana:gaserIhti
ni:ni:sAndire'AssabagOhta
`, `
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
si:si:'AhteribIndabinAhta
`, `
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
ma:nApari:ni:rUhteribImba
`, `
nampasinIhtera'a:nisirIngi
'a:ka:nIsige:'AhtabamI:
`
    ];"#;
    let latter = r#"
    function print_poem(f, texts) {
        let line_index = 0;
        for (let i = 0; i < texts.length; i++) {
            document.getElementById(`res${i + 1}`).innerHTML =
                f(texts[i])
                    .split("\n")
                    .filter(a => a !== "")
                    .map(a => {
                        line_index++;
                        if (line_index % 5 === 0) {
                            return `<p>${a}<span class="line_number">${line_index}</span></p>`
                        } else {
                            return `<p>${a}</p>`
                        }
                    })
                    .join("\n");
        }
    }
        print_poem(convert, texts);
</script>    
    "#;
    write!(file, "{}{}", text, latter)?;
    Ok(())
}
