fn main() {
    let input = 323081;
    let digits = digits(37);
    let mut scoreboard = Scoreboard::new(digits);
    let scores = scoreboard.ten_after(input);
    println!("Selected scores: {}", scores);
}

struct Scoreboard {
    pub scores: Vec<u8>,
    elves: Vec<usize>,
}

impl Scoreboard {
    fn new(scores: Vec<u8>) -> Self {
        let elves = (0..2).collect();
        Self { scores, elves }
    }

    fn next(&mut self) -> usize {
        let sum = self
            .elves
            .iter()
            .fold(0, |sum, &elf| sum + self.scores[elf] as u64);
        let mut digits = digits(sum);
        let num_digits = digits.len();
        self.scores.append(&mut digits);
        for elf in &mut self.elves {
            *elf = (*elf + self.scores[*elf] as usize + 1) % self.scores.len();
        }
        num_digits
    }

    fn recipes(&mut self, count: usize) {
        let mut count = count as i32;
        while count > 0 {
            count -= self.next() as i32;
        }
    }

    fn slice(&self, begin: usize, count: usize) -> u64 {
        let digits = &self.scores[begin..begin + count];
        let mut total = 0;
        for digit in digits {
            total *= 10;
            total += *digit as u64;
        }
        return total;
    }

    fn ten_after(&mut self, begin: usize) -> u64 {
        self.recipes(begin + 9);
        self.slice(begin, 10)
    }
}

fn digits(mut value: u64) -> Vec<u8> {
    let mut digits = Vec::new();
    if value == 0 {
        digits.push(0);
    }
    while value > 0 {
        digits.insert(0, (value % 10) as u8);
        value /= 10;
    }
    return digits;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut scoreboard = Scoreboard::new(vec![3, 7]);
        scoreboard.next();
        assert_eq!(vec![3, 7, 1, 0], scoreboard.scores);
        scoreboard.next();
        assert_eq!(vec![3, 7, 1, 0, 1, 0], scoreboard.scores);
        scoreboard.next();
        assert_eq!(vec![3, 7, 1, 0, 1, 0, 1], scoreboard.scores);
        scoreboard.next();
        assert_eq!(vec![3, 7, 1, 0, 1, 0, 1, 2], scoreboard.scores);
    }

    #[test]
    fn test_digits() {
        assert_eq!(vec![0], digits(0));
        assert_eq!(vec![3], digits(3));
        assert_eq!(vec![1, 5], digits(15));
        assert_eq!(vec![2, 4, 7], digits(247));
    }

    #[test]
    fn test_slice() {
        let mut scoreboard = Scoreboard::new(vec![3, 7]);
        scoreboard.recipes(19);
        assert_eq!(5158916779, scoreboard.slice(9, 10));
    }

    #[test]
    fn test_examples() {
        let mut scoreboard = Scoreboard::new(vec![3, 7]);
        assert_eq!(5158916779, scoreboard.ten_after(9));
        assert_eq!(
            vec![3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7, 9, 2],
            scoreboard.scores
        );

        let mut scoreboard = Scoreboard::new(vec![3, 7]);
        assert_eq!(0124515891, scoreboard.ten_after(5));

        let mut scoreboard = Scoreboard::new(vec![3, 7]);
        assert_eq!(9251071085, scoreboard.ten_after(18));

        let mut scoreboard = Scoreboard::new(vec![3, 7]);
        assert_eq!(5941429882, scoreboard.ten_after(2018));
    }
}
