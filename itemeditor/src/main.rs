#![windows_subsystem = "windows"]

#[macro_use]
extern crate derivative;

extern crate num_traits;
extern crate serde_big_array;

mod item;

use araiseal_ui::*;
use backtrace::Backtrace;
use iced::{Column, Container, Element, Length, Sandbox, Settings};
use item::*;
use lazy_static::lazy_static;
use slog::{error, info};
use sloggers::file::FileLoggerBuilder;
use sloggers::types::Severity;
use sloggers::Build;
use std::fs;
use std::panic;

lazy_static! {
    static ref LOGGER: slog::Logger = {
        let mut builder = FileLoggerBuilder::new("log.txt");
        builder.level(Severity::Debug);
        builder.build().unwrap()
    };
}

pub fn main() -> Result<(), String> {
    info!(LOGGER, "starting up");
    info!(LOGGER, "Setting Panic Hook");

    panic::set_hook(Box::new(|panic_info| {
        let bt = Backtrace::new();

        error!(LOGGER, "PANIC: {}, BACKTRACE: {:?}", panic_info, bt);
    }));

    if let Err(e) = fs::create_dir_all("./data/items/") {
        return Err(format!("Err: {:?}", e));
    }

    info!(LOGGER, "Checked or Created Directorys");
    if let Err(e) = ItemData::create_files() {
        return Err(e);
    }

    info!(LOGGER, "Checked or Created Files");

    if let Err(err) = Pages::run(Settings::default()) {
        error!(LOGGER, "{}", err);
    }

    Ok(())
}

pub struct Pages {
    page: Box<dyn UiRenderer<Message = item::Message>>,
}

impl Sandbox for Pages {
    type Message = item::Message;

    fn new() -> Pages {
        Pages {
            page: Box::new(item::ItemUI::new()),
        }
    }

    fn title(&self) -> String {
        self.page.title().to_string()
    }

    fn update(&mut self, event: Message) {
        self.page.update(event);
    }

    fn view(&mut self) -> Element<Message> {
        let page = self.page.view();

        let content: Element<_> = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(20)
            .padding(20)
            .push(page)
            .into();

        Container::new(content)
            .height(Length::Fill)
            .style(araiseal_styles::MainContainer)
            .into()
    }
}
