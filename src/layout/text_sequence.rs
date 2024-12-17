#[derive(Clone)]
pub struct TextSequence {
    text: String,
    font: SizedFont
}

impl TextSequence {
    pub fn split_by_line_breaks(&self) -> impl Iterator<Item=TextSequence> + '_ {
        self.text.split(|ch| ch == '\n').map(
            |text| TextSequence { 
                text: text.to_owned(), 
                font: self.font.clone() 
            })
    }
}