/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::LevelFilter;

fn create_format_dispatch(colors: ColoredLevelConfig) -> Dispatch {
    Dispatch::new().format(move |out, message, record| {
        out.finish(format_args!(
            "{} {:5} [{}]\x1B[1m -- {}\x1B[0m",
            chrono::Local::now().format("%H:%M:%S%.3f"),
            colors.color(record.level()),
            record.target(),
            message,
        ))
    })
}

pub(crate) fn setup_logging(debug: bool) {
    let logging_colors = ColoredLevelConfig::new()
        .debug(Color::BrightBlack)
        .info(Color::Black)
        .warn(Color::BrightYellow)
        .error(Color::BrightRed);

    let dispatch =
        Dispatch::new().chain(create_format_dispatch(logging_colors).chain(std::io::stdout()));

    dispatch.apply().expect("Couldn't start logger");

    let mut level = LevelFilter::Info;

    if debug {
        level = LevelFilter::Debug;
    }

    log::set_max_level(level)
}
