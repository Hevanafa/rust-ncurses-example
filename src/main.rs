use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
extern crate pancurses;
use pancurses::{ initscr, start_color, endwin, Input, noecho, init_pair, COLOR_PAIR, Window };
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
            man_x: 40,
            man_y: 12,
            rng: thread_rng()
        }
    }

    fn init(&mut self) {
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
    
        self.window.mv(24, 0);
        self.window.printw("Press q to quit.");
    
        self.window.refresh();
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

            Some(Input::KeyUp) => p.man_y -= 1,
            Some(Input::KeyDown) => p.man_y += 1,
            Some(Input::KeyLeft) => p.man_x -= 1,
            Some(Input::KeyRight) => p.man_x += 1,

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
