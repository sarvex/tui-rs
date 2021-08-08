//! [tui](https://github.com/fdehau/tui-rs) is a library used to build rich
//! terminal users interfaces and dashboards.
//!
//! ![](https://raw.githubusercontent.com/fdehau/tui-rs/master/assets/demo.gif)
//!
//! # Get started
//!
//! ## Adding `tui` as a dependency
//!
//! ```toml
//! [dependencies]
//! tui = "0.16"
//! termion = "1.5"
//! ```
//!
//! The crate is using the `termion` backend by default but if for example you want your
//! application to work on Windows, you might want to use the `crossterm` backend instead. This can
//! be done by changing your dependencies specification to the following:
//!
//! ```toml
//! [dependencies]
//! crossterm = "0.20"
//! tui = { version = "0.16", default-features = false, features = ['crossterm'] }
//! ```
//!
//! The same logic applies for all other available backends.
//!
//! ## Creating a `Terminal`
//!
//! Every application using `tui` should start by instantiating a `Terminal`. It is a light
//! abstraction over available backends that provides basic functionalities such as clearing the
//! screen, hiding the cursor, etc.
//!
//! ```rust,no_run
//! use std::io;
//! use tui::Terminal;
//! use tui::backend::TermionBackend;
//! use termion::raw::IntoRawMode;
//!
//! fn main() -> Result<(), io::Error> {
//!     let stdout = io::stdout().into_raw_mode()?;
//!     let backend = TermionBackend::new(stdout);
//!     let mut terminal = Terminal::new(backend)?;
//!     Ok(())
//! }
//! ```
//!
//! If you had previously chosen `crossterm` as a backend, the terminal can be created in a similar
//! way:
//!
//! ```rust,ignore
//! use std::io;
//! use tui::Terminal;
//! use tui::backend::CrosstermBackend;
//!
//! fn main() -> Result<(), io::Error> {
//!     let stdout = io::stdout();
//!     let backend = CrosstermBackend::new(stdout);
//!     let mut terminal = Terminal::new(backend)?;
//!     Ok(())
//! }
//! ```
//!
//! You may also refer to the examples to find out how to create a `Terminal` for each available
//! backend.
//!
//! ## Building a User Interface (UI)
//!
//! Every component of your interface will be implementing the `Widget` trait. The library comes
//! with a predefined set of widgets that should meet most of your use cases. You are also free to
//! implement your own.
//!
//! Each widget follows a builder pattern API providing a default configuration along with methods
//! to customize them. The widget is then rendered using [`Frame::render_widget`] which takes
//! your widget instance and an area to draw to.
//!
//! The following example renders a block of the size of the terminal:
//!
//! ```rust,no_run
//! use std::io;
//! use termion::raw::IntoRawMode;
//! use tui::Terminal;
//! use tui::backend::TermionBackend;
//! use tui::widgets::{Widget, Block, Borders};
//! use tui::layout::{Layout, Constraint, Direction};
//!
//! fn main() -> Result<(), io::Error> {
//!     let stdout = io::stdout().into_raw_mode()?;
//!     let backend = TermionBackend::new(stdout);
//!     let mut terminal = Terminal::new(backend)?;
//!     terminal.draw(|f| {
//!         let size = f.size();
//!         let block = Block::default()
//!             .title("Block")
//!             .borders(Borders::ALL);
//!         f.render_widget(block, size);
//!     })?;
//!     Ok(())
//! }
//! ```
//!
//! ## Layout
//!
//! The library comes with a basic yet useful layout management object called `Layout`. As you may
//! see below and in the examples, the library makes heavy use of the builder pattern to provide
//! full customization. And `Layout` is no exception:
//!
//! ```rust,no_run
//! use std::io;
//! use termion::raw::IntoRawMode;
//! use tui::Terminal;
//! use tui::backend::TermionBackend;
//! use tui::widgets::{Widget, Block, Borders};
//! use tui::layout::{Layout, Constraint, Direction, Unit};
//!
//! fn main() -> Result<(), io::Error> {
//!     let stdout = io::stdout().into_raw_mode()?;
//!     let backend = TermionBackend::new(stdout);
//!     let mut terminal = Terminal::new(backend)?;
//!     terminal.draw(|f| {
//!         let chunks = Layout::default()
//!             .direction(Direction::Vertical)
//!             .margin(1)
//!             .constraints(
//!                 [
//!                     Constraint::eq(Unit::Percentage(10)),
//!                     Constraint::eq(Unit::Percentage(80)),
//!                     Constraint::eq(Unit::Percentage(10))
//!                 ]
//!             )
//!             .split(f.size());
//!         let block = Block::default()
//!              .title("Block")
//!              .borders(Borders::ALL);
//!         f.render_widget(block, chunks[0]);
//!         let block = Block::default()
//!              .title("Block 2")
//!              .borders(Borders::ALL);
//!         f.render_widget(block, chunks[1]);
//!     })?;
//!     Ok(())
//! }
//! ```
//!
//! This let you describe responsive terminal UI by nesting layouts.

pub mod backend;
pub mod buffer;
pub mod layout;
pub mod style;
pub mod symbols;
pub mod terminal;
pub mod text;
pub mod widgets;

pub use self::terminal::{Frame, Terminal, TerminalOptions, Viewport};
