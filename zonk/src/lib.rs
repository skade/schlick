#![no_std]

pub struct Vocabulary<'a> {
    verbs: &'a [&'a str],
    objects: &'a [&'a str],
    people: &'a [&'a str],
}

#[derive(Default)]
pub struct Sentence<'a> {
    verb: Option<&'a str>,
    object: Option<&'a str>,
    people: Option<&'a str>,
}

pub fn parse<'a>(input: &'a str, vocabulary: Vocabulary<'a>) -> Sentence<'a> {
    let mut sentence = Sentence::default();
    let mut splitted = input.split(' ').fuse();

    sentence.verb   = splitted.next();
    sentence.object = splitted.next();
    sentence.people = splitted.next();

    sentence
}

struct Object {
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
