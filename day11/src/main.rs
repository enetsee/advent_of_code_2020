use std::cmp::{Eq, PartialEq};
use std::io::{self, BufRead, BufReader};
use std::{fs::File, vec};
fn main() {
    let f = File::open("input.txt").unwrap();

    let plan = SeatPlan::load(f).unwrap();

    let plan_adj = fixpoint(|x| SeatPlan::step_adjacent(x), &plan);
    let occupied_adj = plan_adj.occupied_seats();

    let plan_vis = fixpoint(|x| SeatPlan::step_visible(x), &plan);
    let occupied_vis = plan_vis.occupied_seats();

    println!("adjacent: {}, visible: {}", occupied_adj, occupied_vis)
}

// -- SeatPlan implementation --------------------------------------------------

#[derive(Clone, Debug)]
struct SeatPlan {
    data: Vec<State>,
    width: usize,
    height: usize,
}

impl SeatPlan {
    pub fn step_visible(&self) -> Self {
        let mut data = vec![];
        let mut cur_row = 0;
        let mut cur_col = 0;
        while cur_row < self.height {
            while cur_col < self.width {
                data.push(self.step_seat_visible(cur_row, cur_col));
                cur_col = cur_col + 1;
            }
            cur_col = 0;
            cur_row = cur_row + 1;
        }
        SeatPlan {
            data,
            height: self.height,
            width: self.width,
        }
    }

    fn step_seat_visible(&self, row: usize, col: usize) -> State {
        match self.data[row * self.width + col] {
            State::Floor => (State::Floor),
            State::Occupied if self.visible_occupied(row, col) >= 5 => (State::Vacant),
            State::Occupied => (State::Occupied),
            State::Vacant if self.visible_occupied(row, col) > 0 => (State::Vacant),
            State::Vacant => (State::Occupied),
        }
    }

    pub fn step_adjacent(&self) -> Self {
        let mut data = vec![];
        let mut cur_row = 0;
        let mut cur_col = 0;
        while cur_row < self.height {
            while cur_col < self.width {
                data.push(self.step_seat_adjacent(cur_row, cur_col));
                cur_col = cur_col + 1;
            }
            cur_col = 0;
            cur_row = cur_row + 1;
        }
        SeatPlan {
            data,
            height: self.height,
            width: self.width,
        }
    }

    fn step_seat_adjacent(&self, row: usize, col: usize) -> State {
        match self.data[self.width * row + col] {
            State::Floor => (State::Floor),
            State::Occupied if self.adjacent_occupied(row, col) >= 4 => (State::Vacant),
            State::Occupied => (State::Occupied),
            State::Vacant if self.adjacent_occupied(row, col) > 0 => (State::Vacant),
            State::Vacant => (State::Occupied),
        }
    }

    fn adjacent_occupied(&self, row: usize, col: usize) -> usize {
        self.adjacent_seats(row, col)
            .iter()
            .fold(0, |ttl, s| if is_occupied(s) { ttl + 1 } else { ttl })
    }

