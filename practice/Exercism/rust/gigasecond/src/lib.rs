use std::ops::Add;

use time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.add(Duration::seconds(1_000_000_000))
    // unimplemented!()
}
