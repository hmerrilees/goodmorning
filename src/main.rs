//! A morning greeting cli that uses the Wibold of Cambrai's Ludus Claricalis to determine the virtue of the day.
//! Enumeration of virtues and dice combinations is taken from Donald Knuth's "The Art of Computer Programming, Volume 4A" on page 493.

use crate::ludis_claricalis::{LudusClaricalis, Output as LCOutput};
pub mod ludis_claricalis;

fn main() {
    println!("Good morning!");
    let LCOutput { roll, virtue } = LudusClaricalis::play();
    println!("{roll} -- today will be a day of {virtue}.");
}
