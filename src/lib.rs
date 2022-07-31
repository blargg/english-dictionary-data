
const DICT_TEXT: &'static str = include_str!("../data/dictionary.txt");

pub struct WordList<I> {
    lines: I,
    /// Sometimes we get more than one line, this is used to queue up the values.
    queue: Vec<&'static str>,
    paragraph: Vec<&'static str>,
}

impl<I: Iterator<Item=&'static str>> Iterator for WordList<I> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.len() > 0 {
            return self.queue.pop();
        }

        self.paragraph.clear();
        while let Some(line) = self.lines.next() {
            if !line.trim().is_empty() {
                self.paragraph.push(line)
            } else {
                // new paragraph
                // TODO, this might miss the last paragraph because we only queue the words when we hit a new paragraph.
                self.queue_words();
                return self.next();
            }
        }
        None
    }
}

impl<I> WordList<I> {
    fn queue_words<'a>(&mut self) {
        self.queue = get_word(&self.paragraph);
        self.paragraph.clear();
    }
}

fn get_word<'a>(paragraph: &Vec<&'a str>) -> Vec<&'a str> {
    if paragraph.len() != 2 {
        return vec![];
    }

    let line = paragraph[0];
    // starts with a number means its a definition
    if line.starts_with(('0'..'9').collect::<Vec<_>>().as_slice()) ||
        line.starts_with("Defn") || 
        line.starts_with("Syn") ||
        line.starts_with("Note")
    {
        return vec![];
    }

    line.split(';').map(|s| s.trim()).collect::<Vec<_>>()
}

// fn all_words() -> impl Iterator<Item=&'static str> {
pub fn all_words() -> WordList<impl Iterator<Item=&'static str>> {

    let mut lines = DICT_TEXT
        .lines()
        .skip_while(|line| !line.contains("Produced by"));

    lines.next();


    WordList {
        lines,
        queue: Vec::new(),
        paragraph: Vec::new(),
    }

}

#[cfg(test)]
mod tests {
    use crate::all_words;

    #[test]
    fn gets_words() {
        assert_eq!("A", all_words().nth(0).unwrap());
        assert_eq!("AARONICAL", all_words().nth(10).unwrap());
    }
}
