use regex::Regex;

struct Summary<'a> {
    long: &'a str,
    short: &'a str,
    kind: &'a str,
}
impl<'a> Summary<'a> {
    fn parse(s: &'a str) -> Option<Summary<'a>> {
        const RE_STR: &str = "^([^;]*);([^(]*)\\(([^)]*)\\) - (.*)$";
        lazy_static! {
            static ref RE: Regex = Regex::new(RE_STR).unwrap();
        }

        let cap = RE.captures(s)?;
        Some(Summary {
            long: cap.get(2).unwrap().as_str().trim(),
            short: cap.get(3).unwrap().as_str().trim(),
            kind: cap.get(4).unwrap().as_str().trim(),
        })
    }
    fn get_name(&self) -> String {
        if self.long.len() >= 12 {
            format!("{} - {}", self.short, self.kind)
        } else {
            format!("{} - {}", self.long, self.kind)
        }
    }
}

pub fn clean_summary(sum: &str) -> Option<String> {
    Summary::parse(sum)
        .map(|sum| sum.get_name())
}
