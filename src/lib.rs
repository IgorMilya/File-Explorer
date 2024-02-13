pub mod utils;

mod search;
mod check_os;
mod check_all_disks;

pub use search::search;
pub use check_os::check_os;
pub use check_all_disks::check_all_disks;

