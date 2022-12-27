use std::collections::VecDeque;
use std::fs;

#[derive(Debug,PartialEq,Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    points: Vec<Vec<u8>>,
    x_size: usize,
    y_size: usize,
    start: Point,
    end : Point,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            points: vec![],
            x_size: 0,
            y_size: 0,
            start: Point {x: 0, y: 0},
            end: Point {x: 0, y: 0},
        }
    }

    fn search(&self) ->  Option<usize> {
        let mut xq: VecDeque<usize> = VecDeque::new();
        let mut yq: VecDeque<usize> = VecDeque::new();
        let mut visited = vec![vec![false; self.x_size]; self.y_size];
        let mut reached_end = false;
        let mut nodes_left_in_layer: usize = 1;
        let mut nodes_in_next_layer: usize = 0;
        let mut move_count: usize = 0;
        xq.push_back(self.start.x);
        yq.push_back(self.start.y);
        visited[self.start.y][self.start.x] = true;
        while xq.len() > 0 {
            let x = xq.pop_front().unwrap();
            let y = yq.pop_front().unwrap();
            let point = Point { x, y };
            if point == self.end {
                reached_end = true;
                break;
            }
            self.explore_neighbors(point, &mut xq, &mut yq, &mut visited, &mut nodes_in_next_layer);
            nodes_left_in_layer -= 1;
            if nodes_left_in_layer == 0 {
                nodes_left_in_layer = nodes_in_next_layer;
                nodes_in_next_layer = 0;
                move_count += 1;
            }
        }
        if reached_end {
            Some(move_count)
        } else {
            None
        }
    }

    fn explore_neighbors(&self, point: Point, xq: &mut VecDeque<usize>, yq: &mut VecDeque<usize>,
                         visited: &mut Vec<Vec<bool>>, nodes_in_next_layer: &mut usize) {
        // North, South, East, West
        let dy = vec![-1, 1, 0, 0];
        let dx = vec![0, 0, 1, -1];

        for i in 0..4 {
            let x: isize = point.x as isize + dx[i];
            let y: isize = point.y as isize + dy[i];
            if x < 0 || y < 0 || x as usize >= self.x_size || y as usize >= self.y_size {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            if visited[y][x] {
                continue;
            }
            let new_elevation = self.points[y][x];
            let current_elevation = self.points[point.y][point.x];
            if new_elevation as isize - current_elevation as isize > 1 {
                continue;
            }
            xq.push_back(x);
            yq.push_back(y);
            visited[y][x] = true;
            *nodes_in_next_layer += 1;
        }
    }

    fn prepare_for_search(&mut self) {
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                if self.points[y][x] == "S".as_bytes()[0] {
                    self.start = Point {x,y};
                    self.points[y][x] = "a".as_bytes()[0];
                }
                if self.points[y][x] == "E".as_bytes()[0] {
                    self.end = Point {x,y};
                    self.points[y][x] = "z".as_bytes()[0];
                }
            }
        }
    }

    fn all_points_with_character(&self, character: &str) -> Vec<Point> {
        let mut all_points = vec![];
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                if self.points[y][x] == character.as_bytes()[0] {
                    all_points.push(Point {x,y});
                }
            }
        }
        all_points
    }
}

fn main() {
    // let file_path = "input_sample.txt";
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("could not open file");
    let mut grid = Grid::new();
    for line in contents.lines() {
        let line_vector = line.as_bytes().iter().map(|b| b.clone()).collect::<Vec<u8>>();
        grid.x_size = line_vector.len();
        grid.points.push(line_vector);
        grid.y_size += 1;
    }
    grid.prepare_for_search();
    // dbg!(&grid);
    let path_len = grid.search();
    // answer for part1
    dbg!(path_len);

    // part 2 ...
    let mut all_path_lens = vec![];
    let all_lowest_elevation = grid.all_points_with_character("a");
    for point in all_lowest_elevation {
        grid.start = point;
        if let Some(len) = grid.search() {
            all_path_lens.push(len);
        }
    }
    dbg!(&all_path_lens);
    dbg!(all_path_lens.iter().min());
}
