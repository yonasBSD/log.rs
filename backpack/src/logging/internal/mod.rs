mod enums;
pub mod log;
mod logproxy;
mod utils;

pub mod globals;

pub use enums::*;
pub use globals::*;
pub use log::*;
pub use logproxy::*;
pub use utils::*;

/*
use std::{collections::BTreeMap, sync::Arc};
use terminal_banner::Banner;
use tracing_subscriber::{
    Layer, Registry, filter::LevelFilter, fmt::writer::BoxMakeWriter, prelude::*,
};

use globals::{INIT, LOGGER, PROJECT_DESC, PROJECT_NAME};
use utils::format_duration;
*/
