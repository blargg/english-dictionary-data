
const DICT_TEXT: &'static str = include_str!("../data/dictionary.txt");

pub struct WordList<I> {
    lines: I,
    paragraph: Vec<&'static str>,
}

impl<I: Iterator<Item=&'static str>> Iterator for WordList<I> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.paragraph.clear();
        while let Some(line) = self.lines.next() {
            if !line.trim().is_empty() {
                self.paragraph.push(line)
            } else {
                // new paragraph
                // TODO, this might miss the last paragraph
                let word = get_word(&self.paragraph);
                self.paragraph.clear();
                return word.or_else(|| self.next());
            }
        }
        None
    }
}

fn get_word<'a>(paragraph: &Vec<&'a str>) -> Option<&'a str> {
    if paragraph.len() != 2 {
        return None;
    }

    let line = paragraph[0];
    // starts with a number means its a definition
    if line.starts_with(('0'..'9').collect::<Vec<_>>().as_slice()) ||
        line.starts_with("Defn") || 
        line.starts_with("Syn") ||
        line.starts_with("Note")
    {
        return None;
    }

    Some(line)
}

// fn all_words() -> impl Iterator<Item=&'static str> {
pub fn all_words() -> WordList<impl Iterator<Item=&'static str>> {

    let mut lines = DICT_TEXT
        .lines()
        .skip_while(|line| !line.contains("Produced by"));

    lines.next();


    WordList {
        lines,
        paragraph: Vec::new(),
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
