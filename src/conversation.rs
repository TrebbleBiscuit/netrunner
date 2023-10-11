enum ConversationLine {
    Text(String),
}

impl ConversationLine {
    fn from_str(string: &str) -> Self {
        ConversationLine::Text(string.to_string())
    }
}

pub struct Conversation {
    lines: Vec<ConversationLine>,
    line_index: i32,
}

impl Conversation {
    pub fn new() -> Self {
        let mut lines = vec![
            ConversationLine::from_str("Hello"),
            ConversationLine::from_str("World"),
        ];
        Self {
            lines: lines,
            line_index: 0,
        }
    }

    pub fn show_lines_up_to(&self) -> String {
        let mut total_string = "".to_owned();
        for index in 0..self.line_index {
            match self.lines.get(index as usize) {
                Some(convo_line) => match convo_line {
                    ConversationLine::Text(txt) => {
                        total_string.push_str(&txt);
                        total_string.push_str("\n");
                    }
                },
                None => {}
            }
        }
        return total_string;
    }

    pub fn show_line(&self) -> String {
        match self.lines.get(self.line_index as usize) {
            Some(convo_line) => match convo_line {
                ConversationLine::Text(txt) => return txt.to_owned(),
            },
            None => return "".to_owned(),
        }
    }

    pub fn next_line(&mut self) {
        self.line_index += 1;
    }
}
