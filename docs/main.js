function scansion(text) {
    return text.split("\n").map(t => scansion_line(t)).join("\n");
}

function scansion_line(text) {
    let arr = scan_syllables(text);
    let ans = "";
    let mora_count = 0;

    let i = 0;

    // extrametric `úmuu` or `úmm`
    if (arr[0] === "u\u0301" && arr[1] === "m" && arr[2] === "u" && arr[3] === "u") {
        ans += `${arr[0]} ${arr[1]}${arr[2]}${arr[3]} `;
        i = 4;
    } else if (arr[0] === "u\u0301" && arr[1] === "m" && arr[2] === "m") {
        ans += `${arr[0]} ${arr[1]}${arr[2]} `;
        i = 3;
    }

    for (; i < arr.length; i++) {
        ans += arr[i];
        if (arr[i][0] === "m") {
            mora_count += 2;
        } else {
            mora_count += 1;
        }

        if (mora_count === 4) {
            ans += " ";
            mora_count = 0;
        } else if (mora_count > 5) {
            ans += "<span style='color:red'>wrong meter!</span>";
            mora_count = 0;
        }
    }
    return ans;
}

function scan_syllables(text) {
    let ans = [];
    while (true) {
        if (text === "") {
            return ans;
        }
        const [syl_weight, remaining] = scan_one_syllable(text);
        ans.push(syl_weight);
        text = remaining;
    }
}

