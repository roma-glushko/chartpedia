/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use chrono::Utc;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::LevelFilter;

fn create_format_dispatch(colors: Option<ColoredLevelConfig>) -> Dispatch {
    Dispatch::new().format(move |out, message, record| {
        if let Some(colors) = colors {
            out.finish(format_args!(
                "{} {} [{}] {}",
                Utc::now().format("%Y/%m/%d %H:%M:%S"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        } else {
            out.finish(format_args!(
                "{} {} [{}] {}",
                Utc::now().format("%Y/%m/%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        }
    })
}

pub(crate) fn setup_logging(verbosity: LevelFilter) {
    let logging_colors = ColoredLevelConfig::new()
        .debug(Color::BrightMagenta)
        .info(Color::BrightCyan)
        .warn(Color::BrightYellow)
        .error(Color::BrightRed);

    let dispatch = Dispatch::new()
        .chain(create_format_dispatch(Some(logging_colors)).chain(std::io::stdout()));

    dispatch.apply().expect("Couldn't start logger.");

    log::set_max_level(verbosity)
}
