// 12-12-2022
// "extern crate pancurses", "extern crate" is no longer needed since Rust 2018 because Cargo knows what dependencies to load.
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use pancurses::{ initscr, resize_term, start_color, endwin, Input, noecho, init_pair, COLOR_PAIR, Window, set_title, curs_set };
use pancurses::{ COLOR_GREEN, COLOR_WHITE, COLOR_BLACK };

use std::error::Error;

// for audio
use kira::manager::{
    AudioManager, AudioManagerSettings,
    backend::cpal::CpalBackend,
};
use kira::sound::static_sound::{ StaticSoundData, StaticSoundSettings };

static MAN_C: char = '☺';
static TREE_C: char = 'φ';

struct Point {
    x: i32,
    y: i32
}

struct Program {
    window: Window,
    rng: ThreadRng,

    score: i32,

    // the lumberjack
    man: Point,
    // trees to cut
    trees: Vec<Point>,

    // SFX
    audio_manager: Option<AudioManager<CpalBackend>>,
    hit_sfx: Option<StaticSoundData>,
}

impl Program {
    fn new() -> Program {
        Program {
            window: initscr(),
            trees: Vec::new(),
            score: 0,
            man: Point { x: 40, y: 12 },
            rng: thread_rng(),

            audio_manager: None,
            hit_sfx: None
        }
    }

    fn init_audio(&mut self) -> Result<(), Box<dyn Error>> {
        self.audio_manager = Some(AudioManager::<CpalBackend>::new(
            AudioManagerSettings::default())?);

        Ok(())
    }

    fn load_audio(&mut self) -> Result<(), Box<dyn Error>> {
        let sound_data = StaticSoundData::from_file(
            "Blow2.ogg",
            StaticSoundSettings::default())?;

        self.hit_sfx = Some(sound_data);

        Ok(())
    }

    fn play_hit_sfx(&mut self) -> Result<(), Box<dyn Error>> {
        match &mut self.audio_manager {
            Some(e) => {
                e.play(self.hit_sfx.as_mut().unwrap().clone())?;
            },
            None => ()
        }

        Ok(())
    }

    fn init(&mut self) {
        set_title("Lumberjack - by Hevanafa (Dec 2022)");

        self.init_audio().expect("Can't init audio.");
        self.load_audio().expect("Can't load audio.");

        resize_term(25, 80);

        start_color();
        init_pair(1, COLOR_WHITE, COLOR_BLACK); // man
        init_pair(2, COLOR_GREEN, COLOR_BLACK); // trees
    
        self.window.keypad(true);
        self.window.clear();
        curs_set(0);
        noecho();

        // fill the field with 20 trees
        for _ in 0..20 {
            let (x, y) = (self.rng.gen_range(0..80), self.rng.gen_range(0..25));
            self.trees.push(Point { x, y });
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
    
        self.window.attron(COLOR_PAIR(1));
        self.window.mv(self.man.y, self.man.x);
        self.window.addch(MAN_C);
        self.window.attroff(COLOR_PAIR(1));

        self.window.mv(0, 0);
        self.window.printw(format!("Score: {}", self.score));
    
        self.window.mv(24, 0);
        self.window.printw("Press q to quit.");
    
        self.window.refresh();
    }

    // returns true if there's a tree
    fn hit_tree(&mut self, delta_x: i32, delta_y: i32) -> bool {
        if self.trees.iter().any(|tree|
            tree.x == self.man.x + delta_x &&
            tree.y == self.man.y + delta_y
        ) {
            let idx = self.trees.iter().position(|tree|
                tree.x == self.man.x + delta_x &&
                tree.y == self.man.y + delta_y
            ).unwrap();
            self.trees.remove(idx);

            self.play_hit_sfx().unwrap();
            self.score += 1;

            return true;
        }

        false
    }

    fn step_x(&mut self, inc: i32) {
        // check tree hit
        if self.hit_tree(inc, 0) { return; }

        self.man.x += inc;

        if self.man.x < 0  { self.man.x = 0; }
        if self.man.x > 79 { self.man.x = 79; }
    }

    fn step_y(&mut self, inc: i32) {
        // check tree hit
        if self.hit_tree(0, inc) { return; }

        self.man.y += inc;

        if self.man.y < 0  { self.man.y = 0; }
        if self.man.y > 24 { self.man.y = 24; }
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
