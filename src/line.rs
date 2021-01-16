use super::convert;
pub struct Line(Vec<syllabify::Syllable>);

pub mod syllabify;

impl Line {
    #[must_use]
    pub fn new(line: &str) -> Self {
        let ans = Self(syllabify::convert_line_to_sylls(line));

        // FIXME: this duplicates the "rare instance" warnings
        match convert::to_ipa(&ans) {
            Err(e) => panic!("{}, in line `{}`", e, line),
            Ok(_) => {}
        }
        ans
    }

    #[must_use]
    pub fn to_ipa(&self) -> String {
        convert::to_ipa(self).unwrap()
    }

    // clippy is currently buggy (https://github.com/rust-lang/rust-clippy/issues/4979)
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn into_vec(self) -> Vec<syllabify::Syllable> {
        self.0
    }

    #[must_use]
    pub const fn as_vec(&self) -> &Vec<syllabify::Syllable> {
        &self.0
    }
}
