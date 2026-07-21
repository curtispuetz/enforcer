use std::sync::LazyLock;

use colored::{ColoredString, Colorize};

pub static SUCCESS_TAG: LazyLock<ColoredString> =
    LazyLock::new(|| "[success]".green().bold());
