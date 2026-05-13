use colored::Colorize;
use rodio::source::{SineWave, Source};
use rodio::DeviceSinkBuilder;
use std::time::Duration;
use std::io::Write;
use std::time::{Instant};
use clap::{Args};

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
    #[arg(short, long, default_value_t=false)]
    pub quiet: bool,

    /// Scale of the metronome visual (max 3)
    #[arg(short, long, default_value_t=1, value_parser = clap::value_parser!(i8).range(1..4))]
    pub scale: i8,
}


pub fn start(bpm: u64, beat_nb: u16, big_key_index: Vec<u16>, quiet: bool, scale: i8) {
    #[cfg(target_os = "windows")]
    unsafe {
        winapi::um::timeapi::timeBeginPeriod(1);
        let thread = winapi::um::processthreadsapi::GetCurrentThread();
        winapi::um::processthreadsapi::SetThreadPriority(
            thread,
            winapi::um::winbase::THREAD_PRIORITY_TIME_CRITICAL as i32,
        );
    }

    let interval_ms = 60_000 / bpm;
    let interval = Duration::from_millis(interval_ms);
    let mut next_tick = Instant::now();

    let handle = DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let mixer = handle.mixer();

    let mut beat : u16 = 1;

    print_description(bpm, scale as usize);

    let start = Instant::now();

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
        
        let ts = start.elapsed().as_millis();

        print_interval(beat, beat_nb, big_key_index.clone(), scale as usize);

        std::io::stdout().flush().unwrap();

        if !quiet {
            let freq = if big_key_index.contains(&beat) { 880.0 } else { 440.0 };
            let source = SineWave::new(freq).take_duration(Duration::from_millis(50)).amplify(0.5);
            mixer.add(source);
        }

        beat += 1;
        if beat > beat_nb {
            beat = 1;
        }
        
        // TODO : for debug
        let ts2 = start.elapsed().as_millis();
        if ts2 - ts  > 300 {
            println!("{} : {}", "Delay !".red().bold(), ((ts2 - ts).to_string().red()));
        }
    }
}

fn print_description(bpm: u64, scale: usize) {
    // Hide cursor
    print!("\x1B[?25l");
    std::io::stdout().flush().unwrap();

    // Quit and reshow cursor
    ctrlc::set_handler(|| {
        print!("\x1B[2B"); 
        print!("\x1B[?25h"); 
        std::io::stdout().flush().unwrap();
        std::process::exit(0);
    }).unwrap();


    println!("{} {} {}", "Starting metronome at".green(), bpm.to_string().yellow().bold(), "BPM".green());

    println!();
    put_cursor_down(scale); // Future space for the visual
    println!();

    println!("Press Ctrl+C to stop\n");
    print!("\x1B[3A"); 
}

fn print_interval(current_beat : u16, nb_beat: u16, big_key_index: Vec<u16>, scale: usize) {
    let top_pattern  = format!(" {} ", "---".repeat(scale));
    let side_pattern = format!("|{}|", "   ".repeat(scale));
    let empty_space  = format!(" {} ", "   ".repeat(scale));

    let visual_height = get_visual_height(scale);  
    let big_beat_top_line = [0, visual_height - 1];      
    let small_beat_top_line = [scale, visual_height - 1 - scale]; 

    put_cursor_up(scale);

    for current_line in 0..visual_height {
        let mut line_to_display : String = "".to_string();

        for beat_nb in 1..nb_beat+1 {

            let is_current_beat = current_beat == beat_nb;
            let new_pattern : &String;

            if big_key_index.contains(&beat_nb) {
                if big_beat_top_line.contains(&current_line) {
                    new_pattern = &top_pattern;
                } else {
                    new_pattern = &side_pattern;
                }
            } else {
                if small_beat_top_line.contains(&current_line) {
                    new_pattern = &top_pattern;
                } else if current_line < small_beat_top_line[0] || current_line > small_beat_top_line[1] {
                    new_pattern = &empty_space;
                } else {
                    new_pattern = &side_pattern;
                }
            }

            line_to_display = format!("{}{}{}", line_to_display, " ".repeat(scale), red_or_white(new_pattern.clone(), is_current_beat));
        }
        print!("\r{}\n", line_to_display);
    }
}

fn red_or_white(pattern : String, is_current_beat : bool) -> String {
    if is_current_beat {
        return pattern.red().to_string();
    }
    return pattern
}

fn put_cursor_up(repeat : usize) {
    let line_up = get_visual_height(repeat);
    for _ in 0..line_up {
        print!("\x1B[1A"); 
    }
}

fn put_cursor_down(repeat : usize) {
    let line_down = get_visual_height(repeat);
    for _ in 0..line_down {
        println!(""); 
    }
}

fn get_visual_height(scale : usize) -> usize{
    return 6*scale;
}