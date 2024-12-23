#[derive(Clone)]
pub struct TextSequence {
    text: String,
}

impl From<&str> for TextSequence {
    fn from(value: &str) -> Self {
        Self{text: value.to_owned()}
    }
}

impl TextSequence {
    pub fn split_by_line_breaks(&self) -> impl Iterator<Item = TextSequence> + '_ {
        self.text.split(|ch| ch == '\n').map(|text| TextSequence {
            text: text.to_owned(),
        })
    }
}

