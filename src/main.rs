//! A morning greeting cli that uses the Wibold of Cambrai's Ludus Claricalis to determine the virtue of the day.
//! Enumeration of virtues and dice combinations is taken from Donald Knuth's "The Art of Computer Programming, Volume 4A" on page 493.

use crate::ludus_claricalis::{LudusClaricalis, Output as LCOutput};
use clap::Parser;

pub mod ludis_claricalis;

#[derive(Parser)]
struct Cli {
    /// duration of animation in milliseconds
    #[clap(default_value = "4000")]
    duration: usize,
}

fn main() {
    println!("Good morning!");
    let duration = Cli::parse().duration;
    let LCOutput { roll, virtue } = LudusClaricalis::play(duration);
    println!("{roll} -- today will be a day of {virtue}.");
}
