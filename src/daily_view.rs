use cosmic::iced_core::Alignment;
use cosmic::widget::{column, grid, icon, row, text};
use cosmic::Element;

use crate::app::{App, Message};

impl App where Self: cosmic::Application {
    pub fn view_daily_forecast(&self) -> Element<Message> {
        column()
            .spacing(24)
            .push(text::title1(self.config.location.clone()))
            .push(text("Daily View will appear here."))
            .into()
    }
}
