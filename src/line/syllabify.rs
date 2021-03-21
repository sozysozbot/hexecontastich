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

    /// following Vowel::A, gives /aj/; only used in interjections
    Falling,
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

    /// Used only for interjections
    H,
}

impl Coda {
    #[must_use]
    pub const fn to_representative_ipa(self) -> &'static str {
        match self {
            Self::Nasal => "N",
            Self::H => "h",
            Self::Long => "ː",
            Self::Falling => "j",
        }
    }
}

impl Vowel {
    #[must_use]
    pub const fn to_representative_ipa(self) -> &'static str {
        match self {
            Self::A => "ə",
            Self::E => "e̞",
            Self::I => "i",
            Self::O => "o̞",
            Self::U => "u",
        }
    }
}

impl Onset {
    #[must_use]
    pub const fn to_representative_ipa(self) -> &'static str {
        match self {
            Self::P => "p",
            Self::B => "β",
            Self::M => "m",
            Self::T => "t",
            Self::R => "ɾ",
            Self::N => "n",
            Self::S => "s",
            Self::K => "k",
            Self::G => "ɣ",
            Self::Q => "ʔ",
            Self::H => "h",
        }
    }
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

fn replace<T>(source: &[T], from: &[T], to: &[T]) -> Vec<T>
where
    T: Clone + PartialEq,
{
    let mut result = source.to_vec();
    let from_len = from.len();
    let to_len = to.len();

    let mut i = 0;
    while i + from_len <= result.len() {
        if result[i..].starts_with(from) {
            result.splice(i..i + from_len, to.iter().cloned());
            i += to_len;
        } else {
            i += 1;
        }
    }

    result
}

pub fn convert_line_to_sylls(text_: &str) -> Vec<Syllable> {
    let ans = convert_line_to_sylls_literally(text_);

    // There is a rare instance of an interjection known to be monosyllabic by metric but written as a three-syllable sequence `/ɣəh/ + /kə/ + /ɣi/`. It is usually interpreted that this sequence denoted `/hɑj/`.
    replace(
        &ans,
        &vec![
            Syllable {
                onset: Onset::G,
                accented: false,
                coda: Some(Coda::H),
                vowel: Vowel::A,
            }, // ɣəh
            Syllable {
                onset: Onset::K,
                accented: false,
                coda: None,
                vowel: Vowel::A,
            }, // kə
            Syllable {
                onset: Onset::G,
                accented: false,
                coda: None,
                vowel: Vowel::I,
            }, // ɣi
        ],
        &vec![Syllable {
            onset: Onset::H,
            accented: false,
            coda: Some(Coda::Falling),
            vowel: Vowel::A,
        }],
    )
}

#[allow(clippy::too_many_lines)]
#[must_use]
pub fn convert_line_to_sylls_literally(text_: &str) -> Vec<Syllable> {
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
