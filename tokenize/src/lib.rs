pub struct Tokens<'a> {
    chars: std::str::Chars<'a>,
}

impl<'a> Tokens<'a> {
    pub fn new(chars: std::str::Chars<'a>) -> Self {
        Self { chars }
    }

    pub fn from_str(s: &'a str) -> Self {
        Self { chars: s.chars() }
    }
}

pub trait IntoTokens<'a> {
    fn tokens(&'a self) -> Tokens<'a>;
}

impl<'a> IntoTokens<'a> for str {
    fn tokens(&'a self) -> Tokens<'a> {
        Tokens::from_str(self)
    }
}

impl<'a> IntoTokens<'a> for String {
    fn tokens(&'a self) -> Tokens<'a> {
        Tokens::from_str(self)
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut s = String::with_capacity(6);

        fn alpha(c: char) -> bool {
            c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z'
        }

        loop {
            let c = self.chars.next();
            match c {
                None => return None,
                Some(c) if alpha(c) => {
                    s.push(c);
                    break;
                }
                _ => continue,
            }
        }

        loop {
            let c = self.chars.next();
            match c {
                None => break,
                Some(c) if alpha(c) => s.push(c),
                _ => break,
            }
        }

        return Some(s);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter_token() {
        let sentence = "Hello, World! My name is Nils Martel.";

        let collection: Vec<String> = sentence.tokens().collect();

        assert_eq!(
            collection,
            vec!["Hello", "World", "My", "name", "is", "Nils", "Martel"]
        );
    }
}
