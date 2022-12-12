// 12-12-2022
// "extern crate pancurses", "extern crate" is no longer needed since Rust 2018 because Cargo knows what dependencies to load.
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use pancurses::{ initscr, resize_term, start_color, endwin, Input, noecho, init_pair, COLOR_PAIR, Window, set_title };
use pancurses::{ COLOR_GREEN, COLOR_WHITE, COLOR_BLACK };

static MAN_C: char = '☺';
static TREE_C: char = 'φ';

struct Tree {
    x: i32,
    y: i32
}

struct Program {
    window: Window,
    rng: ThreadRng,

    score: i32,

    // the lumberjack
    man_x: i32,
    man_y: i32,

    trees: Vec<Tree>
}

impl Program {
    fn new() -> Program {
        Program {
            window: initscr(),
            trees: Vec::new(),
            score: 0,
            man_x: 40,
            man_y: 12,
            rng: thread_rng()
        }
    }

    // fn load_audio(&mut self) {

    // }

    fn init(&mut self) {
        set_title("Lumberjack - by Hevanafa (Dec 2022)");

        // load_audio();

        resize_term(25, 80);

        start_color();
        init_pair(1, COLOR_WHITE, COLOR_BLACK); // man
        init_pair(2, COLOR_GREEN, COLOR_BLACK); // trees
    
        self.window.keypad(true);
        self.window.clear();
        noecho();

        // fill the field with 20 trees
        for _ in 0..20 {
            let (x, y) = (self.rng.gen_range(0..80), self.rng.gen_range(0..25));
            self.trees.push(Tree { x, y });
        }
    }

    fn draw(&self) {
        self.window.clear();

        self.window.attron(COLOR_PAIR(2));
        for tree in self.trees.iter() {
            self.window.mv(tree.y, tree.x);
            self.window.addch(TREE_C);
        }
        self.window.attroff(COLOR_PAIR(2));
    
        self.window.attroff(COLOR_PAIR(1));
        self.window.mv(self.man_y, self.man_x);
        self.window.addch(MAN_C);
        self.window.attroff(COLOR_PAIR(1));

        self.window.mv(0, 0);
        self.window.printw(format!("Score: {}", self.score));
    
        self.window.mv(24, 0);
        self.window.printw("Press q to quit.");
    
        self.window.refresh();
    }

    // returns true if there's a tree
    fn check_tree(&mut self, delta_x: i32, delta_y: i32) -> bool {
        if self.trees.iter().any(|tree|
            tree.x == self.man_x + delta_x &&
            tree.y == self.man_y + delta_y
        ) {
            // let tree = self.trees.iter().find(|tree| tree.x == self.man_x + inc);
            let idx = self.trees.iter().position(|tree|
                tree.x == self.man_x + delta_x &&
                tree.y == self.man_y + delta_y
            ).unwrap();
            self.trees.remove(idx);

            self.score += 1;

            return true;
        }

        false
    }

    fn step_x(&mut self, inc: i32) {
        // check tree hit
        if self.check_tree(inc, 0) { return; }

        self.man_x += inc;

        if self.man_x < 0  { self.man_x = 0; }
        if self.man_x > 79 { self.man_x = 79; }
    }

    fn step_y(&mut self, inc: i32) {
        // check tree hit
        if self.check_tree(0, inc) { return; }

        self.man_y += inc;

        if self.man_y < 0  { self.man_y = 0; }
        if self.man_y > 24 { self.man_y = 24; }
    }

    fn update(&self) {

    }
}

fn main() {
    let mut p = Program::new();
    p.init();

    p.draw();

    loop {
        let input = p.window.getch();

        match input {
            Some(Input::Character('q')) => break,

            Some(Input::KeyUp) => p.step_y(-1), // p.man_y -= 1,
            Some(Input::KeyDown) => p.step_y(1), // p.man_y += 1,
            Some(Input::KeyLeft) => p.step_x(-1), // p.man_x -= 1,
            Some(Input::KeyRight) => p.step_x(1), // p.man_x += 1,

            None => (),
            _ => {
                p.window.mv(0, 0);
                p.window.printw(format!("{:?}", input.unwrap()));
            }
        }

        p.draw();
        p.update();
    }

    endwin();
}
