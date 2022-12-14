use std::fs;

#[derive(Debug)]
struct Grid {
    points: Vec<Vec<u8>>,
    x_size: usize,
    y_size: usize,
}

impl Grid {
    fn count_visible(&self) -> usize {
        let mut count = (2*self.x_size) + (2*(self.y_size - 2));
        for y in 1..self.y_size-1 {
            for x in 1..self.x_size-1 {
                if self.is_visible(x, y) {
                    count += 1;
                }
            }
        }
        count
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let mut visible = false;
        for direction in vec!["north", "east", "south", "west"] {
            if self.points[y][x] > self.max_height_in_direction(x, y, direction) {
                visible = true;
            }
        }
        visible
    }

    fn max_height_in_direction(&self, x: usize, y: usize, direction: &str) -> u8 {
        match direction {
            "north" => {
                let mut vector: Vec<u8> = vec![];
                for i in 0..y {
                    vector.push(self.points[i][x]);
                }
                vector.iter().max().unwrap().clone()
            },
            "east" => {
                let mut vector: Vec<u8> = vec![];
                for i in (x+1)..self.x_size {
                    vector.push(self.points[y][i]);
                }
                vector.iter().max().unwrap().clone()
            },
            "south" => {
                let mut vector: Vec<u8> = vec![];
                for i in (y+1)..self.y_size {
                    vector.push(self.points[i][x]);
                }
                vector.iter().max().unwrap().clone()
            },
            "west" => {
                let mut vector: Vec<u8> = vec![];
                for i in (0)..x {
                    vector.push(self.points[y][i]);
                }
                vector.iter().max().unwrap().clone()
            },
            _ => panic!("bad direction")
        }
    }

    fn score_in_direction(&self, x: usize, y: usize, direction: &str) -> u32 {
        match direction {
            "north" => {
                let mut vector: Vec<u8> = vec![];
                for i in 0..y {
                    vector.push(self.points[i][x]);
                }
                vector.reverse();
                Grid::scan_trees(self.points[y][x], vector)
            },
            "east" => {
                let mut vector: Vec<u8> = vec![];
                for i in (x+1)..self.x_size {
                    vector.push(self.points[y][i]);
                }
                Grid::scan_trees(self.points[y][x], vector)
            },
            "south" => {
                let mut vector: Vec<u8> = vec![];
                for i in (y+1)..self.y_size {
                    vector.push(self.points[i][x]);
                }
                Grid::scan_trees(self.points[y][x], vector)
            },
            "west" => {
                let mut vector: Vec<u8> = vec![];
                for i in (0)..x {
                    vector.push(self.points[y][i]);
                }
                vector.reverse();
                Grid::scan_trees(self.points[y][x], vector)
            },
            _ => panic!("bad direction")
        }
    }

    fn scan_trees(me: u8, trees: Vec<u8>) -> u32 {
        let mut score = 0;
        for tree in trees {
            if tree < me {
                score += 1;
            }
            else {
                score += 1;
                break;
            }
        }
        score
    }

    fn scenic_score(&self, x: usize, y: usize) -> u32 {
        let mut scores = vec![];
        for direction in vec!["north", "east", "south", "west"] {
            scores.push(self.score_in_direction(x,y, direction));
        }
        let mut score = 1;
        scores.iter().for_each(|s| score *= s);
        score
    }

    fn best_tree_score(&self) -> u32 {
        let mut scores = vec![];
        // disregard edge trees since these will never be best
        for y in 1..self.y_size-1 {
            for x in 1..self.x_size-1 {
                scores.push(self.scenic_score(x, y));
            }
        }
        scores.iter().max().unwrap().clone()
    }
}

fn main() {
    // let file_path = "input_sample.txt";
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("could not open file");
    let mut grid = Grid { points: vec![], x_size: 0, y_size: 0};
    for line in contents.lines() {
        let line_vector = line.as_bytes().iter().map(|b| b - 48).collect::<Vec<u8>>();
        grid.x_size = line_vector.len();
        grid.points.push(line_vector);
        grid.y_size += 1;
    }

    // dbg!(&grid);
    dbg!(grid.count_visible());
    dbg!(grid.best_tree_score());
}
