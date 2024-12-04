#[derive(Debug)]
struct Puzzle {
    grid: Vec<Vec<char>>,
    hbound: i64,
    vbound: i64,
}

type Position = (usize, usize);

#[derive(Clone, Debug)]
enum SearchDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

struct BrokenDownWord {
    fulcrum: char,
    part_length: usize,
}

impl SearchDirection {
    fn step(&self, (x, y): Position) -> Position {
        match self {
            Self::North => (x, y - 1),
            Self::NorthEast => (x + 1, y - 1),
            Self::East => (x + 1, y),
            Self::SouthEast => (x + 1, y + 1),
            Self::South => (x, y + 1),
            Self::SouthWest => (x - 1, y + 1),
            Self::West => (x - 1, y),
            Self::NorthWest => (x - 1, y - 1),
        }
    }
}

impl Puzzle {
    fn load(s: &str) -> Result<Self, &'static str> {
        let lines: Vec<Vec<char>> = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect())
            .collect();
        if lines.is_empty() {
            return Err("Puzzle cannot be empty");
        }
        let expected_len = lines[0].len();
        let hbound: i64 = (expected_len - 1).try_into().unwrap();
        let vbound: i64 = (lines.len() - 1).try_into().unwrap();
        if lines.iter().all(|row| row.len() == expected_len) {
            Ok(Puzzle {
                grid: lines,
                hbound,
                vbound,
            })
        } else {
            Err("All rows must have the same length")
        }
    }

    fn get(&self, (x, y): Position) -> char {
        self.grid[y][x]
    }

    fn search_position(&self, word: &str, position: Position, direction: &SearchDirection) -> bool {
        word.chars()
            .enumerate()
            .try_fold(position, |pos, (i, char)| {
                if self.get(pos) == char {
                    if i < word.len() - 1 {
                        Some(direction.step(pos))
                    } else {
                        Some(pos)
                    }
                } else {
                    None
                }
            })
            .is_some()
    }

    fn is_valid_direction(
        &self,
        word: &str,
        (x, y): Position,
        direction: &SearchDirection,
    ) -> bool {
        let len = (word.len() - 1) as i64;
        let (x, y) = (x as i64, y as i64);
        match direction {
            SearchDirection::North => y - len >= 0,
            SearchDirection::NorthEast => (x + len <= self.hbound) && (y - len >= 0),
            SearchDirection::East => x + len <= self.hbound,
            SearchDirection::SouthEast => (x + len <= self.hbound) && (y + len <= self.vbound),
            SearchDirection::South => y + len <= self.vbound,
            SearchDirection::SouthWest => (x - len >= 0) && (y + len <= self.vbound),
            SearchDirection::West => x - len >= 0,
            SearchDirection::NorthWest => (x - len >= 0) && (y - len >= 0),
        }
    }

    fn search(&self, word: &str) -> i32 {
        static DIRECTIONS: [SearchDirection; 8] = [
            SearchDirection::North,
            SearchDirection::NorthEast,
            SearchDirection::East,
            SearchDirection::SouthEast,
            SearchDirection::South,
            SearchDirection::SouthWest,
            SearchDirection::West,
            SearchDirection::NorthWest,
        ];

        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().flat_map(move |(x, _)| {
                    DIRECTIONS
                        .iter()
                        .filter(move |&dir| self.is_valid_direction(word, (x, y), dir))
                        .filter(move |&dir| self.search_position(word, (x, y), dir))
                })
            })
            .count() as i32
    }

    fn xsearch(&self, word: &str) -> i32 {
        let broken_word = break_down_word(word).expect("Word should of odd length, > 0");
        let part_len = broken_word.part_length;
        let mut count = 0;
        for x in part_len..self.hbound as usize - part_len + 1 {
            for y in part_len..self.vbound as usize - part_len + 1 {
                if self.get((x, y)) == broken_word.fulcrum {
                    let positions = [
                        ((x - part_len, y - part_len), SearchDirection::SouthEast),
                        ((x + part_len, y - part_len), SearchDirection::SouthWest),
                        ((x - part_len, y + part_len), SearchDirection::NorthEast),
                        ((x + part_len, y + part_len), SearchDirection::NorthWest),
                    ];

                    let validcount = positions
                        .iter()
                        .filter(|&&(pos, ref dir)| self.search_position(word, pos, dir))
                        .count();
                    if validcount == 2 {
                        count += 1
                    };
                }
            }
        }

        count
    }
}

fn break_down_word(s: &str) -> Option<BrokenDownWord> {
    let length = s.chars().count();
    if length == 0 || length % 2 == 0 {
        return None;
    }

    let mid = length / 2;
    let fulcrum = s.chars().nth(mid)?;
    let part_length = mid;

    Some(BrokenDownWord {
        fulcrum,
        part_length,
    })
}

fn main() -> Result<(), &'static str> {
    let file_content =
        std::fs::read_to_string("src/input.txt").expect("expected file at src/input.txt");
    let puzzle = Puzzle::load(&file_content)?;

    let acount = puzzle.search("XMAS");
    println!("Part a: {}", acount);

    let bcount = puzzle.xsearch("MAS");
    println!("Part b: {}", bcount);
    Ok(())
}
