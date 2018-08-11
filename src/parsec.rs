pub type ParserFn<T> = Box<Fn(&str) -> Vec<(T, &str)>>;
pub struct Parser<T> {
    parse_fn: ParserFn<T>,
}

// 3 primitive parsers ---------------------------------------------------
pub fn result<T>(v: T) -> Parser<T>
where
    T: 'static + Copy,
{
    Parser::new(Box::new(move |inp| vec![(v, inp)]))
}

pub fn zero() -> Parser<i32> {
    Parser::new(Box::new(|_| vec![]))
}

pub fn item() -> Parser<char> {
    Parser::new(Box::new(|inp| {
        let mut chars = inp.chars();
        match chars.next() {
            Some(c) => vec![(c, chars.as_str())],
            None => vec![],
        }
    }))
}

impl<T> Parser<T>
where
    T: 'static + Copy,
{
    pub fn new(parse_fn: ParserFn<T>) -> Parser<T> {
        Parser { parse_fn }
    }

    pub fn parse(self, inp: &str) -> Vec<(T, &str)> {
        (self.parse_fn)(inp)
    }

    // Parser combinators ----------------------------------------------
    pub fn seq<U>(self, other: Parser<U>) -> Parser<(T, U)>
    where
        U: 'static + Copy,
    {
        Parser {
            parse_fn: (Box::new(move |inp| {
                let mut r = vec![];
                for (v, inp1) in (self.parse_fn)(inp) {
                    for (w, inp2) in (other.parse_fn)(inp1) {
                        r.push(((v, w), inp2));
                    }
                }
                r
            })),
        }
    }

    pub fn bind<U>(self, f: Box<Fn(T) -> Parser<U>>) -> Parser<U>
    where
        U: 'static + Copy,
    {
        Parser::new(Box::new(move |inp| {
            let mut r = vec![];
            for (v, inp1) in (self.parse_fn)(inp) {
                for (w, inp2) in f(v).parse(inp1) {
                    r.push((w, inp2));
                }
            }
            r
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result_succeeds_without_consuming_input_string() {
        assert_eq!(result(42).parse("brol"), vec![(42, "brol")]);
    }

    #[test]
    fn zero_always_fails() {
        assert_eq!(zero().parse("brol"), vec![]);
    }

    #[test]
    fn item_consumes_the_first_char() {
        assert_eq!(item().parse(""), vec![]);
        assert_eq!(item().parse("a"), vec![('a', "")]);
        assert_eq!(item().parse("↓"), vec![('↓', "")]);
        assert_eq!(item().parse("brol"), vec![('b', "rol")]);
        assert_eq!(item().parse("↓brol"), vec![('↓', "brol")]);
    }

    #[test]
    fn seq_applies_one_parser_after_another() {
        let two_items = item().seq(item());
        assert_eq!(two_items.parse("brol"), [(('b', 'r'), "ol")]);
    }

    #[test]
    fn bind_first_test() {
        let two_items = item().bind(Box::new(|v| {
            item()
        }));
        assert_eq!(two_items.parse("brol"), [('r', "ol")]);
    }
}