    fn adjacent_seats(&self, row: usize, col: usize) -> Vec<State> {
        let pos = row * self.width + col;

        if row == 0 && col == 0 {
            vec![
                self.data[pos + 1],
                self.data[pos + self.width],
                self.data[pos + self.width + 1],
            ]
        } else if row == 0 && col + 1 == self.width {
            vec![
                self.data[pos - 1],
                self.data[pos + self.width],
                self.data[pos + self.width - 1],
            ]
        } else if row + 1 == self.height && col == 0 {
            vec![
                self.data[pos + 1],
                self.data[pos - self.width],
                self.data[pos - self.width + 1],
            ]
        } else if row + 1 == self.height && col + 1 == self.width {
            vec![
                self.data[pos - 1],
                self.data[pos - self.width],
                self.data[pos - self.width - 1],
            ]
        } else if row == 0 {
            vec![
                self.data[pos - 1],
                self.data[pos + 1],
                self.data[pos + self.width],
                self.data[pos + self.width - 1],
                self.data[pos + self.width + 1],
            ]
        } else if row + 1 == self.height {
            vec![
                self.data[pos - 1],
                self.data[pos + 1],
                self.data[pos - self.width],
                self.data[pos - self.width - 1],
                self.data[pos - self.width + 1],
            ]
        } else if col == 0 {
            vec![
                self.data[pos + 1],
                self.data[pos - self.width],
                self.data[pos - self.width + 1],
                self.data[pos + self.width],
                self.data[pos + self.width + 1],
            ]
        } else if col + 1 == self.width {
            vec![
                self.data[pos - 1],
                self.data[pos - self.width],
                self.data[pos - self.width - 1],
                self.data[pos + self.width],
                self.data[pos + self.width - 1],
            ]
        } else {
            vec![
                self.data[pos - 1],
                self.data[pos + 1],
                self.data[pos - self.width],
                self.data[pos - self.width - 1],
                self.data[pos - self.width + 1],
                self.data[pos + self.width],
                self.data[pos + self.width - 1],
                self.data[pos + self.width + 1],
            ]
        }
    }

    fn occupied_north(&self, row: usize, col: usize) -> bool {
        let mut occ = None;
        let mut cur_row = (row as i64) - 1;
        while cur_row >= 0 && occ.is_none() {
            match self.data[(cur_row as usize) * self.width + col] {
                State::Floor => cur_row = cur_row - 1,
                State::Vacant => occ = Some(false),
                State::Occupied => occ = Some(true),
            }
        }
        occ.unwrap_or(false)
    }

    fn occupied_west(&self, row: usize, col: usize) -> bool {
        let mut occ = None;
        let mut cur_col = (col as i64) - 1;
        while cur_col >= 0 && occ.is_none() {
            match self.data[row * self.width + (cur_col as usize)] {
                State::Floor => cur_col = cur_col - 1,
                State::Vacant => occ = Some(false),
                State::Occupied => occ = Some(true),
            }
        }
        occ.unwrap_or(false)
    }

    fn occupied_northwest(&self, row: usize, col: usize) -> bool {
        let mut occ = None;
        let mut cur_col = (col as i64) - 1;
        let mut cur_row = (row as i64) - 1;

        while cur_col >= 0 && cur_row >= 0 && occ.is_none() {
            match self.data[(cur_row as usize) * self.width + (cur_col as usize)] {
                State::Floor => {
                    cur_col = cur_col - 1;
                    cur_row = cur_row - 1;
                }
                State::Vacant => occ = Some(false),
                State::Occupied => occ = Some(true),
            }
        }
        occ.unwrap_or(false)
    }

    fn occupied_northeast(&self, row: usize, col: usize) -> bool {
        let mut occ = None;
        let mut cur_col = col + 1;
        let mut cur_row = (row as i64) - 1;

        while cur_col < self.width && cur_row >= 0 && occ.is_none() {
            match self.data[(cur_row as usize) * self.width + cur_col] {
                State::Floor => {
                    cur_col = cur_col + 1;
                    cur_row = cur_row - 1;
                }
                State::Vacant => occ = Some(false),
                State::Occupied => occ = Some(true),
            }
        }
        occ.unwrap_or(false)
    }

    fn occupied_southwest(&self, row: usize, col: usize) -> bool {
        let mut occ = None;
        let mut cur_col = (col as i64) - 1;
        let mut cur_row = row + 1;

        while cur_col >= 0 && cur_row < self.height && occ.is_none() {
            match self.data[cur_row * self.width + (cur_col as usize)] {
                State::Floor => {
                    cur_col = cur_col - 1;
                    cur_row = cur_row + 1;
                }
                State::Vacant => occ = Some(false),
                State::Occupied => occ = Some(true),
            }
        }
        occ.unwrap_or(false)
    }

