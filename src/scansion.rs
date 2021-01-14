use regex::Regex;
mod tests {
    #[test]
    fn it_works4() {
        use super::*;
        assert_eq!(
            scansion(
                "
mentirinAssege'a:makonUmba
sa:mAtige:me:gAssimanI:
na:'a:ri:tInasa:matenIssi
sa:tInasa:ba:bAssagonI:
'e:rami'Issibo'a:garonIhpi
'i:sAturo:ma:nAssarenI:
mi:ri:'Antige'a:masorI:'a
no:gArume:'ambAhpanamI:
'i:ma:ka:kAriba:rimorEhti
'i:tIrone:'ambAssedarI:
'i:mo:'Assagoma:rakarEhpi
'Ana:gasanIhtibAgassamanI:
mi:sUpene:'a:gAntironE:'a
sa:mo:'Indiko'AssiribOnda"
            ),
            "
muu ḿuu muu ḿu
múu mm ḿuu ḿ
mm múu muu ḿu
múu mm ḿuu ḿ
muu ḿuu muu ḿu
múu mm ḿuu ḿ
mm ḿuu muu ḿu
múu mm ḿuu ḿ
mm múu muu ḿu
múu mm ḿuu ḿ
mm ḿuu muu ḿu
ú muu ḿuú muu ḿ
múu mm ḿuu ḿu
mm ḿuu ḿuu ḿu"
        )
    }

    #[test]
    fn it_works3() {
        use super::*;
        assert_eq!(
            scansion(
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
múu mm ḿuu ḿu
muu ḿuu muu ḿu
múu mm ḿuu ḿu
mḿ mm úum ḿ
muu múu muu ḿu
ú muu ḿuú muu ḿ
mḿ mm ḿuu ḿu
mm úum ḿuu ḿ
mḿ mm ḿuu ḿu
múu muu úum ḿ
mm múu muu ḿu
múu múu muu ḿ
mm múu muu ḿu
múu mm úum ḿ
ḿuu ḿuu muu ḿu
ú muu ḿuu ḿuu ḿu"
        )
    }

    #[test]
    fn it_works2() {
        use super::*;
        assert_eq!(
            scansion(
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
ḿuu ḿuu muu ḿu
múu mm ḿuu ḿ
mm múu muu ḿu
múu mm ḿuu ḿ
mm múu muu ḿu
muu ḿuu muu ḿ
múu mm ḿuu ḿu
múu mm ḿuu ḿ
múu mm ḿuu mu
mm úuuu muu ḿu"
        )
    }

    #[test]
    fn it_works() {
        use super::*;
        assert_eq!(
            scansion(
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
ḿuu ḿuu ḿuu ḿm 
múu mm ḿuu ḿu
muu ḿuu ḿm múu 
mm ḿuu úum ḿ
ḿuu múu mm ḿu
muu ḿuu úum ḿ
ḿuu ḿuu ḿuu ḿm 
muu ḿm ḿuu ḿ
mm ḿuu muu ḿu
muu ḿuú muu ḿ
ḿuu mḿ muu ḿu
mm ḿuu ḿuu ḿ
muu múu mm ḿu
ḿuu ḿuu ḿuu ḿu"
        )
    }
}

pub fn scansion(text: &str) -> String {
    text.lines()
        .map(|line| scansion_line(line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn scan_syllables(mut text: &str) -> Vec<String> {
    let mut ans = vec![];
    loop {
        if text.is_empty() {
            return ans;
        }
        let (syl_weight, remaining) = scan_one_syllable(text);
        ans.push(syl_weight.to_owned());
        text = remaining;
    }
}

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().nth(pos) {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}

fn scan_one_syllable(text: &str) -> (&'static str, &str) {
    lazy_static! {
        static ref M1: Regex = Regex::new(r"^([pbmtdrnskg'][aeiou][:;h])").unwrap();
        static ref M2: Regex = Regex::new(r"^([pbmtdrnskg'][aeiou]ss)").unwrap();
        static ref M3: Regex = Regex::new(r"^([pbmtdrnskg'][aeiou][nm]$)").unwrap();
        static ref M4: Regex = Regex::new(r"^([pbmtdrnskg'][aeiou][nm][^AEIOUaeiou])").unwrap();
        static ref MACC1: Regex = Regex::new(r"^([pbmtdrnskg'][AEIOU][:;h])").unwrap();
        static ref MACC2: Regex = Regex::new(r"^([pbmtdrnskg'][AEIOU]ss)").unwrap();
        static ref MACC3: Regex = Regex::new(r"^([pbmtdrnskg'][AEIOU][nm]$)").unwrap();
        static ref MACC4: Regex = Regex::new(r"^([pbmtdrnskg'][AEIOU][nm][^AEIOUaeiou])").unwrap();
        static ref U: Regex = Regex::new(r"^([pbmtdrnskg'][aeiou])").unwrap();
        static ref UACC1: Regex = Regex::new(r"^([pbmtdrnskg'][AEIOU])").unwrap();
        static ref UACC2: Regex = Regex::new(r"^([bdrg]\*[AEIOU])").unwrap();
    }
    if M1.find(text).is_some()
        || M2.find(text).is_some()
        || M3.find(text).is_some()
        || M4.find(text).is_some()
    {
        return ("m", crop_letters(text, 3));
    } else if MACC1.find(text).is_some()
        || MACC2.find(text).is_some()
        || MACC3.find(text).is_some()
        || MACC4.find(text).is_some()
    {
        return ("m\u{0301}", crop_letters(text, 3));
    } else if U.find(text).is_some() {
        return ("u", crop_letters(text, 2));
    } else if UACC1.find(text).is_some() {
        return ("u\u{0301}", crop_letters(text, 2));
    } else if UACC2.find(text).is_some() {
        // extrametricity elides the pause, leniting the plosives
        return ("u\u{0301}", crop_letters(text, 3));
    } else {
        panic!("unparsable string: \"{}\"", text)
    }
}

fn scansion_line(text: &str) -> String {
    let arr = scan_syllables(text);
    let mut ans = String::new();
    let mut mora_count = 0;

    let mut i = 0;

    // extrametric `úmuu` or `úmm`
    if arr.len() >= 4 && arr[0] == "u\u{0301}" && arr[1] == "m" && arr[2] == "u" && arr[3] == "u" {
        ans += &format!("{} {}{}{} ", arr[0], arr[1], arr[2], arr[3]);
        i = 4;
    } else if arr.len() >= 3 && arr[0] == "u\u{0301}" && arr[1] == "m" && arr[2] == "m" {
        ans += &format!("{} {}{} ", arr[0], arr[1], arr[2]);
        i = 3;
    }

    while i < arr.len() {
        ans += &arr[i];
        if arr[i].starts_with('m') {
            mora_count += 2;
        } else {
            mora_count += 1;
        }

        if mora_count == 4 {
            ans += " ";
            mora_count = 0;
        } else if mora_count > 5 {
            ans += "<span style='color:red'>wrong meter!</span>";
            mora_count = 0;
        }
        i += 1;
    }
    ans
}
