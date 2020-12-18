use core::ops::Add;

const SIZE_X: i32 = 200;
const SIZE_Y: i32 = 200;

/*===============
=     Instr     =
================*/
#[derive(Debug)]
pub enum Instr {
    Left,
    Right,
    Forward,
    Noop,
}

/*===============
=     Dir       =
================*/
#[derive(Debug, Copy, Clone)]
pub enum Dir {
    North,
    East,
    South,
    West,
}


impl Dir {
    fn from_i8(v: i8) -> Dir {
        let rem = v.rem_euclid(4);
        match rem {
            0 => Dir::North,
            1 => Dir::East,
            2 => Dir::South,
            _ => Dir::West,
        }       
    }
}

/*===============
=     Point     =
================*/
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn is_out_of_bounds(&self) -> bool {
        !(
               0 <= self.x && self.x < SIZE_X
            && 0 <= self.y && self.y < SIZE_Y
        )
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Self;
    fn add(self, b: (i32, i32)) -> Self{
        Self{
            x: self.x + b.0,
            y: self.y + b.1
        }
    }
}

/*===============
=     Rover     =
================*/
#[derive(Debug)]
pub struct Rover {
    pos: Point,
    facing: Dir
}

impl Rover {
    fn move_forward(&mut self) -> Result<(), Point>{
        let d = match &self.facing {
            Dir::North   => ( 0,  1),
            Dir::East    => ( 1,  0),
            Dir::South   => ( 0, -1),
            Dir::West    => (-1,  0),
        };
        let next_p = self.pos+d;
        if next_p.is_out_of_bounds() {
            return Err(next_p);
        }
        self.pos = next_p;
        Ok(())
    }

    fn rotate(&mut self, d: i8) {

        // facing = dir[+-1 % 4]
        self.facing = Dir::from_i8(
            ((self.facing as i8) + d).rem_euclid(4)
        );
    }

    fn rotate_left(&mut self) -> Result<(), Point>{
        self.rotate(-1);
        Ok(())
    }

    fn rotate_right(&mut self) -> Result<(), Point>{
        self.rotate(1);
        Ok(())
    }

    fn act_if_possible(&mut self, instr: &Instr) -> Result<(), Point>{
        match instr {
            Instr::Left    => self.rotate_left(),
            Instr::Right   => self.rotate_right(),
            Instr::Forward => self.move_forward(),
            Instr::Noop    => Ok(())
        }
    }
}

/*================
= Parse input    =
=================*/
pub fn parse_input(input: &str) -> Option<(Rover, Vec<Instr>)> {
    let mut lines = input.trim().split('\n');
    // Parse starting position
    let mut pos_iter = lines.next()?
        .split(',')
        .map(|x| x.trim().parse::<i32>().unwrap_or(0));
    let pos = Point{
        x: pos_iter.next()?,
        y: pos_iter.next()?,
    };
    // Parse starting direction
    let dir_str = lines.next()?;
    let facing = match dir_str.chars().next()? {
        'N' => Dir::North,
        'E' => Dir::East,
        'W' => Dir::West,
        'S' => Dir::South,
        _   => return None
    };
    // Parse commands
    let commands = lines.next()?.chars().map(|c|
        match c {
            'R' => Instr::Right,
            'L' => Instr::Left,
            'F' => Instr::Forward,
            _   => Instr::Noop,
        }
    ).collect::<Vec<Instr>>();

    Some( (Rover{pos, facing}, commands) )
}


/*================
= Parse input    =
=================*/
pub fn solve(input: &mut (Rover, Vec<Instr>)) -> Result<Point, Point> {
    let (rover, instrs) = input;
    //println!("Status {:#?}", rover );
    for instr in instrs.iter() {
        //println!("Acting {:?}", instr);
        if let Err(p) = rover.act_if_possible(instr) {
            return Err(p)
        };
        //println!("Status {:?}", rover );
    }
    Ok(rover.pos)
}

/*================
=      Tests     =
=================*/
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn solve_basic_test() {
        let input = String::from("
10, 10
N
F
");
        let mut input = parse_input(&input).unwrap();
        let p = solve(&mut input);
        assert_eq!(Ok(Point{x: 10, y: 11}), p);
    }

    #[test]
    fn solve_longer_test() {
        let input = String::from("
10, 10
N
FFFFLFFFFRFFFFRFFFFRFFFF
");
        let mut input = parse_input(&input).unwrap();
        let p = solve(&mut input);
        assert_eq!(Ok(Point{x: 10, y: 14}), p);
    }

    #[test]
    fn solve_basic_collision() {
        let input = String::from("
0, 0
S
F
");
        let mut input = parse_input(&input).unwrap();
        let p = solve(&mut input);
        assert_eq!(Err(Point{x: 0, y: -1}), p);
    }

    #[test]
    fn solve_longer_collision() {
        let input = String::from("
0, 0
N
FFFFRFFFFRFFFFF
");
        let mut input = parse_input(&input).unwrap();
        let p = solve(&mut input);
        assert_eq!(Err(Point{x: 4, y: -1}), p);
    }

    #[test]
    fn solve_critical_points() {
        let mut input = String::from("
0, 0
N
");
        input.push_str(&"F".repeat(199));
        input.push_str("R");
        input.push_str(&"F".repeat(199));
        input.push_str("R");
        input.push_str(&"F".repeat(199));
        input.push_str("R");
        input.push_str(&"F".repeat(199));
        let mut input = parse_input(&input).unwrap();
        let p = solve(&mut input);
        assert_eq!(Ok(Point{x: 0, y: 0}), p);
    }

}