    fn occupied_southeast(&self, row: usize, col: usize) -> bool {
        let mut occ = None;
        let mut cur_col = col + 1;
        let mut cur_row = row + 1;

        while cur_col < self.width && cur_row < self.height && occ.is_none() {
            match self.data[cur_row * self.width + cur_col] {
                State::Floor => {
                    cur_col = cur_col + 1;
                    cur_row = cur_row + 1;
                }
                State::Vacant => occ = Some(false),
                State::Occupied => occ = Some(true),
            }
        }
        occ.unwrap_or(false)
    }

    fn occupied_south(&self, row: usize, col: usize) -> bool {
        let mut occ = None;
        let mut cur_row = row + 1;

        while cur_row < self.height && occ.is_none() {
            match self.data[cur_row * self.width + col] {
                State::Floor => cur_row = cur_row + 1,

                State::Vacant => occ = Some(false),
                State::Occupied => occ = Some(true),
            }
        }
        occ.unwrap_or(false)
    }

    fn occupied_east(&self, row: usize, col: usize) -> bool {
        let mut occ = None;
        let mut cur_col = col + 1;
        while cur_col < self.width && occ.is_none() {
            match self.data[row * self.width + cur_col] {
                State::Floor => cur_col = cur_col + 1,

                State::Vacant => occ = Some(false),
                State::Occupied => occ = Some(true),
            }
        }
        occ.unwrap_or(false)
    }

    fn occupied_dir(&self, row: usize, col: usize, dir: &Direction) -> bool {
        match dir {
            Direction::N => self.occupied_north(row, col),
            Direction::S => self.occupied_south(row, col),
            Direction::E => self.occupied_east(row, col),
            Direction::W => self.occupied_west(row, col),
            Direction::NE => self.occupied_northeast(row, col),
            Direction::NW => self.occupied_northwest(row, col),
            Direction::SE => self.occupied_southeast(row, col),
            Direction::SW => self.occupied_southwest(row, col),
        }
    }

    fn visible_occupied(&self, row: usize, col: usize) -> usize {
        DIRS.iter().fold(0, |ttl, dir| {
            if self.occupied_dir(row, col, dir) {
                ttl + 1
            } else {
                ttl
            }
        })
    }

    pub fn occupied_seats(&self) -> usize {
        self.data
            .iter()
            .fold(0, |ttl, s| if is_occupied(s) { ttl + 1 } else { ttl })
    }

    pub fn load(f: File) -> io::Result<Self> {
        let mut width = 0;
        let mut height = 0;
        BufReader::new(f)
            .lines()
            .map(|res| {
                res.map(|str| {
                    width = str.len();
                    height = height + 1;
                    str.chars().map(|c| state_from_char(c)).collect()
                })
            })
            .collect::<io::Result<Vec<Vec<State>>>>()
            .map(|data| SeatPlan {
                data: data.concat(),
                width,
                height,
            })
    }
}

impl PartialEq for SeatPlan {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
            && self.width == other.width
            && self
                .data
                .iter()
                .zip(other.data.iter())
                .all(|(a, b)| a.eq(b))
    }
}
// -- Seat State ---------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    Floor,
    Occupied,
    Vacant,
}
fn is_occupied(s: &State) -> bool {
    match s {
        State::Occupied => true,
        _ => false,
    }
}
fn state_from_char(c: char) -> State {
    match c {
        '.' => State::Floor,
        'L' => State::Vacant,
        _ => State::Occupied,
    }
}

// -- Compass direction --------------------------------------------------------
enum Direction {
    N,
    E,
    S,
    W,
    NE,
    NW,
    SE,
    SW,
}
const DIRS: [Direction; 8] = [
    Direction::N,
    Direction::E,
    Direction::S,
    Direction::W,
    Direction::NE,
    Direction::NW,
    Direction::SE,
    Direction::SW,
];

// -- Generic fixpoint ---------------------------------------------------------

fn fixpoint<T>(f: fn(&T) -> T, a: &T) -> T
where
    T: PartialEq,
{
    let b = f(a);
    if a.eq(&b) {
        b
    } else {
        fixpoint(f, &b)
    }
}
