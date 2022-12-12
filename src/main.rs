use rand::{thread_rng, Rng};
extern crate pancurses;
use pancurses::{ initscr, start_color, endwin, Input, noecho, init_pair, COLOR_PAIR };
use pancurses::{ COLOR_GREEN, COLOR_WHITE, COLOR_BLACK };

static man_c: char = '☺';
static tree_c: char = 'φ';

struct Tree {
    x: i32,
    y: i32
}

fn main() {
    let (mut man_x, mut man_y) = (40, 12);
    let mut rng = thread_rng();
    let mut trees: Vec<Tree> = Vec::new();
    let mut last_input = 0;

    let window = initscr();

    start_color();
    init_pair(1, COLOR_WHITE, COLOR_BLACK); // man
    init_pair(2, COLOR_GREEN, COLOR_BLACK); // trees

    window.keypad(true);
    window.clear();
    noecho();

    for a in (0..20) {
        let (x, y) = (rng.gen_range(0..80), rng.gen_range(0..25));
        trees.push(Tree { x, y });
    }

    window.attron(COLOR_PAIR(2));
    for tree in trees.iter() {
        window.mv(tree.y, tree.x);
        window.addch(tree_c);
    }
    window.attroff(COLOR_PAIR(2));

    window.attroff(COLOR_PAIR(1));
    window.mv(man_y, man_x);
    window.addch(man_c);
    window.attroff(COLOR_PAIR(1));

    loop {
        window.mv(24, 0);
        window.printw("Press q to quit.");
        let input = window.getch();

        match input {
            Some(Input::Character('q')) => break,

            Some(Input::KeyUp) => man_y -= 1,
            Some(Input::KeyDown) => man_y += 1,
            Some(Input::KeyLeft) => man_x -= 1,
            Some(Input::KeyRight) => man_x += 1,

            None => (),
            _ => {
                window.mv(0, 0);
                window.printw(format!("{:?}", input.unwrap()));
            }
        }
    }

    window.refresh();

    endwin();
}
