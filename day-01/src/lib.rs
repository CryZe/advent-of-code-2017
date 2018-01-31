#![feature(test)]
#![feature(slice_rotate)]

mod iter;

pub fn part1(text: &str) -> usize {
    text.bytes()
        .zip(text.bytes().skip(1).chain(text.bytes()))
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| (a - b'0') as usize)
        .sum()
}

pub fn slikrick(text: &str) -> u32 {
    let nums: Vec<char> = text.chars().collect();
    let length = text.len();
    let addition = length / 2;

    let sum = nums.iter()
        .enumerate()
        .take(addition)
        .filter(|&(index, &num)| num == nums[index + addition])
        .fold(0, |acc, (_, num)| acc + num.to_digit(10).unwrap_or(0) * 2);

    sum
}

pub fn restioson(text: &str) -> u64 {
    let digits: Vec<u8> = text.chars()
        .map(|digit| {
            digit.to_digit(10).expect("Digits should be ascii!") as u8
        })
        .collect();
    let mut next_digits = digits.clone();
    next_digits.rotate(digits.len() / 2);

    // Add the number to the total if it is equal to the next number
    digits.iter().zip(next_digits).fold(
        0u64,
        |acc, (&num, next)| {
            if num == next {
                acc + num as u64
            } else {
                acc
            }
        },
    )
}

pub fn part2(text: &str) -> usize {
    let half: usize = text.bytes()
        .zip(text.bytes().skip(text.len() / 2))
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| (a - b'0') as usize)
        .sum();

    2 * half
}

pub fn xavil(text: &str) -> usize {
    let digits = text.as_bytes();

    let half_point = digits.len() / 2;

    2
        * digits[..half_point]
            .iter()
            .zip(digits.iter().skip(half_point))
            .filter(|&(a, b)| a == b)
            .map(|(a, _)| (a - b'0') as usize)
            .sum::<usize>()
}

pub fn xavil2(text: &str) -> usize {
    let digits = text.as_bytes();
    let mut sum = 0;

    let half_point = digits.len() / 2;
    let first_half = &digits;
    let second_half = &digits[half_point..];

    for index in 0..half_point {
        let cur = unsafe { first_half.get_unchecked(index) };
        let next = unsafe { second_half.get_unchecked(index) };

        if cur == next {
            sum += (cur - b'0') as usize;
        }
    }

    2 * sum
}

pub fn xavil3(text: &str) -> usize {
    let digits = text.as_bytes();
    let half: usize = digits
        .iter()
        .zip(&digits[digits.len() / 2..])
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| (a - b'0') as usize)
        .sum();
    2 * half
}

pub fn xavil_loop(text: &str) -> usize {
    fn get_wrapped<T: Copy>(arr: &[T], index: usize) -> T {
        arr[index % arr.len()]
    }

    fn sum_repeated(digits: &[u8]) -> usize {
        let mut sum = 0;

        for index in 0..digits.len() {
            let cur = get_wrapped(digits, index);
            let next = get_wrapped(digits, index + digits.len() / 2);

            if cur == next {
                sum += (cur - b'0') as usize;
            }
        }

        sum
    }

    sum_repeated(text.as_bytes())
}

pub fn archer(s: &str) -> u32 {
    fn eval(u: u8) -> u8 {
        u - b'0'
    }

    let a = &s[..s.len() / 2];
    let b = &s[s.len() / 2..];

    let a = a.bytes().map(eval);
    let b = b.bytes().map(eval);

    let sum: u32 = a.zip(b)
        .filter(|pair| pair.0 == pair.1)
        .map(|pair| pair.0 as u32)
        .sum();

    sum * 2
}

pub fn archer_new(text: &str) -> usize {
    let s = text.as_bytes();
    let half: usize = s.iter()
        .zip(&s[s.len() / 2..])
        .filter_map(|(&a, &b)| {
            if a == b {
                Some((a - b'0') as usize)
            } else {
                None
            }
        })
        .sum();

    2 * half
}

pub fn star_wars(text: &str) -> u32 {
    let chars = text.chars().collect::<Vec<_>>();

    let len = chars.len();
    let mut sum = 0;

    for i in 0..len {
        let mut halfway_around = i + (len / 2);
        if halfway_around >= len {
            halfway_around -= len
        }
        if chars[i] == chars[halfway_around] {
            sum += chars[i].to_digit(10).unwrap();
        }
    }

    sum
}

pub const INPUT: &str = "6592822488931338589815525425236818285229555616392928433262436847386544514648645288129834834862363847542262953164877694234514375164927616649264122487182321437459646851966649732474925353281699895326824852555747127547527163197544539468632369858413232684269835288817735678173986264554586412678364433327621627496939956645283712453265255261565511586373551439198276373843771249563722914847255524452675842558622845416218195374459386785618255129831539984559644185369543662821311686162137672168266152494656448824719791398797359326412235723234585539515385352426579831251943911197862994974133738196775618715739412713224837531544346114877971977411275354168752719858889347588136787894798476123335894514342411742111135337286449968879251481449757294167363867119927811513529711239534914119292833111624483472466781475951494348516125474142532923858941279569675445694654355314925386833175795464912974865287564866767924677333599828829875283753669783176288899797691713766199641716546284841387455733132519649365113182432238477673375234793394595435816924453585513973119548841577126141962776649294322189695375451743747581241922657947182232454611837512564776273929815169367899818698892234618847815155578736875295629917247977658723868641411493551796998791839776335793682643551875947346347344695869874564432566956882395424267187552799458352121248147371938943799995158617871393289534789214852747976587432857675156884837634687257363975437535621197887877326295229195663235129213398178282549432599455965759999159247295857366485345759516622427833518837458236123723353817444545271644684925297477149298484753858863551357266259935298184325926848958828192317538375317946457985874965434486829387647425222952585293626473351211161684297351932771462665621764392833122236577353669215833721772482863775629244619639234636853267934895783891823877845198326665728659328729472456175285229681244974389248235457688922179237895954959228638193933854787917647154837695422429184757725387589969781672596568421191236374563718951738499591454571728641951699981615249635314789251239677393251756396";

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    // #[bench]
    // fn bench_mine(b: &mut Bencher) {
    //     b.iter(|| part2(INPUT));
    // }

    // #[bench]
    // fn bench_slikrick(b: &mut Bencher) {
    //     b.iter(|| slikrick(INPUT));
    // }

    // #[bench]
    // fn bench_restioson(b: &mut Bencher) {
    //     b.iter(|| restioson(INPUT));
    // }

    #[bench]
    fn bench_xavil(b: &mut Bencher) {
        b.iter(|| xavil(INPUT));
    }

    #[bench]
    fn bench_xavil2(b: &mut Bencher) {
        b.iter(|| xavil2(INPUT));
    }

    #[bench]
    fn bench_xavil3(b: &mut Bencher) {
        b.iter(|| xavil3(INPUT));
    }

    #[bench]
    fn bench_xavil_loop(b: &mut Bencher) {
        b.iter(|| xavil_loop(INPUT));
    }

    #[bench]
    fn bench_star_wars(b: &mut Bencher) {
        b.iter(|| star_wars(INPUT));
    }

    #[bench]
    fn bench_archer_new(b: &mut Bencher) {
        b.iter(|| archer_new(INPUT));
    }

    // #[bench]
    // fn bench_archer(b: &mut Bencher) {
    //     b.iter(|| archer(INPUT));
    // }
}
