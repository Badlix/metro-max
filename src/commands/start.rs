use colored::Colorize;
use rodio::source::{SineWave, Source};
use rodio::DeviceSinkBuilder;
use std::time::Duration;
use std::io::Write;
use std::time::{Instant};
use clap::{Args};

// - - - PARSE ARGUMENTS - - -

#[derive(Args)]
#[command(about)]
pub struct StartArgs {

    #[arg(value_parser = clap::value_parser!(u64).range(1..301))]
    pub bpm: u64, 
    
    /// Number of beats per measure
    #[arg(short, default_value_t = 4)]
    pub beat_number: u16,

    /// List of accented beat numbers
    #[arg(long("acc-notes"), value_delimiter = ',', default_values_t = vec![1])]
    pub big_key_index: Vec<u16>,

    /// Run the metronome in visual-only mode
    #[arg(short, long)]
    pub quiet: bool,

    /// Scale of the metronome visual (max 3)
    #[arg(short, long, default_value_t=1, value_parser = clap::value_parser!(i8).range(1..4))]
    pub scale: i8,
}

// - - - CONSTANT / DATA STRUCTURE - - - 

const HIDE_CURSOR :&str = "\x1B[?25l";
const SHOW_CURSOR :&str = "\x1B[?25h";

macro_rules! cursor_move_down {
    ($n:expr) => { 
        print!("\x1B[{}B", $n);
    } 
}

macro_rules! cursor_move_up {
    ($n:expr) => { 
        print!("\x1B[{}A", $n);
    } 
}

struct Patterns {
    top:   String,
    side:  String,
    empty: String,
}

impl Patterns {
    fn new(scale: usize) -> Self {
        Self {
            top:   format!(" {} ", "---".repeat(scale)),
            side:  format!("|{}|", "   ".repeat(scale)),
            empty: format!(" {} ", "   ".repeat(scale)),
        }
    }
}

// - - - COMMAND IMPLEMENTATION - - -

pub fn start(bpm: u64, beat_nb: u16, big_key_index: Vec<u16>, quiet: bool, scale: i8) {
    let interval = Duration::from_millis(60_000 / bpm);
    let mut next_tick = Instant::now();

    let handle = DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let mixer = handle.mixer();

    let mut beat : u16 = 1;

    let patterns = Patterns::new(scale as usize);
    print_description(bpm, scale as usize);

    loop {

        next_tick += interval;

        // Sleep until close to the next tick
        let now = Instant::now();
        if next_tick > now {
            let sleep_time = next_tick - now;
            // Sleep most of the time normally
            if sleep_time > Duration::from_millis(2) {
                std::thread::sleep(sleep_time - Duration::from_millis(2));
            }
            // Busy-wait for the last 2ms to be precise
            while Instant::now() < next_tick {
                std::hint::spin_loop();
            }
        }

        show_visual(beat, beat_nb, &big_key_index, &patterns, scale as usize);

        std::io::stdout().flush().unwrap();

        if !quiet {
            let freq = if big_key_index.contains(&beat) { 880.0 } else { 440.0 };
            let source = SineWave::new(freq).take_duration(Duration::from_millis(50)).amplify(0.5);
            mixer.add(source);
        }

        beat = (beat % (beat_nb+1)) + 1;
    }
}

fn print_description(bpm: u64, scale: usize) {
    print!("{}", HIDE_CURSOR);
    std::io::stdout().flush().unwrap();

    // Quit and reshow cursor
    ctrlc::set_handler(|| {
        cursor_move_down!(2);
        print!("{}", SHOW_CURSOR); 
        std::io::stdout().flush().unwrap();
        std::process::exit(0);
    }).unwrap();


    println!("{} {} {}", "Starting metronome at".green(), bpm.to_string().yellow().bold(), "BPM".green());
    put_cursor_under_visual(scale);
    println!("Press Ctrl+C to stop\n");
    cursor_move_up!(3);
}

fn show_visual(current_beat : u16, nb_beat: u16, big_key_index: &[u16], patterns :&Patterns, scale: usize) {
    let visual_height = get_visual_height(scale);  
    let big_beat_top_line = [0, visual_height - 1];      
    let small_beat_top_line = [scale, visual_height - 1 - scale]; 

    put_cursor_above_visual(scale);

    for current_line in 0..visual_height {
        let mut line_to_display = String::new();

        for beat_nb in 1..nb_beat+1 {

            let is_current_beat = current_beat == beat_nb;
            let new_pattern : &String;

            if big_key_index.contains(&beat_nb) {
                if big_beat_top_line.contains(&current_line) {
                    new_pattern = &patterns.top;
                } else {
                    new_pattern = &patterns.side;
                }
            } else {
                if small_beat_top_line.contains(&current_line) {
                    new_pattern = &patterns.top;
                } else if current_line < small_beat_top_line[0] || current_line > small_beat_top_line[1] {
                    new_pattern = &patterns.empty;
                } else {
                    new_pattern = &patterns.side;
                }
            }

            line_to_display = format!("{}{}{}", line_to_display, " ".repeat(scale), red_or_white(new_pattern, is_current_beat));
        }
        print!("\r{}\n", line_to_display);
    }
}

fn red_or_white(pattern: &str, is_current_beat: bool) -> String {
    match is_current_beat {
        true  => pattern.red().to_string(),
        false => pattern.to_string(),
    }
}

fn put_cursor_above_visual(scale : usize) {
    print!("\x1B[{}A", get_visual_height(scale));
}

fn put_cursor_under_visual(scale : usize) {
    cursor_move_down!(get_visual_height(scale) + 2);
}

fn get_visual_height(scale : usize) -> usize{
    6*scale
}