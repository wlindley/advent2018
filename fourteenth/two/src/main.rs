fn main() {
    let input = 323081;
    let digits = digits(37);
    let mut scoreboard = Scoreboard::new(digits, input, 6);
    let count = scoreboard.until();
    println!("Recipes before pattern: {}", count);
}

struct Scoreboard {
    pub scores: Vec<u8>,
    elves: Vec<usize>,
    target: u64,
    cur: u64,
    recipes: usize,
    target_size: u32,
}

impl Scoreboard {
    fn new(scores: Vec<u8>, target: u64, target_size: u32) -> Self {
        let elves = (0..2).collect();
        let recipes = scores.len();
        let mut cur = 0;
        for s in &scores {
            cur = (cur * 10) + *s as u64;
        }
        Self {
            scores,
            elves,
            target,
            cur,
            recipes,
            target_size,
        }
    }

    fn next(&mut self) -> Option<usize> {
        let sum = self
            .elves
            .iter()
            .fold(0, |sum, &elf| sum + self.scores[elf] as u64);
        let mut numbers = digits(sum);

        let limiter = 10u64.pow(self.target_size - 1);
        for digit in numbers {
            self.cur = ((self.cur % limiter) * 10) + digit as u64;
            self.scores.push(digit);
            self.recipes += 1;
            if self.cur == self.target {
                return Some(self.recipes - self.target_size as usize);
            }
        }

        for elf in &mut self.elves {
            *elf = (*elf + self.scores[*elf] as usize + 1) % self.scores.len();
        }
        None
    }

    fn recipes(&mut self, count: usize) {
        while self.recipes < count {
            self.next();
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

    fn until(&mut self) -> usize {
        loop {
            if let Some(recipes) = self.next() {
                return recipes;
            }
        }
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
        let mut scoreboard = Scoreboard::new(vec![3, 7], 1000, 4);
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
        let mut scoreboard = Scoreboard::new(vec![3, 7], 1000, 4);
        scoreboard.recipes(19);
        assert_eq!(5158916779, scoreboard.slice(9, 10));
    }

    #[test]
    fn test_examples() {
        let mut scoreboard = Scoreboard::new(vec![3, 7], 51589, 5);
        assert_eq!(9, scoreboard.until());

        let mut scoreboard = Scoreboard::new(vec![3, 7], 01245, 5);
        assert_eq!(5, scoreboard.until());

        let mut scoreboard = Scoreboard::new(vec![3, 7], 92510, 5);
        assert_eq!(18, scoreboard.until());

        let mut scoreboard = Scoreboard::new(vec![3, 7], 59414, 5);
        assert_eq!(2018, scoreboard.until());
    }
}
