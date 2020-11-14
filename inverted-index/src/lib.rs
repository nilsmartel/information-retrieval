#[cfg(test)]
mod tests {

    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::collections::HashMap;
pub type Token = String;
pub type DocId = String;
pub type Pos = usize;

#[derive(Clone, Debug)]
pub struct InvertedIndex {
    pub data: HashMap<Token, HashMap<DocId, Vec<Pos>>>,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn index_doc(&mut self, name: &str, content: &str) {
        use tokenize::IntoTokens;

        let tokens = content.tokens();

        for (pos, token) in tokens.enumerate() {
            if let Some(mut posting) = self.data.get_mut(&token) {
                if let Some(mut pos_list) = posting.get_mut(name) {
                    pos_list.push(pos);
                } else {
                    posting.insert(name.to_string(), vec![pos]);
                }
            } else {
                self.data.insert(token, {
                    let value = vec![pos];
                    let mut postings = HashMap::new();
                    postings.insert(name.to_string(), value);
                    postings
                });
            }
        }
    }
}
