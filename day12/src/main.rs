use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input.txt").unwrap();
    let alpha: RelState = Default::default();
    let omega = BufReader::new(f).lines().fold(alpha, |p, res| {
        let instr = Instr::parse(&res.unwrap()).unwrap();
        p.interpret(&instr)
    });
    let d = omega.pos.manhattan_dist(&alpha.pos);
    println!("{}", d)
}

#[derive(Debug, Clone, Copy)]
struct RelState {
    pos: Pos,
    waypoint: Pos,
}

impl RelState {
    fn interpret(&self, instr: &Instr) -> Self {
        match instr {
            Instr::Move(m, d) => self.interpret_move(m, d),
            Instr::Turn(dir, deg) => self.interpret_turn(dir, deg),
        }
    }

    fn interpret_turn(&self, dir: &Dir, deg: &usize) -> Self {
        let waypoint = self.waypoint.rotate(dir, deg);
        RelState {
            waypoint,
            pos: self.pos,
        }
    }

    fn interpret_move(&self, mov: &Move, mag: &usize) -> Self {
        match mov {
            Move::Relative => self.interpret_move_pos(mag),
            Move::Absolute(orient) => self.interpret_move_waypoint(orient, mag),
        }
    }

    fn interpret_move_pos(&self, mag: &usize) -> Self {
        let pos = self
            .pos
            .interpret_move_horiz(&self.waypoint.h.0, &(self.waypoint.h.1 * mag))
            .interpret_move_vert(&self.waypoint.v.0, &(self.waypoint.v.1 * mag));
        RelState {
            pos,
            waypoint: self.waypoint,
        }
    }

