use super::line::syllabify::{Coda, Onset, Syllable, Vowel};
use super::scansion::WeightAndAccent;
use crate::w;
use log::warn;
use regex::Regex;

mod tests {
    #[allow(dead_code)]
    fn convert_line2(text: &str) -> String {
        use super::*;
        match to_ipa(&Line::new(text), false) {
            Ok(a) => a,
            Err(e) => panic!("{}, in line `{}`", e, text),
        }
    }

    #[allow(dead_code)]
    fn convert(text: &str) -> String {
        use super::*;
        text.lines()
            .map(|line| elide_initial_glottal_stop(&convert_line2(line)))
            .collect::<Vec<_>>()
            .join("\n")
    }
    #[test]
    fn it_works3() {
        assert_eq!(
            convert(
                "
gampAnume:'a:rEssakomInna
'impironEhkarisa:duronEhta
'i:rIsege:'a:nAssarobIndi
'i:sAnga:bandIsere:bI:
mo:bara'a:pIresa:nogorEsse
b*Ani:risanIndemOra:gasenI:
'o:mInne:'o:rInterobEssa
mo:mo:sUpani:sAnteganI:
ka:nOhte:'a:nAssagomInte
bi:nIsenamparobAkare:nI:
se:re:'a:tInasa:rikomEndi
ninsAme'antIra'a:makonI:
no:mo:'e:sIrima:ratanI:'a
me:sIroga:'a:mAsage:nI:
sIndiragAssagoma:rakanE:'a
sAta:masorIndiripA:sobarI:ba"
            ),
            "
ɡəmˈpɑnume̞ːʔɑːˈɾe̞ssəko̞ˈminnə
ʔimpiɾo̞ˈne̞hkəɾisɑːduɾo̞ˈne̞htə
ʔiːˈɾise̞ɣe̞ːʔɑːˈnɑssəɾo̞ˈβindi
ʔiːˈsɑŋɡɑːβənˈdise̞ɾe̞ːˈβiː
mo̞ːβəɾəʔɑːˈpiɾe̞sɑːno̞ɣo̞ˈɾe̞sse̞
ˈβɑniːɾisəˈninde̞ˈmo̞ɾɑːɣəse̞ˈniː
ʔo̞ːˈminne̞ːʔo̞ːˈɾinte̞ɾo̞ˈβe̞ssə
mo̞ːmo̞ːˈsupəniːˈsɑnte̞ɣəˈniː
kɑːˈno̞hte̞ːʔɑːˈnɑssəɣo̞ˈminte̞
biːˈnise̞nɑmpəɾo̞ˈβɑkəɾe̞ːˈniː
se̞ːɾe̞ːʔɑːˈtinəsɑːɾiko̞ˈme̞ndi
ninˈsɑme̞ʔənˈtiɾəʔɑːməko̞ˈniː
no̞ːmo̞ːʔe̞ːˈsiɾimɑːɾətəˈniːʔə
me̞ːˈsiɾo̞ɣɑːʔɑːˈmɑsəɣe̞ːˈniː
ˈsindiɾəˈɣɑssəɣo̞mɑːɾəkəˈne̞ːʔə
ˈsɑtɑːməso̞ˈɾindiɾiˈpɑːso̞βəˈɾiːβə"
        )
    }

    #[test]
    fn it_works2() {
        assert_eq!(
            convert(
                "
nA:ga'inIssibana:gasorInte
ba:sAmori:ga:'AntigonI:
de:re:si:sInana:gasinInti
'antAmani:'andAhtigorI:
mo:mo:si:rEkono:resinIndi
di:nisi'Indarina:gasenI:
'o:sAtiro:'o:rEndimanEndi
ni:pIrame:ka:rAntegonI:
'i:sInare:'i:'Intikabe:'a
sa:sa:bAserisina:masorInda"
            ),
            "
ˈnɑːɣəʔiˈnissiβənɑːɣəso̞ˈɾinte̞
bɑːˈsɑmo̞ɾiːɣɑːˈʔɑntiɣo̞ˈniː
de̞ːɾe̞ːsiːˈsinənɑːɣəsiˈninti
ʔənˈtɑməniːʔənˈdɑhtiɣo̞ˈɾiː
mo̞ːmo̞ːsiːˈɾe̞ko̞no̞ːɾe̞siˈnindi
diːnisiˈʔindəɾinɑːɣəse̞ˈniː
ʔo̞ːˈsɑtiɾo̞ːʔo̞ːˈɾe̞ndiməˈne̞ndi
niːˈpiɾəme̞ːkɑːˈɾɑnte̞ɣo̞ˈniː
ʔiːˈsinəɾe̞ːʔiːˈʔintikəβe̞ːʔə
sɑːsɑːˈβɑse̞ɾisinɑːməso̞ˈɾində"
        )
    }

