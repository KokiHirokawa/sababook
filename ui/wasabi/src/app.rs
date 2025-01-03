use alloc::format;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use core::cell::RefCell;
use noli::error::Result as OsResult;
use noli::prelude::SystemApi;
use noli::println;
use noli::rect::Rect;
use noli::sys::api::MouseEvent;
use noli::sys::wasabi::Api;
use noli::window::{StringSize, Window};
use saba_core::browser::Browser;
use saba_core::constants::{ADDRESSBAR_HEIGHT, BLACK, DARKGRAY, GREY, LIGHTGRAY, TITLE_BAR_HEIGHT, TOOLBAR_HEIGHT, WHITE, WINDOW_HEIGHT, WINDOW_INIT_X_POS, WINDOW_INIT_Y_POS, WINDOW_WIDTH};
use saba_core::error::Error;

#[derive(Debug)]
pub struct WasabiUI {
    browser: Rc<RefCell<Browser>>,
    input_url: String,
    input_mode: InputMode,
    window: Window,
}

impl WasabiUI {
    pub fn new(browser: Rc<RefCell<Browser>>) -> Self {
        Self {
            browser,
            input_url: String::new(),
            input_mode: InputMode::Normal,
            window: Window::new(
                "saba".to_string(),
                WHITE,
                WINDOW_INIT_X_POS,
                WINDOW_INIT_Y_POS,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
            )
                .unwrap()
        }
    }
}

impl WasabiUI {
    pub fn start(&mut self) -> Result<(), Error> {
        self.setup()?;
        self.run_app()?;
        Ok(())
    }

    fn setup(&mut self) -> Result<(), Error> {
        if let Err(error) = self.setup_toolbar() {
            return Err(Error::InvalidUI(format!(
                "failed to initialize a toolbar with error: {:#?}",
                error
            )));
        }

        self.window.flush();
        Ok(())
    }

    fn setup_toolbar(&mut self) -> OsResult<()> {
        // ツールバーの背景の四角を描画
        self.window.fill_rect(LIGHTGRAY, 0, 0, WINDOW_WIDTH, TOOLBAR_HEIGHT)?;

        // ツールバーとコンテンツエリアの境目の線を描画
        self.window.draw_line(GREY, 0, TOOLBAR_HEIGHT, WINDOW_WIDTH - 1, TOOLBAR_HEIGHT)?;
        self.window.draw_line(DARKGRAY, 0, TOOLBAR_HEIGHT + 1, WINDOW_WIDTH - 1, TOOLBAR_HEIGHT + 1)?;

        // アドレスバーの横に "Address:" という文字列を描画
        self.window.draw_string(BLACK, 5, 5, "Address:", StringSize::Medium, false)?;

        // アドレスバーの四角を描画
        self.window.fill_rect(WHITE, 70, 2, WINDOW_WIDTH - 74, 2 + ADDRESSBAR_HEIGHT)?;

        // アドレスバーの影の線を描画
        self.window.draw_line(GREY, 70, 2, WINDOW_WIDTH - 4, 2)?;
        self.window.draw_line(GREY, 70, 2, 70, 2 + ADDRESSBAR_HEIGHT)?;
        self.window.draw_line(BLACK, 71, 3, WINDOW_WIDTH - 5, 3)?;
        self.window.draw_line(GREY, 71, 3, 71, 1 + ADDRESSBAR_HEIGHT)?;

        Ok(())
    }

    fn run_app(&mut self) -> Result<(), Error> {
        loop {
            self.handle_mouse_input()?;
            self.handle_key_input()?;
        }
    }

    fn handle_mouse_input(&mut self) -> Result<(), Error> {
        if let Some(
            MouseEvent {
                button,
                position,
            }
        ) = Api::get_mouse_cursor_info() {
            if button.l() || button.c() || button.r() {
                let relative_pos = (
                    position.x - WINDOW_INIT_X_POS,
                    position.y - WINDOW_INIT_Y_POS,
                );

                if relative_pos.0 < 0
                    || WINDOW_WIDTH < relative_pos.0
                    || relative_pos.1 < 0
                    || WINDOW_HEIGHT < relative_pos.1
                {
                    println!("button clicked OUTSIDE window: {button:?} {position:?}");
                    return Ok(());
                }

                if TITLE_BAR_HEIGHT <= relative_pos.1
                    && relative_pos.1 < TOOLBAR_HEIGHT + TITLE_BAR_HEIGHT
                {
                    self.clear_address_bar()?;
                    self.input_url = String::new();
                    self.input_mode = InputMode::Editing;
                    println!("button clicked in toolbar: {button:?} {position:?}");
                    return Ok(());
                }

                self.input_mode = InputMode::Normal;
            }
        }

        Ok(())
    }

    fn handle_key_input(&mut self) -> Result<(), Error> {
        match self.input_mode {
            InputMode::Normal => {
                // 入力を無視する
                let _ = Api::read_key();
            }
            InputMode::Editing => {
                if let Some(c) = Api::read_key() {
                    if c == 0x7F as char || c == 0x08 as char {
                        self.input_url.pop();
                        self.update_address_bar();
                    } else {
                        self.input_url.push(c);
                        self.update_address_bar();
                    }
                }
            }
        }

        Ok(())
    }

    fn update_address_bar(&mut self) -> Result<(), Error> {
        if self
            .window
            .fill_rect(WHITE, 72, 4, WINDOW_WIDTH - 76, ADDRESSBAR_HEIGHT - 2)
            .is_err() {
            return Err(Error::InvalidUI(
                "failed to clear an address bar".to_string(),
            ));
        }

        if self
            .window
            .draw_string(
                BLACK,
                74,
                6,
                &self.input_url,
                StringSize::Medium,
                false,
            )
            .is_err() {
            return Err(Error::InvalidUI(
                "failed to update an address bar".to_string(),
            ));
        }

        self.window.flush_area(
            Rect::new(
                WINDOW_INIT_X_POS,
                WINDOW_INIT_Y_POS + TITLE_BAR_HEIGHT,
                WINDOW_WIDTH,
                TOOLBAR_HEIGHT,
            )
                .expect("failed to create a rect for the address bar"),
        );

        Ok(())
    }

    fn clear_address_bar(&mut self) -> Result<(), Error> {
        if self
            .window
            .fill_rect(WHITE, 72, 4, WINDOW_WIDTH - 76, ADDRESSBAR_HEIGHT - 2)
            .is_err() {
            return Err(Error::InvalidUI(
                "failed to clear an address bar".to_string(),
            ));
        }

        self.window.flush_area(
            Rect::new(
                WINDOW_INIT_X_POS,
                WINDOW_INIT_Y_POS + TITLE_BAR_HEIGHT,
                WINDOW_WIDTH,
                TOOLBAR_HEIGHT,
            )
                .expect("failed to create a rect for the address bar"),
        );

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InputMode {
    Normal,
    Editing,
}