    fn interpret_move_waypoint(&self, orient: &Orientation, mag: &usize) -> Self {
        match orient {
            Orientation::H(horiz) => {
                let waypoint = self.waypoint.interpret_move_horiz(&horiz, mag);
                RelState {
                    waypoint,
                    pos: self.pos,
                }
            }
            Orientation::V(vert) => {
                let waypoint = self.waypoint.interpret_move_vert(&vert, mag);
                RelState {
                    waypoint,
                    pos: self.pos,
                }
            }
        }
    }
}
impl Default for RelState {
    fn default() -> Self {
        RelState {
            pos: Default::default(),
            waypoint: Pos {
                h: (Horiz::E, 10),
                v: (Vert::N, 1),
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    h: (Horiz, usize),
    v: (Vert, usize),
}

impl Default for Pos {
    fn default() -> Self {
        Pos {
            h: (Horiz::E, 0),
            v: (Vert::N, 0),
        }
    }
}

impl Pos {
    fn rotate_90(&self) -> Self {
        let h = match &self.v {
            (Vert::N, d) => (Horiz::E, *d),
            (Vert::S, d) => (Horiz::W, *d),
        };
        let v = match &self.h {
            (Horiz::E, d) => (Vert::S, *d),
            (Horiz::W, d) => (Vert::N, *d),
        };
        Pos { v, h }
    }

    fn rotate_270(&self) -> Self {
        let h = match &self.v {
            (Vert::N, d) => (Horiz::W, *d),
            (Vert::S, d) => (Horiz::E, *d),
        };
        let v = match &self.h {
            (Horiz::E, d) => (Vert::N, *d),
            (Horiz::W, d) => (Vert::S, *d),
        };
        Pos { v, h }
    }

    fn rotate_180(&self) -> Self {
        let h = match &self.h {
            (Horiz::W, d) => (Horiz::E, *d),
            (Horiz::E, d) => (Horiz::W, *d),
        };
        let v = match &self.v {
            (Vert::N, d) => (Vert::S, *d),
            (Vert::S, d) => (Vert::N, *d),
        };
        Pos { v, h }
    }

    fn rotate(&self, dir: &Dir, deg: &usize) -> Self {
        let delta = match dir {
            Dir::R => (*deg) % 360,
            Dir::L => 360 - (deg % 360),
        };
        match delta {
            0 | 360 => *self,
            90 => self.rotate_90(),
            180 => self.rotate_180(),
            270 => self.rotate_270(),
            _ => panic!("can't rotate"),
        }
    }
    fn manhattan_dist(&self, other: &Self) -> usize {
        self.horiz_dist(other) + self.vert_dist(other)
    }

    fn horiz_dist(&self, other: &Self) -> usize {
        match (&self.h.0, &other.h.0) {
            (Horiz::E, Horiz::E) if self.h.1 > other.h.1 => self.h.1 - other.h.1,
            (Horiz::E, Horiz::E) => other.h.1 - self.h.1,
            (Horiz::W, Horiz::W) if self.h.1 > other.h.1 => self.h.1 - other.h.1,
            (Horiz::W, Horiz::W) => other.h.1 - self.h.1,
            (Horiz::E, Horiz::W) | (Horiz::W, Horiz::E) => self.h.1 + other.h.1,
        }
    }

    fn vert_dist(&self, other: &Self) -> usize {
        match (&self.v.0, &other.v.0) {
            (Vert::N, Vert::N) if self.v.1 > other.v.1 => self.v.1 - other.v.1,
            (Vert::N, Vert::N) => other.v.1 - self.v.1,
            (Vert::S, Vert::S) if self.v.1 > other.v.1 => self.v.1 - other.v.1,
            (Vert::S, Vert::S) => other.v.1 - self.v.1,
            (Vert::N, Vert::S) | (Vert::S, Vert::N) => self.v.1 + other.v.1,
        }
    }

    fn interpret_move_vert(&self, vert: &Vert, n: &usize) -> Pos {
        let new_vert = match (self.v, vert) {
            ((Vert::N, m), Vert::N) => (Vert::N, n + m),
            ((Vert::N, m), Vert::S) if *n < m => (Vert::N, m - n),
            ((Vert::N, m), Vert::S) => (Vert::S, n - m),
            ((Vert::S, m), Vert::S) => (Vert::S, n + m),
            ((Vert::S, m), Vert::N) if *n < m => (Vert::S, m - n),
            ((Vert::S, m), Vert::N) => (Vert::N, n - m),
        };
        Pos {
            v: new_vert,
            h: self.h,
        }
    }

    fn interpret_move_horiz(&self, horiz: &Horiz, n: &usize) -> Pos {
        let new_horiz = match (self.h, horiz) {
            ((Horiz::E, m), Horiz::E) => (Horiz::E, n + m),
            ((Horiz::E, m), Horiz::W) if *n < m => (Horiz::E, m - n),
            ((Horiz::E, m), Horiz::W) => (Horiz::W, n - m),
            ((Horiz::W, m), Horiz::W) => (Horiz::W, n + m),
            ((Horiz::W, m), Horiz::E) if *n < m => (Horiz::W, m - n),
            ((Horiz::W, m), Horiz::E) => (Horiz::E, n - m),
        };
        Pos {
            h: new_horiz,
            v: self.v,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    pos: Pos,
    orient: Orientation,
}

impl Default for State {
    fn default() -> Self {
        State {
            pos: Default::default(),
            orient: Orientation::H(Horiz::E),
        }
    }
}

impl State {
    // -- Top level interpretter ---------------------------------------------------
    fn interpret(&self, instr: &Instr) -> State {
        match instr {
            Instr::Move(mvmt, n) => self.interpret_move(mvmt, n),
            Instr::Turn(dir, deg) => self.interpret_turn(dir, deg),
        }
    }

    fn interpret_turn(&self, dir: &Dir, deg: &usize) -> State {
        let delta = match dir {
            Dir::R => (*deg) % 360,
            Dir::L => 360 - (deg % 360),
        };
        let new_deg = (self.orient.to_deg() + delta) % 360;
        let new_orient = Orientation::from_deg(new_deg);
        State {
            orient: new_orient,
            pos: self.pos,
        }
    }

    fn interpret_move(&self, mvmt: &Move, n: &usize) -> State {
        match mvmt {
            Move::Relative => match &self.orient {
                Orientation::V(vert) => self.interpret_move_vert(&vert, n),
                Orientation::H(horiz) => self.interpret_move_horiz(&horiz, n),
            },
            Move::Absolute(Orientation::V(vert)) => self.interpret_move_vert(vert, n),
            Move::Absolute(Orientation::H(horiz)) => self.interpret_move_horiz(horiz, n),
        }
    }

    fn interpret_move_vert(&self, vert: &Vert, n: &usize) -> State {
        let new_pos = self.pos.interpret_move_vert(vert, n);
        State {
            pos: new_pos,
            orient: self.orient,
        }
    }

    fn interpret_move_horiz(&self, horiz: &Horiz, n: &usize) -> State {
        let new_pos = self.pos.interpret_move_horiz(horiz, n);
        State {
            pos: new_pos,
            orient: self.orient,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instr {
    Move(Move, usize),
    Turn(Dir, usize),
}

impl Instr {
    fn parse(str: &String) -> Option<Instr> {
        let (op, mag_str) = str.split_at(1);
        let mag = mag_str.parse().ok()?;
        match op {
            "L" => Some(Instr::Turn(Dir::L, mag)),
            "R" => Some(Instr::Turn(Dir::R, mag)),
            "F" => Some(Instr::Move(Move::Relative, mag)),
            "N" => Some(Instr::Move(Move::Absolute(Orientation::V(Vert::N)), mag)),
            "S" => Some(Instr::Move(Move::Absolute(Orientation::V(Vert::S)), mag)),
            "E" => Some(Instr::Move(Move::Absolute(Orientation::H(Horiz::E)), mag)),
            "W" => Some(Instr::Move(Move::Absolute(Orientation::H(Horiz::W)), mag)),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Move {
    Relative,
    Absolute(Orientation),
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Vert {
    N,
    S,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Horiz {
    E,
    W,
}
#[derive(Debug, Copy, Clone, PartialEq)]
enum Orientation {
    V(Vert),
    H(Horiz),
}

impl Orientation {
    fn to_deg(&self) -> usize {
        match self {
            Orientation::V(Vert::N) => 0,
            Orientation::V(Vert::S) => 180,
            Orientation::H(Horiz::E) => 90,
            Orientation::H(Horiz::W) => 270,
        }
    }

    fn from_deg(deg: usize) -> Self {
        match deg {
            0 => Orientation::V(Vert::N),
            180 => Orientation::V(Vert::S),
            90 => Orientation::H(Horiz::E),
            270 => Orientation::H(Horiz::W),
            _ => panic!("illegal degree orientation!"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Dir {
    L,
    R,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn interpret_move_vert() {
        let init: Pos = Default::default();

        let move_vert_n = init.interpret_move_vert(&Vert::N, &1);
        assert_eq!(move_vert_n.v, (Vert::N, 1));
        assert_eq!(move_vert_n.h, init.h);

        let move_vert_s = move_vert_n.interpret_move_vert(&Vert::S, &4);
        assert_eq!(move_vert_s.v, (Vert::S, 3));
        assert_eq!(move_vert_s.h, init.h);

        let move_vert_n2 = move_vert_s.interpret_move_vert(&Vert::N, &1);
        assert_eq!(move_vert_n2.v, (Vert::S, 2));
        assert_eq!(move_vert_n2.h, init.h);
    }

    #[test]
    fn interpret_move_horiz() {
        let init: Pos = Default::default();

        let move_vert_e = init.interpret_move_horiz(&Horiz::E, &1);
        assert_eq!(move_vert_e.h, (Horiz::E, 1));
        assert_eq!(move_vert_e.v, init.v);

        let move_vert_w = move_vert_e.interpret_move_horiz(&Horiz::W, &4);
        assert_eq!(move_vert_w.h, (Horiz::W, 3));
        assert_eq!(move_vert_w.v, init.v);

        let move_vert_e2 = move_vert_w.interpret_move_horiz(&Horiz::E, &1);
        assert_eq!(move_vert_e2.h, (Horiz::W, 2));
        assert_eq!(move_vert_e2.v, init.v);
    }

    #[test]
    fn interpret_move_forward() {
        let init_e = State {
            pos: Default::default(),
            orient: Orientation::H(Horiz::E),
        };
        let final_e = init_e.interpret_move(&Move::Relative, &4);
        assert_eq!(final_e.pos.h, (Horiz::E, 4));
        assert_eq!(final_e.pos.v, init_e.pos.v);

        let init_w = State {
            pos: Default::default(),
            orient: Orientation::H(Horiz::W),
        };
        let final_w = init_w.interpret_move(&Move::Relative, &4);
        assert_eq!(final_w.pos.h, (Horiz::W, 4));
        assert_eq!(final_w.pos.v, init_w.pos.v);
    }

    #[test]
    fn interpret_turn() {
        let init: State = Default::default();
        let w_l180 = init.interpret_turn(&Dir::L, &180);
        assert_eq!(w_l180.orient, Orientation::H(Horiz::W));

        let w_r180 = init.interpret_turn(&Dir::R, &180);
        assert_eq!(w_r180.orient, Orientation::H(Horiz::W));

        let e_l360 = init.interpret_turn(&Dir::L, &360);
        assert_eq!(e_l360.orient, Orientation::H(Horiz::E));

        let e_r360 = init.interpret_turn(&Dir::R, &360);
        assert_eq!(e_r360.orient, Orientation::H(Horiz::E));

        let n_l90 = init.interpret_turn(&Dir::L, &90);
        assert_eq!(n_l90.orient, Orientation::V(Vert::N));

        let n_r270 = init.interpret_turn(&Dir::R, &270);
        assert_eq!(n_r270.orient, Orientation::V(Vert::N));

        let s_l270 = init.interpret_turn(&Dir::L, &270);
        assert_eq!(s_l270.orient, Orientation::V(Vert::S));

        let s_r90 = init.interpret_turn(&Dir::R, &90);
        assert_eq!(s_r90.orient, Orientation::V(Vert::S));
    }
}