    #[test]
    fn it_works() {
        assert_eq!(
            convert(
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
'IhtoresAntanisUhtabi'Onda
"
            ),
            "
ˈʔiːɣəkiˈnɑːse̞βiˈʔɑnte̞ɣo̞ˈniːsiː
se̞ːˈsupe̞ɾiːʔɑːˈɣɑntəso̞ˈɾiːʔə
ɑːkəsiˈne̞ːɾəɣəˈʔɑːɾiːniːˈse̞ko̞
ɡe̞ːte̞ːˈɾimbəʔəˈsɑkəβe̞ːˈniː
ˈde̞hto̞nəɣiːˈɾɑse̞ʔɑːkɑːˈsuhte̞
sɑːməɣəˈɾintisiˈsɑkəɾe̞ːˈniː
ˈsissətiˈnɑːɣəβiˈɣe̞nte̞ɣo̞ˈniːsiː
se̞ːɾəmiˈɾɑːɣəmˈbo̞hte̞ɣəˈniː
siːsiːˈʔɑnte̞ɾəkɑːsəɾiˈnisse̞
ɑːkəɣiˈpisse̞ˈkɑɾe̞ːɾiməˈniː
ˈdɑŋkətinɑːˈsɑhtɑntiβiˈne̞sse̞
ɑːkɑːˈkɑssiɾiˈtɑhte̞ɣəˈniː
biːɾine̞ʔɑːˈɾɑkiɣɑːnɑːˈʔihti
ˈʔihto̞ɾe̞ˈsɑntəniˈsuhtəβiˈʔo̞ndə"
        )
    }
}

use super::Line;

pub fn to_ipa(line: &Line, warn: bool) -> Result<String, &'static str> {
    let sylls = line.as_vec();
    let scansions: Vec<WeightAndAccent> = sylls.iter().map(|a| (*a).into()).collect();
    let mut ans = String::new();
    for (i, syll) in sylls.clone().into_iter().enumerate() {
        let accent = if syll.accented { "ˈ" } else { "" };
        let is_plosive = (i > 0 && matches!(sylls[i - 1].coda, Some(Coda::Nasal)))
            || (i == 0
                && !matches!(
                    &scansions[..],
                    [w!('U'), w!('m'), w!('u'), w!('u'), ..] | [w!('U'), w!('m'), w!('m'), ..]
                ));
        let onset = match (is_plosive, syll.onset) {
            (_, Onset::T) => "t",
            (_, Onset::K) => "k",
            (_, Onset::S) => "s",
            (_, Onset::N) => "n",
            (_, Onset::M) => "m",
            (_, Onset::Q) => "ʔ",
            (_, Onset::P) => "p",
            (true, Onset::B) => "b",
            (false, Onset::B) => "β",
            (true, Onset::G) => "ɡ",
            (false, Onset::G) => "ɣ",
            (true, Onset::R) => "d",
            (false, Onset::R) => {
                // `/ɾ/` + unaccented short vowel + `/ɾ/` turns the first `/ɾ/` into `[d]`
                if WeightAndAccent::from(syll) == w!('u')
                    && matches!(
                        sylls.get(i + 1),
                        Some(Syllable {
                            onset: Onset::R, ..
                        })
                    )
                {
                    "d"
                } else {
                    "ɾ"
                }
            }
        };
        let coda = match syll.coda {
            None => "",
            Some(Coda::Long) => "ː",
            Some(Coda::H) => match sylls[i + 1].onset {
                Onset::K | Onset::T | Onset::P => "h",
                Onset::S => "s",
                Onset::G | Onset::B | Onset::N | Onset::M | Onset::Q => return Err(
                    "Aspirations should not be followed by a glottal stop or a voiced consonant",
                ),
                Onset::R => {
                    // `/ɾ/` + unaccented short vowel + `/ɾ/` turns the first `/ɾ/` into `[d]`
                    if !sylls[i + 1].accented
                        && sylls[i + 1].coda.is_none()
                        && matches!(
                            sylls.get(i + 2),
                            Some(Syllable {
                                onset: Onset::R, ..
                            })
                        )
                    {
                        if warn {
                            warn!("Rare instance of h+d");
                        }
                        "h"
                    } else {
                        return Err("Aspirations should not be followed by a glottal stop or a voiced consonant");
                    }
                }
            },
            Some(Coda::Nasal) => match sylls[i + 1].onset {
                Onset::K | Onset::G => "ŋ",
                Onset::T | Onset::R | Onset::N => "n",
                Onset::S => {
                    if warn {
                        warn!("Rare instance of n+s");
                    }
                    "n"
                }
                Onset::P | Onset::B | Onset::M => "m",
                Onset::Q => return Err("Nasals should not be followed by a glottal stop"),
            },
        };
        let vowel = match syll.vowel {
            Vowel::E => "e̞",
            Vowel::O => "o̞",
            Vowel::I => "i",
            Vowel::U => "u",
            Vowel::A => match (syll.coda, syll.accented) {
                (Some(Coda::Long), _) | (_, true) => "ɑ",
                (None, false) | (Some(Coda::H), false) => "ə",
                (Some(Coda::Nasal), false) => {
                    // if closed syllable, ə becomes ɑ, except when the next syllable is accented
                    if matches!(sylls.get(i + 1), Some(Syllable { accented: true, .. })) {
                        "ə"
                    } else {
                        "ɑ"
                    }
                }
            },
        };
        ans += &format!("{}{}{}{}", accent, onset, vowel, coda)
    }
    Ok(ans)
}

pub fn elide_initial_glottal_stop(ans: &str) -> String {
    lazy_static! {
        static ref RG1: Regex = Regex::new(r"^ʔɑ([mnŋ])([^ˈ])").unwrap();
        static ref RG2: Regex = Regex::new(r"^ʔɑː([^ˈ])").unwrap();
    }
    let stage1 = RG1.replace_all(ans, "ɑ$1$2");
    RG2.replace_all(&stage1, "ɑː$1").to_string()
}
