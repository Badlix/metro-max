use colored::Colorize;
use rodio::source::{SineWave, Source};
use rodio::DeviceSinkBuilder;
use std::time::Duration;
use std::io::Write;
use std::time::{Instant};
use clap::{Args};

#[derive(Args)]
pub struct StartArgs {
    #[arg(value_parser = clap::value_parser!(u64).range(1..301))]
    pub bpm: u64, 
    #[arg(short, default_value_t = 4)]
    pub beat_number: u16,
    #[arg(long("acc-notes"), value_delimiter = ',', default_values_t = vec![1])]
    pub big_key_index: Vec<u16>,
    #[arg(short, long, default_value_t=false)]
    pub quiet: bool,
}

pub fn start(bpm: u64, beat_nb: u16, big_key_index: Vec<u16>, quiet: bool) {
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

    print_description(bpm);

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

        print_interval(beat, beat_nb, big_key_index.clone());

        std::io::stdout().flush().unwrap();

        if quiet == false {
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

fn print_description(bpm: u64) {
    println!("{} {} {}", "Starting metronome at".green(), bpm.to_string().yellow().bold(), "BPM".green());
    print!("\x1B[?25l");
    std::io::stdout().flush().unwrap();

    ctrlc::set_handler(|| {
        print!("\x1B[2B"); // put cursor at the next free line
        print!("\x1B[?25h"); // show cursor when user quit
        std::io::stdout().flush().unwrap();
        std::process::exit(0);
    }).unwrap();

    for _ in 0..8 {
        println!();
    }

    println!("Press Ctrl+C to stop\n");
    print!("\x1B[3A"); 
}

fn print_interval(current_beat : u16, nb_beat: u16, big_key_index: Vec<u16>) {
    let white_top : String = " --- ".to_string();
    let white_bar : String = "|   |".to_string();
    let red_top   : String = " --- ".red().to_string();
    let red_bar   : String = "|   |".red().to_string();
    let empty_bar : String = "     ".to_string();

    print!("\x1B[6A"); // move cursor 6 lines up

    for line_nb in 0..6 {
        let mut line_to_display : String= "".to_string();

        for beat_nb in 1..nb_beat+1 {
            // Draw big bip top and bottom bar
            if line_nb == 0 || line_nb == 5 {
                if big_key_index.contains(&beat_nb) {
                    if current_beat == beat_nb {
                        line_to_display = format!("{} {}", line_to_display, red_top);
                    } else {
                        line_to_display = format!("{} {}", line_to_display, white_top);
                    }
                } else {
                    line_to_display = format!("{} {}", line_to_display, empty_bar);
                }
            // Draw small bip top and bottom bar
            } else if (line_nb == 1 || line_nb == 4) && !big_key_index.contains(&beat_nb)  {
                if current_beat == beat_nb {
                    line_to_display = format!("{} {}", line_to_display, red_top);
                } else {
                    line_to_display = format!("{} {}", line_to_display, white_top);
                } 
            } else {
                if current_beat == beat_nb {
                    line_to_display = format!("{} {}", line_to_display, red_bar);
                } else {
                    line_to_display = format!("{} {}", line_to_display, white_bar);
                }
            }    
        }
        print!("\r{}\n", line_to_display);
    }
}