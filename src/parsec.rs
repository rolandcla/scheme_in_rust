pub type Parser<T> = Box<Fn(&str) -> Vec<(T, &str)>>;

pub fn result<T>(v: T) -> Parser<T>
where
    T: 'static + Copy,
{
    Box::new(move |inp| vec![(v, inp)])
}

pub fn zero() -> Parser<i32> {
    Box::new(|_| vec![])
}

pub fn item() -> Parser<char> {
    Box::new(|inp| {
        let mut chars = inp.chars();
        match chars.next() {
            Some(c) => vec![(c, chars.as_str())],
            None => vec![],
        }
    })
}

pub fn seq<T, U>(p: Parser<T>, q: Parser<U>) -> Parser<(T, U)>
where
    T: 'static + Copy,
    U: 'static + Copy,
{
    Box::new(move |inp| {
        let mut r = vec![];
        for (v, inp1) in p(inp) {
            for (w, inp2) in q(inp1) {
                r.push(((v, w), inp2))
            }
        }
        r
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result_succeeds_without_consuming_input_string() {
        assert_eq!(result(42)("brol"), vec![(42, "brol")]);
    }

    #[test]
    fn zero_always_fails() {
        assert_eq!(zero()("brol"), vec![]);
    }

    #[test]
    fn item_consumes_the_first_char() {
        assert_eq!(item()(""), vec![]);
        assert_eq!(item()("a"), vec![('a', "")]);
        assert_eq!(item()("↓"), vec![('↓', "")]);
        assert_eq!(item()("brol"), vec![('b', "rol")]);
        assert_eq!(item()("↓brol"), vec![('↓', "brol")]);
    }

    #[test]
    fn seq_applies_one_parser_after_another() {
        let two_items = seq(item(), item());
        assert_eq!(two_items("brol"), [(('b', 'r'), "ol")]);
    }
}
