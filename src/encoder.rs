const SEPARATOR: &str = "Α";

pub fn encode(s: &str) -> String {
    let mut res = String::from("");

    let lower = s.to_lowercase();
    for c in lower.chars() {
        let x = match c {
            'a' => "aA",
            'b' => "Aaaa",
            'c' => "AaAa",
            'd' => "Aaa",
            'e' => "a",
            'f' => "aaAa",
            'g' => "AAa",
            'h' => "aaaa",
            'i' => "aa",
            'j' => "aAAA",
            'k' => "AaA",
            'l' => "aAaa",
            'm' => "AA",
            'n' => "Aa",
            'o' => "AAA",
            'p' => "aAAa",
            'q' => "AAaA",
            'r' => "aAa",
            's' => "aaa",
            't' => "A",
            'u' => "aaA",
            'v' => "aaaA",
            'w' => "aAA",
            'x' => "AaaA",
            'y' => "AaAA",
            'z' => "AAaa",
            '1' => "aAAAA",
            '2' => "aaAAA",
            '3' => "aaaAA",
            '4' => "aaaaA",
            '5' => "aaaaa",
            '6' => "Aaaaa",
            '7' => "AAaaa",
            '8' => "AAAaa",
            '9' => "AAAAa",
            '0' => "AAAAA",
            ' ' | '_' => "aaAAaA",
            _ => panic!("only ascii letters and numbers allowed")
        };

        res += x;
        res += SEPARATOR;
    }
    return res;
}

pub fn decode(s: &str) -> String {
    let mut res = String::from("");

    let letters: Vec<&str> = s.split(SEPARATOR).collect();

    for enc_letter in letters {
        if enc_letter == "" { continue; }

        let dec_letter = match enc_letter {
            "aA" => "a",
            "Aaaa" => "b",
            "AaAa" => "c",
            "Aaa" => "d",
            "a" => "e",
            "aaAa" => "f",
            "AAa" => "g",
            "aaaa" => "h",
            "aa" => "i",
            "aAAA" => "j",
            "AaA" => "k",
            "aAaa" => "l",
            "AA" => "m",
            "Aa" => "n",
            "AAA" => "o",
            "aAAa" => "p",
            "AAaA" => "q",
            "aAa" => "r",
            "aaa" => "s",
            "A" => "t",
            "aaA" => "u",
            "aaaA" => "v",
            "aAA" => "w",
            "AaaA" => "x",
            "AaAA" => "y",
            "AAaa" => "z",
            "aAAAA" => "1",
            "aaAAA" => "2",
            "aaaAA" => "3",
            "aaaaA" => "4",
            "aaaaa" => "5",
            "Aaaaa" => "6",
            "AAaaa" => "7",
            "AAAaa" => "8",
            "AAAAa" => "9",
            "AAAAA" => "0",
            "aaAAaA" => "_",
            _ => panic!("Unknown symbol")
        };
        res += dec_letter;
    }

    return res;
}