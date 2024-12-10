use itertools::Itertools;

#[derive(Debug)]
pub struct Map {
    pub heights: Vec<Vec<u32>>,
    pub width: usize,
    pub length: usize,
}

impl std::convert::From<&str> for Map {
    fn from(s: &str) -> Map {
        let width = s.find('\n').expect("Invalid input file");
        let mut heights = vec![vec![]; s.find('\n').unwrap()];
        s.lines().for_each(|l| {
            l.chars()
                .enumerate()
                .for_each(|(i, c)| heights[i].push(c.to_digit(10).unwrap()))
        });

        Map {
            width,
            length: heights.len(),
            heights,
        }
    }
}

impl Map {
    // I call this solution "I forgot how to implement A* and I'm too lazy to go look it up"
    pub fn get_score_from_trailhead(&self, x: usize, y: usize) -> usize {
        self.search(x, y, 0).iter().unique().count()
    }

    pub fn get_rating_from_trailhead(&self, x: usize, y: usize) -> usize {
        self.search(x, y, 0).iter().count()
    }

    fn search(&self, x: usize, y: usize, height: u32) -> Vec<(usize, usize)> {
        if height == 9 {
            vec![(x, y)]
        } else {
            let mut score = Vec::new();

            for (i, j) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let to_x = x as i32 + i;
                let to_y = y as i32 + j;
                if to_x >= 0
                    && to_x < self.width as i32
                    && to_y >= 0
                    && to_y < self.length as i32
                    && self.heights[to_x as usize][to_y as usize] == height + 1
                {
                    score.extend(self.search(to_x as usize, to_y as usize, height + 1));
                }
            }

            score
        }
    }
}
