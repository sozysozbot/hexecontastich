use regex::Regex;
mod tests {
    #[test]
    fn it_works3() {
        use super::*;
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
        use super::*;
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
        use super::*;
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

pub fn second_from_last_mut<T>(a: &mut [T]) -> Option<&mut T> {
    if let [.., snd_from_last, _] = a {
        Some(snd_from_last)
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Syllable(pub Onset, pub Vowel, pub Option<Coda>, pub bool);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Coda {
    Nasal,
    H,
    Long,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vowel {
    A,
    E,
    I,
    O,
    U,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Onset {
    P,
    B,
    M,
    T,
    R,
    N,
    S,
    K,
    G,
    Q,
}

fn consonant(c: char) -> Option<Onset> {
    match c {
        '\'' | 'q' => Some(Onset::Q),
        'p' => Some(Onset::P),
        't' => Some(Onset::T),
        'k' => Some(Onset::K),
        's' => Some(Onset::S),
        'b' => Some(Onset::B),
        'g' => Some(Onset::G),
        'd' | 'r' => Some(Onset::R),

        'n' => Some(Onset::N),
        'm' => Some(Onset::M),
        _ => None,
    }
}

fn vocalic(c: char) -> Option<(Vowel, bool)> {
    match c {
        'e' => Some((Vowel::E, false)),
        'o' => Some((Vowel::O, false)),
        'i' => Some((Vowel::I, false)),
        'u' => Some((Vowel::U, false)),
        'a' => Some((Vowel::A, false)),
        'I' => Some((Vowel::I, true)),
        'E' => Some((Vowel::E, true)),
        'O' => Some((Vowel::O, true)),
        'U' => Some((Vowel::U, true)),
        'A' => Some((Vowel::A, true)),
        _ => None,
    }
}

#[allow(clippy::too_many_lines)]
fn convert_line_to_sylls(text_: &str) -> Vec<Syllable> {
    #[derive(Clone, Copy)]
    enum ParserState {
        Nothing,
        Onset(Onset),
        OnsetAndVowel(Onset, Vowel, bool),
        OnsetAndVowelAndNasalOrS(Onset, Vowel, bool, Onset),
    }

    let text: Vec<char> = text_.chars().collect();
    let mut ans = vec![];
    let mut state = ParserState::Nothing;
    for chr in text.clone() {
        match state {
            ParserState::Nothing => {
                if let Some(onset) = consonant(chr) {
                    state = ParserState::Onset(onset)
                } else {
                    panic!("Expected an onset, but got an unexpected character {}", chr)
                }
            }
            ParserState::Onset(onset) => {
                if let Some((vowel, accented)) = vocalic(chr) {
                    state = ParserState::OnsetAndVowel(onset, vowel, accented)
                } else if chr == '*' {
                    /* nothing */
                } else {
                    panic!(
                        "Expected a vowel, but got an unexpected character {}, in line {}",
                        chr,
                        text_.clone()
                    )
                }
            }
            ParserState::OnsetAndVowelAndNasalOrS(onset, vowel, accented, onset2) => {
                if let Some((vowel2, accented2)) = vocalic(chr) {
                    // open syllable
                    ans.push(Syllable(onset, vowel, None, accented));

                    state = ParserState::OnsetAndVowel(onset2, vowel2, accented2)
                } else if let Some(onset3) = consonant(chr) {
                    let coda = match onset2 {
                        Onset::M | Onset::N => Coda::Nasal,
                        Onset::S => Coda::H,
                        _ => panic!("cannot happen")
                    };

                    // closed syllable
                    ans.push(Syllable(onset, vowel, Some(coda), accented));

                    state = ParserState::Onset(onset3)
                } else {
                    panic!(
                        "Expected a vowel, but got an unexpected character {}, in line `{}`",
                        chr,
                        text_.clone()
                    )
                }
            }
            ParserState::OnsetAndVowel(onset, vowel, accented) => {
                if chr == 'n' || chr == 'm' || chr == 's' {
                    state = ParserState::OnsetAndVowelAndNasalOrS(
                        onset,
                        vowel,
                        accented,
                        consonant(chr).unwrap(),
                    )
                } else if let Some(new_onset) = consonant(chr) {
                    ans.push(Syllable(onset, vowel, None, accented));
                    state = ParserState::Onset(new_onset)
                } else if chr == 'h' {
                    ans.push(Syllable(onset, vowel, Some(Coda::H), accented));
                    state = ParserState::Nothing
                } else if chr == ':' || chr == ';' {
                    ans.push(Syllable(onset, vowel, Some(Coda::Long), accented));
                    state = ParserState::Nothing
                } else {
                    panic!(
                        "Expected an onset or a coda, but got an unexpected character {}, in line {}",
                        chr,
                        text_.clone()
                    )
                }
            }
        }
    }

    // after the loop is over, check the state for the last syllable
    match state {
        ParserState::Nothing => {}
        ParserState::Onset(onset) => panic!("The line ended with an onset {:?}", onset),
        ParserState::OnsetAndVowel(onset, vowel, accented) => {
            ans.push(Syllable(onset, vowel, None, accented))
        }
        ParserState::OnsetAndVowelAndNasalOrS(..) => panic!("The line ended with a nasal"),
    }
    ans
}

#[allow(clippy::too_many_lines)]
fn convert_line(text: &str) -> String {
    let text: Vec<char> = text.chars().collect();
    let mut ans = vec![];
    for (i, chr) in text.clone().into_iter().enumerate() {
        match chr {
            '\'' | 'q' => ans.push("ʔ"),

            'p' => {
                if ans.last() == Some(&"n") || ans.last() == Some(&"m") {
                    *(ans.last_mut().unwrap()) = "m";
                }
                ans.push("p");
            }
            't' => ans.push("t"),
            'k' => {
                if ans.last() == Some(&"n") || ans.last() == Some(&"m") {
                    *(ans.last_mut().unwrap()) = "ŋ";
                }
                ans.push("k")
            }
            's' => ans.push("s"),
            'n' => {
                // if closed syllable, ə becomes ɑ, except when the next syllable is accented
                if ans.last() == Some(&"ə") {
                    // closed syllable?
                    if text.len() == i + 1
                    /* end of a line */
                    || (!vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(&text[i + 1]) /* consonant follows */
                        && !vec!['A', 'E', 'I', 'O', 'U'].contains(&text[i + 2])
                /* accented vowel does not follow */)
                    {
                        *(ans.last_mut().unwrap()) = "ɑ"
                    }
                }
                ans.push("n")
            }
            'm' => {
                // if closed syllable, ə becomes ɑ, except when the next syllable is accented
                if ans.last() == Some(&"ə") {
                    // closed syllable?
                    if text.len() == i + 1 /* end of a line */
                    || (!vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(&text[i + 1]) /* consonant follows */
                        && !vec!['A', 'E', 'I', 'O', 'U'].contains(&text[i + 2])
                /* accented vowel does not follow */)
                    {
                        *(ans.last_mut().unwrap()) = "ɑ"
                    }
                }
                ans.push("m")
            }

            // a workaround for the case where the extrametricity elides the pause and lenites the plosives.
            '*' => {
                if ans.last() == Some(&"b") {
                    *(ans.last_mut().unwrap()) = "β";
                } else if ans.last() == Some(&"ɡ") {
                    *(ans.last_mut().unwrap()) = "ɣ";
                } else if ans.last() == Some(&"d") {
                    *(ans.last_mut().unwrap()) = "ɾ"
                }
            }

            'b' => {
                if ans.is_empty() {
                    ans.push("b");
                } else if ans.last() == Some(&"n") || ans.last() == Some(&"m") {
                    *(ans.last_mut().unwrap()) = "m";
                    ans.push("b");
                } else {
                    ans.push("β");
                }
            }

            'g' => {
                if ans.is_empty() {
                    ans.push("ɡ");
                } else if ans.last() == Some(&"n") {
                    *(ans.last_mut().unwrap()) = "ŋ";
                    ans.push("ɡ");
                } else {
                    ans.push("ɣ");
                }
            }

            'd' | 'r' => {
                if ans.is_empty()
                    || ans.last() == Some(&"n")
                    || (vec!['a', 'e', 'i', 'o', 'u'].contains(&text[i + 1])
                        && text.len() != i + 2
                        && vec!['d', 'r'].contains(&text[i + 2]))
                {
                    // `/ɾ/` + unaccented short vowel + `/ɾ/` turns the first `/ɾ/` into `[d]`
                    ans.push("d");
                } else {
                    ans.push("ɾ");
                }
            }

            'e' => ans.push("e̞"),
            'o' => ans.push("o̞"),
            'i' => ans.push("i"),
            'u' => ans.push("u"),
            'h' => ans.push("h"),
            ':' | ';' => {
                if ans.last() == Some(&"ə") {
                    *(ans.last_mut().unwrap()) = "ɑ"
                }
                ans.push("ː")
            }

            'a' => ans.push("ə"),

            // accentuated vowel: must put the stress mark before the consonant
            'I' => {
                ans.push(ans[ans.len() - 1]); // duplicate the consonant
                *(second_from_last_mut(&mut ans).unwrap()) = "ˈ"; // overwrite the first of the duplicated consonant with an accent mark
                ans.push("i")
            }

            'E' => {
                ans.push(ans[ans.len() - 1]); // duplicate the consonant
                *(second_from_last_mut(&mut ans).unwrap()) = "ˈ"; // overwrite the first of the duplicated consonant with an accent mark
                ans.push("e̞")
            }

            'O' => {
                ans.push(ans[ans.len() - 1]); // duplicate the consonant
                *(second_from_last_mut(&mut ans).unwrap()) = "ˈ"; // overwrite the first of the duplicated consonant with an accent mark
                ans.push("o̞")
            }

            'U' => {
                ans.push(ans[ans.len() - 1]); // duplicate the consonant
                *(second_from_last_mut(&mut ans).unwrap()) = "ˈ"; // overwrite the first of the duplicated consonant with an accent mark
                ans.push("u")
            }

            'A' => {
                ans.push(ans[ans.len() - 1]); // duplicate the consonant
                *(second_from_last_mut(&mut ans).unwrap()) = "ˈ"; // overwrite the first of the duplicated consonant with an accent mark
                ans.push("ɑ")
            }

            _ => {
                panic!("Unexpected character {}", chr)
            }
        }
    }
    let stage0 = ans.join("");
    lazy_static! {
        static ref RG1: Regex = Regex::new(r"^ʔɑ([mnŋ])([^ˈ])").unwrap();
        static ref RG2: Regex = Regex::new(r"^ʔɑː([^ˈ])").unwrap();
    }
    let stage1 = RG1.replace_all(&stage0, "ɑ$1$2");
    RG2.replace_all(&stage1, "ɑː$1").to_string()
}

pub fn convert(text: &str) -> String {
    text.lines()
        .map(|line| convert_line(line))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn convert_to_sylls(text: &str) -> Vec<Vec<Syllable>> {
    text.lines()
        .map(|line| convert_line_to_sylls(line))
        .collect::<Vec<_>>()
}
