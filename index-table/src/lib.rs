#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter_word() {
        let word = "Hello, World! My name is Nils Martel.";
        let iter = Words {
            chars: word.chars(),
        };

        let collection: Vec<String> = iter.collect();

        assert_eq!(
            collection,
            vec!["Hello", "World", "My", "name", "is", "Nils", "Martel"]
        );
    }
}

struct Words<'a> {
    chars: std::str::Chars<'a>,
}

impl<'a> Iterator for Words<'a> {
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
