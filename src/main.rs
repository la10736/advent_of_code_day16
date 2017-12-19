use std::io::prelude::*;

fn read_all<S: AsRef<std::path::Path>>(path: S) -> String {
    let mut content = String::new();
    let mut f = std::fs::File::open(path).unwrap();
    f.read_to_string(&mut content).unwrap();
    content
}

fn main() {
    let len = std::env::args()
        .nth(1)
        .unwrap_or(String::from("5"))
        .parse().unwrap();
    let fname = std::env::args()
        .nth(2)
        .unwrap_or(String::from("example"));
    let times = std::env::args()
        .nth(3)
        .unwrap_or(String::from("1"))
        .parse().unwrap();
    let content = read_all(fname);

    let actions = content.split(',')
        .map(|token| token.parse::<Action>().unwrap())
        .collect::<Vec<_>>();
    let mut result = Programs::new(len);
    let mut results = std::collections::HashMap::new();
    let mut step = 1;

    for i in 0..times {
        result = actions
            .iter()
            .fold(result, |p, &a| p.apply(a));

        let k = result.to_string();
        if let Some(store) = results.get(&k) {
            step = i - store;
            println!("{} [{}]", store, step);
            break;
        }
        results.insert(k, i);
    }

    for i in 0..(times % step)-1 {
        result = actions
            .iter()
            .fold(result, |p, &a| p.apply(a));
    }

    println!("Result = {}", result.to_string())
}

struct Programs {
    data: Vec<char>
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Action {
    Exchange(usize, usize),
    Partner(char, char),
    Spin(usize),
}

impl std::str::FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            match s.chars().nth(0).unwrap() {
                's' => Action::Spin(Self::val(&s[1..])?),
                'x' => {
                    let mut splitter = s[1..].splitn(2, '/').map(Self::val);
                    Action::Exchange(splitter.next().unwrap()?, splitter.next().unwrap()?)
                }
                'p' => {
                    let mut splitter = s[1..].chars();
                    Action::Partner(splitter.next().unwrap(), splitter.nth(1).unwrap())
                }

                _ => panic!("Non rieco a comprendere '{}'", s)
            }
        )
    }
}

impl Action {
    fn val(s: &str) -> Result<usize, String> {
        s.parse().map_err(|e| format!("{}", e))
    }
}


impl Programs {
    fn new(size: u8) -> Self {
        Programs {
            data: (0..size).map(Self::p).collect(),
        }
    }

    fn apply(self, action: Action) -> Self {
        match action {
            Action::Exchange(a, b) => { self.exchange(a, b) }
            Action::Partner(p0, p1) => { self.partner(p0, p1) }
            Action::Spin(s) => { self.spin(s) }
        }
    }

    fn exchange(mut self, a: usize, b: usize) -> Self {
        self.data.swap(a, b);
        self
    }

    fn partner(mut self, p0: char, p1: char) -> Self {
        let (p0, p1) = (self.pos(p0), self.pos(p1));
        self.data.swap(p0, p1);
        self
    }

    fn pos(&self, p: char) -> usize {
        self.data.iter().position(|&c| c == p).unwrap()
    }

    fn spin(self, amount: usize) -> Self {
        let mut new = Vec::with_capacity(self.data.len());
        let amount = amount % self.data.len();
        let split_at = self.data.len() - amount;
        new.extend(&self.data[split_at..]);
        new.extend(&self.data[..split_at]);
        Self { data: new }
    }

    fn p(n: u8) -> char {
        match n {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            8 => 'i',
            9 => 'j',
            10 => 'k',
            11 => 'l',
            12 => 'm',
            13 => 'n',
            14 => 'o',
            15 => 'p',
            _ => unreachable!()
        }
    }
}

impl std::string::ToString for Programs {
    fn to_string(&self) -> String {
        self.data.iter().fold(
            String::new(), |mut s, &c| {
                s.push(c);
                s
            },
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn programs() {
        assert_eq!("abc", &Programs::new(3).to_string());
        assert_eq!("abcde", &Programs::new(5).to_string());
        assert_eq!("abcdefg", &Programs::new(7).to_string());
    }

    #[test]
    fn exchange() {
        assert_eq!("adcbe", Programs::new(5)
            .apply(Action::Exchange(1, 3))
            .to_string());
        assert_eq!("cbade", Programs::new(5)
            .apply(Action::Exchange(2, 0))
            .to_string());
        assert_eq!("abcde", Programs::new(5)
            .apply(Action::Exchange(4, 4))
            .to_string());
    }

    #[test]
    fn partner() {
        assert_eq!("adcbe", Programs::new(5)
            .apply(Action::Partner('b', 'd'))
            .to_string());
        assert_eq!("cbade", Programs::new(5)
            .apply(Action::Partner('c', 'a'))
            .to_string());
        assert_eq!("abcde", Programs::new(5)
            .apply(Action::Partner('e', 'e'))
            .to_string());
    }

    #[test]
    fn spin() {
        assert_eq!("cdeab", Programs::new(5)
            .apply(Action::Spin(3))
            .to_string()
        )
    }

    #[test]
    fn integration() {
        assert_eq!("baedc", Programs::new(5)
            .apply(Action::Spin(1))
            .apply(Action::Exchange(3, 4))
            .apply(Action::Partner('e', 'b'))
            .to_string())
    }

    #[test]
    fn parse_action() {
        assert_eq!(Action::Spin(3), "s3".parse().unwrap());
        assert_eq!(Action::Exchange(3, 4), "x3/4".parse().unwrap());
        assert_eq!(Action::Exchange(12, 15), "x12/15".parse().unwrap());
        assert_eq!(Action::Partner('e', 'b'), "pe/b".parse().unwrap());
    }
}
