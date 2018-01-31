pub(crate) struct CaptchaIter<T> {
    state: IterState,
    source: T,
}

impl<T: Iterator<Item = u8>> CaptchaIter<T> {
    pub fn new<S>(source: S) -> Self
    where
        S: IntoIterator<Item = u8, IntoIter = T>,
    {
        Self {
            state: IterState::New,
            source: source.into_iter(),
        }
    }
}

enum IterState {
    New,
    Running { first: u8, current: u8 },
    Complete,
}

impl<T: Iterator<Item = u8>> Iterator for CaptchaIter<T> {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            IterState::New => {
                let first = self.source.next()?;
                let current = self.source.next()?;

                self.state = IterState::Running { first, current };
                Some((first, current))
            }

            IterState::Running { first, current } => match self.source.next() {
                None => {
                    self.state = IterState::Complete;
                    Some((first, current))
                }

                Some(n) => {
                    self.state = IterState::Running { first, current: n };
                    Some((current, n))
                }
            },

            IterState::Complete => None,
        }
    }
}

pub fn sum_captcha(s: &str) -> u32 {
    fn eval(u: u8) -> u8 {
        u - b'0'
    }

    CaptchaIter::new(s.bytes().map(eval))
        .filter(|pair| pair.0 == pair.1)
        .map(|pair| pair.0 as u32)
        .sum()
}
