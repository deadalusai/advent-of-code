extern crate util;

enum SingleResult<T> { Single(T), Empty, Many }

fn single<T, I: Iterator<Item = T>>(mut iter: I) -> SingleResult<T> {
    match (iter.next(), iter.next()) {
        (Some(_), Some(_))   => SingleResult::Many,
        (Some(result), None) => SingleResult::Single(result),
        _                    => SingleResult::Empty,
    }
}

type X = usize;
type Y = usize;

#[derive(Copy, Clone)]
enum Tile {
    Space,
    Backslash,
    Forwardslash,
    Horizontal,
    Vertical,
    Cross,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            ' ' => Tile::Space,
            '\\' => Tile::Backslash,
            '/' => Tile::Forwardslash,
            '-' | '>' | '<' => Tile::Horizontal,
            '|' | 'v' | '^' => Tile::Vertical,
            '+' => Tile::Cross,
            ___ => panic!("Unexpected character: {}", c),
        }
    }

    fn as_char(&self) -> char {
        match self {
            Tile::Space => ' ',
            Tile::Backslash => '\\',
            Tile::Forwardslash => '/',
            Tile::Horizontal => '-',
            Tile::Vertical => '|',
            Tile::Cross => '+',
        }
    } 
}

struct Grid {
    data: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            data: vec![Tile::Space; width * height],
            width: width,
            height: height,
        }
    }

    fn get(&self, (x, y): (X, Y)) -> Tile {
        let pos = (y * self.width) + x;
        self.data[pos]
    }

    fn get_mut(&mut self, (x, y): (X, Y)) -> &mut Tile {
        let pos = (y * self.width) + x;
        &mut self.data[pos]
    }

    fn print(&self) -> String {
        let mut buf = String::with_capacity(self.data.len());
        for row in self.data.chunks(self.width) {
            for tile in row.iter() {
                buf.push(tile.as_char());
            }
            buf.push('\n');
        }
        buf
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum Turn {
    GoLeft,
    GoStraight,
    GoRight,
}

struct Cart {
    direction: Direction,
    next_turn: Turn,
    pos: (X, Y),
    destroyed: bool,
}

fn main() {

    let input = util::read_input("input.txt").unwrap();

    let width = input.iter().map(String::len).max().unwrap();
    let height = input.len() as usize;

    type CartId = usize;

    let mut grid = Grid::new(width, height);
    let mut carts = Vec::new();

    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            *grid.get_mut((x, y)) = Tile::from_char(c);
            let direction = match c {
                '^' => Some(Direction::Up),
                'v' => Some(Direction::Down),
                '<' => Some(Direction::Left),
                '>' => Some(Direction::Right),
                _   => None,
            };
            if let Some(direction) = direction {
                carts.push(Cart {
                    direction: direction,
                    next_turn: Turn::GoLeft,
                    pos: (x, y),
                    destroyed: false,
                });
            }
        }
    }

    println!("{}", grid.print());

    fn test_for_collision(cart_id: CartId, carts: &[Cart]) -> Option<CartId> {
        for (other_cart_id, other_cart) in carts.iter().enumerate() {
            if other_cart_id == cart_id || other_cart.destroyed {
                continue;
            }
            if carts[cart_id].pos == other_cart.pos {
                return Some(other_cart_id);
            }
        }
        None
    }

    fn tick_cart(grid: &Grid, cart: &mut Cart) {
        use Tile::*;
        use Turn::*;
        use Direction::*;
        // Update cart position
        cart.pos = match (cart.direction, cart.pos) {
            (Up,    (x, y)) => (x, y - 1),
            (Down,  (x, y)) => (x, y + 1),
            (Left,  (x, y)) => (x - 1, y),
            (Right, (x, y)) => (x + 1, y),
        };
        // Determine new cart direction
        let current_tile = grid.get(cart.pos);
        let move_direction = match (cart.direction, current_tile) {
            (_,         Space)        => panic!("Derailed! {:?}", cart.pos),
            (Up,        Backslash)    => Left,
            (Down,      Backslash)    => Right,
            (Left,      Backslash)    => Up,
            (Right,     Backslash)    => Down,
            (Up,        Forwardslash) => Right,
            (Down,      Forwardslash) => Left,
            (Left,      Forwardslash) => Down,
            (Right,     Forwardslash) => Up,
            (direction, Vertical)     => direction,
            (direction, Horizontal)   => direction,
            (direction, Cross) => {
                match (direction, cart.next_turn) {
                    (Up,    GoLeft)     => Left,
                    (Down,  GoLeft)     => Right,
                    (Left,  GoLeft)     => Down,
                    (Right, GoLeft)     => Up,
                    (dir,   GoStraight) => dir,
                    (Up,    GoRight)    => Right,
                    (Down,  GoRight)    => Left,
                    (Left,  GoRight)    => Up,
                    (Right, GoRight)    => Down,
                }
            },
            (direction, tile) => panic!("Unhandled direction {:?} and tile {:?} at {:?}", direction, tile.as_char(), cart.pos),
        };
        // Update cart
        cart.direction = move_direction;
        if let Cross = current_tile {
            cart.next_turn = match cart.next_turn {
                GoLeft     => GoStraight,
                GoStraight => GoRight,
                GoRight    => GoLeft,
            };
        }
    }

    // Runs the simulation, printing when a collision is detected.
    // NOTE: A collision may occur mid-tick!
    fn tick(grid: &Grid, carts: &mut [Cart]) {
        // Since we have to scan this collection *every time* we modify one of its elements
        // we can't iterate directly 😢
        for cart_id in 0..carts.len() {
            // Oh non-lexical lifetimes - where art thou?
            {
                let cart = &mut carts[cart_id];
                if cart.destroyed {
                    continue;
                }
                tick_cart(grid, cart);
            }
            // Check for collisions with this cart
            if let Some(colliding_cart_id) = test_for_collision(cart_id, carts) {
                println!("Collision at {:?} between {} and {}", carts[colliding_cart_id].pos, cart_id, colliding_cart_id);

                // And destroy the colliding carts
                carts[colliding_cart_id].destroyed = true;
                carts[cart_id].destroyed = true;
            }
        }
    }

    loop
    {
        tick(&mut grid, &mut carts);
        
        // Check for a single remaining cart at the end of the tick
        // Rust, why no built-in `single` method?
        match single(carts.iter().filter(|c| !c.destroyed)) {
            SingleResult::Single(cart) => {
                println!("Final remaining cart is at position {:?}", cart.pos);
                break;
            }
            SingleResult::Empty => {
                println!("All out of carts!");
                break;    
            }
            SingleResult::Many => ()
        }
    }
}