function scan_one_syllable(text) {
    if (text.match(/^([pbmtdrnskg'][aeiou][:h])/)
        || text.match(/^([pbmtdrnskg'][aeiou]ss)/)
        || text.match(/^([pbmtdrnskg'][aeiou][nm]$)/)
        || text.match(/^([pbmtdrnskg'][aeiou][nm][^AEIOUaeiou])/)) {
        return ["m", text.slice(3)];
    } else if (text.match(/^([pbmtdrnskg'][AEIOU][:h])/)
        || text.match(/^([pbmtdrnskg'][AEIOU]ss)/)
        || text.match(/^([pbmtdrnskg'][AEIOU][nm]$)/)
        || text.match(/^([pbmtdrnskg'][AEIOU][nm][^AEIOUaeiou])/)) {
        return ["m\u0301", text.slice(3)];
    } else if (text.match(/^([pbmtdrnskg'][aeiou])/)) {
        return ["u", text.slice(2)];
    } else if (text.match(/^([pbmtdrnskg'][AEIOU])/)) {
        return ["u\u0301", text.slice(2)];
    } else if (text.match(/^([bdrg]\*[AEIOU])/)) { // extrametricity elides the pause, leniting the plosives
        return ["u\u0301", text.slice(3)];
    } else {
        alert(`unparsable string: "${text}"`)
    }
}

function convert(text) {
    return text.split("\n").map(t => convert_line(t)).join("\n");
}
function convert_line(text) {
    let ans = [];
    for (let i = 0; i < text.length; i++) {
        switch (text[i]) {
            case "'": ans.push("ʔ"); break;

            case "p": {
                if (ans[ans.length - 1] === "n" || ans[ans.length - 1] === "m") {
                    ans[ans.length - 1] = "m";
                }
                ans.push("p"); break;
            }
            case "t": ans.push("t"); break;
            case "k": {
                if (ans[ans.length - 1] === "n" || ans[ans.length - 1] === "m") {
                    ans[ans.length - 1] = "ŋ";
                }
                ans.push("k"); break;
            }
            case "s": ans.push("s"); break;
            case "n": {
                // if closed syllable, ə becomes ɑ, except when the next syllable is accented
                if (ans[ans.length - 1] === "ə") {
                    // closed syllable?
                    if (text.length === i + 1 /* end of a line */) {
                        ans[ans.length - 1] = "ɑ"
                    } else if (!"aeiouAEIOU".includes(text[i + 1]) /* consonant follows */
                        && !"AEIOU".includes(text[i + 2]) /* accented vowel does not follow */) {
                        ans[ans.length - 1] = "ɑ"
                    }
                }
                ans.push("n"); break;
            }
            case "m": {
                // if closed syllable, ə becomes ɑ, except when the next syllable is accented
                if (ans[ans.length - 1] === "ə") {
                    // closed syllable?
                    if (text.length === i + 1 /* end of a line */) {
                        ans[ans.length - 1] = "ɑ"
                    } else if (!"aeiouAEIOU".includes(text[i + 1]) /* consonant follows */
                        && !"AEIOU".includes(text[i + 2]) /* accented vowel does not follow */) {
                        ans[ans.length - 1] = "ɑ"
                    }
                }
                ans.push("m"); break;
            }

            // a workaround for the case where the extrametricity elides the pause and lenites the plosives.
            case "*": {
                if (ans[ans.length - 1] === "b") {
                    ans[ans.length - 1] = "β";
                } else if (ans[ans.length - 1] === "ɡ") {
                    ans[ans.length - 1] = "ɣ";
                } else if (ans[ans.length - 1] === "d") {
                    ans[ans.length - 1] = "ɾ"
                }
                break;
            }

            case "b": {
                if (ans.length === 0) {
                    ans.push("b");
                } else if (ans[ans.length - 1] === "n" || ans[ans.length - 1] === "m") {
                    ans[ans.length - 1] = "m";
                    ans.push("b");
                } else {
                    ans.push("β");
                }
                break;
            }

            case "g": {
                if (ans.length === 0) {
                    ans.push("ɡ");
                } else if (ans[ans.length - 1] === "n") {
                    ans[ans.length - 1] = "ŋ";
                    ans.push("ɡ");
                } else {
                    ans.push("ɣ");
                }
                break;
            }

            case "d": case "r": {
                if (ans.length === 0) {
                    ans.push("d");
                } else if (ans[ans.length - 1] === "n") {
                    ans.push("d");
                } else if ("aeiou".includes(text[i + 1])
                    && text.length !== i + 2
                    && "dr".includes(text[i + 2])) {
                    // `/ɾ/` + unaccented short vowel + `/ɾ/` turns the first `/ɾ/` into `[d]`
                    ans.push("d");
                } else {
                    ans.push("ɾ");
                }
                break;
            }

            case "e": ans.push("e̞"); break;
            case "o": ans.push("o̞"); break;
            case "i": ans.push("i"); break;
            case "u": ans.push("u"); break;
            case "h": ans.push("h"); break;
            case ":": {
                if (ans[ans.length - 1] === "ə") {
                    ans[ans.length - 1] = "ɑ"
                }
                ans.push("ː"); break;
            }

            case "a": ans.push("ə"); break;

            // accentuated vowel: must put the stress mark before the consonant
            case "I": {
                ans.push(ans[ans.length - 1]); // duplicate the consonant
                ans[ans.length - 2] = "ˈ"; // overwrite the first of the duplicated consonant with an accent mark 
                ans.push("i"); break;
            }

            case "E": {
                ans.push(ans[ans.length - 1]); // duplicate the consonant
                ans[ans.length - 2] = "ˈ"; // overwrite the first of the duplicated consonant with an accent mark 
                ans.push("e̞"); break;
            }

            case "O": {
                ans.push(ans[ans.length - 1]); // duplicate the consonant
                ans[ans.length - 2] = "ˈ"; // overwrite the first of the duplicated consonant with an accent mark 
                ans.push("o̞"); break;
            }

            case "U": {
                ans.push(ans[ans.length - 1]); // duplicate the consonant
                ans[ans.length - 2] = "ˈ"; // overwrite the first of the duplicated consonant with an accent mark 
                ans.push("u"); break;
            }

            case "A": {
                ans.push(ans[ans.length - 1]); // duplicate the consonant
                ans[ans.length - 2] = "ˈ"; // overwrite the first of the duplicated consonant with an accent mark 
                ans.push("ɑ"); break;
            }

            default: {
                alert(`Unexpected character ${text[i]}`)
            }
        }
    }
    return ans.join("").replace(/^ʔɑ([mnŋ])([^ˈ])/g, "ɑ$1$2").replace(/^ʔɑː([^ˈ])/g, "ɑː$1");
}