#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Syllable {
    pub onset: Onset,
    pub vowel: Vowel,
    pub coda: Option<Coda>,
    pub accented: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Coda {
    Nasal,
    H,
    Long,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Vowel {
    A,
    E,
    I,
    O,
    U,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

const fn consonant(c: char) -> Option<Onset> {
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

const fn vocalic(c: char) -> Option<(Vowel, bool)> {
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
#[must_use]
pub fn convert_line_to_sylls(text_: &str) -> Vec<Syllable> {
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
                    ans.push(Syllable {
                        onset,
                        vowel,
                        coda: None,
                        accented,
                    });

                    state = ParserState::OnsetAndVowel(onset2, vowel2, accented2)
                } else if let Some(onset3) = consonant(chr) {
                    let coda = match onset2 {
                        Onset::M | Onset::N => Coda::Nasal,
                        Onset::S => Coda::H,
                        _ => panic!("cannot happen"),
                    };

                    // closed syllable
                    ans.push(Syllable {
                        onset,
                        vowel,
                        coda: Some(coda),
                        accented,
                    });

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
                    ans.push(Syllable {
                        onset,
                        vowel,
                        coda: None,
                        accented,
                    });
                    state = ParserState::Onset(new_onset)
                } else if chr == 'h' {
                    ans.push(Syllable {
                        onset,
                        vowel,
                        coda: Some(Coda::H),
                        accented,
                    });
                    state = ParserState::Nothing
                } else if chr == ':' || chr == ';' {
                    ans.push(Syllable {
                        onset,
                        vowel,
                        coda: Some(Coda::Long),
                        accented,
                    });
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
        ParserState::OnsetAndVowel(onset, vowel, accented) => ans.push(Syllable {
            onset,
            vowel,
            coda: None,
            accented,
        }),
        ParserState::OnsetAndVowelAndNasalOrS(..) => panic!("The line ended with a nasal"),
    }
    ans
}
