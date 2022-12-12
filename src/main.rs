// 12-12-2022
// "extern crate pancurses", "extern crate" is no longer needed since Rust 2018 because Cargo knows what dependencies to load.
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use pancurses::{ initscr, resize_term, start_color, endwin, Input, noecho, init_pair, COLOR_PAIR, Window, set_title };
use pancurses::{ COLOR_GREEN, COLOR_WHITE, COLOR_BLACK };

use std::error::Error;

use kira::manager::{
    AudioManager, AudioManagerSettings,
    backend::cpal::CpalBackend,
};
use kira::sound::static_sound::{ StaticSoundData, StaticSoundSettings };

// for audio
// use rodio::cpal::traits::HostTrait;
// use std::fs::File;
// use std::io::{BufReader, BufRead};
// use std::time::Duration;
// use rodio::{ cpal, Decoder, OutputStream, Sink, DeviceTrait };
// use rodio::source::{SineWave, Source};

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
    trees: Vec<Point>,

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
        // if self.audio_manager.is_none() { return Ok(()); }

        // self.audio_manager.unwrap().play(
        //     self.hit_sfx.unwrap().clone());

        // let mut manager: AudioManager = *self.audio_manager.as_ref().unwrap();
        // manager.play(self.hit_sfx.unwrap().clone())?;

        Ok(())
    }

    // old one, with rodio
    // fn load_audio(&mut self) -> Result<(), Box<dyn Error>> {
    //     println!("Loading audio...");

        // let host = cpal::default_host();
        // let devices = host.output_devices()?;
        // let mut sink: Option<Sink> = None;

        // for d in devices {
        //     dbg!(format!("{:?}", d.name()));
        //     let (_, handle) = OutputStream::try_from_device(&d)?;
        //     sink = Some(Sink::try_new(&handle)?);
        //     break;
        // }

        // let (_, handle) = OutputStream::try_default().unwrap();
        // sink = Sink::try_new(&handle).unwrap();

        // let file = BufReader::new(File::open("Blow2.ogg").expect("Can't find Blow2.ogg"));
        // let source = Decoder::new(file).unwrap();
        // handle.play_raw(source.convert_samples())?;

        // let source = SineWave::new(440.0)
        //     .take_duration(Duration::from_secs_f32(0.25))
        //     .amplify(0.2);

    //     let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    //     let sink = Sink::try_new(&stream_handle).unwrap();

    //     // load_audio
    //     let file = BufReader::new(File::open("Blow2.ogg").unwrap());
    //     self.hit_sfx = Some(file);

    //     Ok(())
    // }

    // fn play_hit_sfx(&self) {
    //     match self.hit_sfx {
    //         Some(..) => {
    //             let source = Decoder::new(&self.hit_sfx).unwrap();
    //             self.sink.append(source);
    //         },
    //         None => return
    //     }

    //     // sink.sleep_until_end();
    // }

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
    
        self.window.attroff(COLOR_PAIR(1));
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
    fn check_tree(&mut self, delta_x: i32, delta_y: i32) -> bool {
        if self.trees.iter().any(|tree|
            tree.x == self.man.x + delta_x &&
            tree.y == self.man.y + delta_y
        ) {
            // let tree = self.trees.iter().find(|tree| tree.x == self.man_x + inc);
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
        if self.check_tree(inc, 0) { return; }

        self.man.x += inc;

        if self.man.x < 0  { self.man.x = 0; }
        if self.man.x > 79 { self.man.x = 79; }
    }

    fn step_y(&mut self, inc: i32) {
        // check tree hit
        if self.check_tree(0, inc) { return; }

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
