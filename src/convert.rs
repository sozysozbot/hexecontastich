use regex::Regex;

pub fn elide_initial_glottal_stop(ans: &str) -> String {
    lazy_static! {
        static ref RG1: Regex = Regex::new(r"^ʔɑ([mnŋ])([^ˈ])").unwrap();
        static ref RG2: Regex = Regex::new(r"^ʔɑː([^ˈ])").unwrap();
    }
    let stage1 = RG1.replace_all(ans, "ɑ$1$2");
    RG2.replace_all(&stage1, "ɑː$1").to_string()
}
