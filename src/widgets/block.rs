use buffer::Buffer;
use layout::Rect;
use style::Style;
use widgets::{Borders, Widget};


use symbols::{line,
              line_heavy,
              line_dash_double,
              line_dash_triple,
              line_rounded,
              line_double};
use style::LineType;

/// Base widget to be used with all upper level ones. It may be used to display a box border around
/// the widget and/or add a title.
///
/// # Examples
///
/// ```
/// # extern crate tui;
/// # use tui::widgets::{Block, Borders};
/// # use tui::style::{Style, Color};
/// # fn main() {
/// Block::default()
///     .title("Block")
///     .title_style(Style::default().fg(Color::Red))
///     .borders(Borders::LEFT | Borders::RIGHT)
///     .border_style(Style::default().fg(Color::White))
///     .style(Style::default().bg(Color::Black));
/// # }
/// ```
#[derive(Clone, Copy)]
pub struct Block<'a> {
    /// Optional title place on the upper left of the block
    title: Option<&'a str>,
    /// Title style
    title_style: Style,
    /// Visible borders
    borders: Borders,
    /// Border style
    border_style: Style,
    /// Widget style
    style: Style,
}

impl<'a> Default for Block<'a> {
    fn default() -> Block<'a> {
        Block {
            title: None,
            title_style: Default::default(),
            borders: Borders::NONE,
            border_style: Default::default(),
            style: Default::default(),
        }
    }
}

impl<'a> Block<'a> {
    pub fn title(mut self, title: &'a str) -> Block<'a> {
        self.title = Some(title);
        self
    }

    pub fn title_style(mut self, style: Style) -> Block<'a> {
        self.title_style = style;
        self
    }

    pub fn border_style(mut self, style: Style) -> Block<'a> {
        self.border_style = style;
        self
    }

    pub fn style(mut self, style: Style) -> Block<'a> {
        self.style = style;
        self
    }

    pub fn borders(mut self, flag: Borders) -> Block<'a> {
        self.borders = flag;
        self
    }

    /// Compute the inner area of a block based on its border visibility rules.
    pub fn inner(&self, area: &Rect) -> Rect {
        if area.width < 2 || area.height < 2 {
            return Rect::default();
        }
        let mut inner = *area;
        if self.borders.intersects(Borders::LEFT) {
            inner.x += 1;
            inner.width -= 1;
        }
        if self.borders.intersects(Borders::TOP) || self.title.is_some() {
            inner.y += 1;
            inner.height -= 1;
        }
        if self.borders.intersects(Borders::RIGHT) {
            inner.width -= 1;
        }
        if self.borders.intersects(Borders::BOTTOM) {
            inner.height -= 1;
        }
        inner
    }
}

impl<'a> Widget for Block<'a> {
    fn draw(&mut self, area: &Rect, buf: &mut Buffer) {
        if area.width < 2 || area.height < 2 {
            return;
        }

        self.background(area, buf, self.style.bg);

        // Sides
        if self.borders.intersects(Borders::LEFT) {
            for y in area.top()..area.bottom() {
                buf.get_mut(area.left(), y)
                   .set_symbol(get_symbol_for_line_type(line::VERTICAL, self.border_style))
                   .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::TOP) {
            for x in area.left()..area.right() {
                buf.get_mut(x, area.top())
                   .set_symbol(get_symbol_for_line_type(line::HORIZONTAL, self.border_style))
                   .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::RIGHT) {
            let x = area.right() - 1;
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y)
                   .set_symbol(get_symbol_for_line_type(line::VERTICAL, self.border_style))
                   .set_style(self.border_style);
            }
        }
        if self.borders.intersects(Borders::BOTTOM) {
            let y = area.bottom() - 1;
            for x in area.left()..area.right() {
                buf.get_mut(x, y)
                   .set_symbol(get_symbol_for_line_type(line::HORIZONTAL, self.border_style))
                   .set_style(self.border_style);
            }
        }

        // Corners
        if self.borders.contains(Borders::LEFT | Borders::TOP) {
            buf.get_mut(area.left(), area.top())
               .set_symbol(get_symbol_for_line_type(line::TOP_LEFT, self.border_style))
               .set_style(self.border_style);
        }
        if self.borders.contains(Borders::RIGHT | Borders::TOP) {
            buf.get_mut(area.right() - 1, area.top())
               .set_symbol(get_symbol_for_line_type(line::TOP_RIGHT, self.border_style))
               .set_style(self.border_style);
        }
        if self.borders.contains(Borders::LEFT | Borders::BOTTOM) {
            buf.get_mut(area.left(), area.bottom() - 1)
               .set_symbol(get_symbol_for_line_type(line::BOTTOM_LEFT, self.border_style))
               .set_style(self.border_style);
        }
        if self.borders.contains(Borders::RIGHT | Borders::BOTTOM) {
            buf.get_mut(area.right() - 1, area.bottom() - 1)
               .set_symbol(get_symbol_for_line_type(line::BOTTOM_RIGHT, self.border_style))
               .set_style(self.border_style);
        }

        if area.width > 2 {
            if let Some(title) = self.title {
                let lx = if self.borders.intersects(Borders::LEFT) {
                    1
                } else {
                    0
                };
                let rx = if self.borders.intersects(Borders::RIGHT) {
                    1
                } else {
                    0
                };
                let width = area.width - lx - rx;
                buf.set_stringn(
                    area.left() + lx,
                    area.top(),
                    title,
                    width as usize,
                    &self.title_style,
                );
            }
        }
    }
}

/// Helper function to swap a border symbol to styled linetype.
pub fn get_symbol_for_line_type(symbol: &str, style: Style) -> &str {
    match style.line_type {
        LineType::Reset => symbol,
        LineType::Regular => symbol,

        LineType::Heavy => match symbol {
            line::TOP_LEFT => line_heavy::TOP_LEFT,
            line::TOP_RIGHT => line_heavy::TOP_RIGHT,
            line::VERTICAL => line_heavy::VERTICAL,
            line::HORIZONTAL => line_heavy::HORIZONTAL,
            line::BOTTOM_RIGHT => line_heavy::BOTTOM_RIGHT,
            line::BOTTOM_LEFT => line_heavy::BOTTOM_LEFT,
            line::VERTICAL_LEFT => line_heavy::VERTICAL_LEFT,
            line::VERTICAL_RIGHT => line_heavy::VERTICAL_RIGHT,
            line::HORIZONTAL_DOWN => line_heavy::HORIZONTAL_DOWN,
            line::HORIZONTAL_UP => line_heavy::HORIZONTAL_UP,
            _ => symbol
        },

        LineType::Double => match symbol {
            line::TOP_LEFT => line_double::TOP_LEFT,
            line::TOP_RIGHT => line_double::TOP_RIGHT,
            line::VERTICAL => line_double::VERTICAL,
            line::HORIZONTAL => line_double::HORIZONTAL,
            line::BOTTOM_RIGHT => line_double::BOTTOM_RIGHT,
            line::BOTTOM_LEFT => line_double::BOTTOM_LEFT,
            line::VERTICAL_LEFT => line_double::VERTICAL_LEFT,
            line::VERTICAL_RIGHT => line_double::VERTICAL_RIGHT,
            line::HORIZONTAL_DOWN => line_double::HORIZONTAL_DOWN,
            line::HORIZONTAL_UP => line_double::HORIZONTAL_UP,
            _ => symbol
        },

        LineType::DashDouble =>
            match symbol {
                line::VERTICAL => line_dash_double::VERTICAL,
                line::HORIZONTAL => line_dash_double::HORIZONTAL,
                _ => symbol
            },

        LineType::DashTriple =>
            match symbol {
                line::VERTICAL => line_dash_triple::VERTICAL,
                line::HORIZONTAL => line_dash_triple::HORIZONTAL,
                _ => symbol
            },

        LineType::RegularRounded =>
            match symbol {
                line::TOP_LEFT => line_rounded::TOP_LEFT,
                line::TOP_RIGHT => line_rounded::TOP_RIGHT,
                line::BOTTOM_RIGHT => line_rounded::BOTTOM_RIGHT,
                line::BOTTOM_LEFT => line_rounded::BOTTOM_LEFT,
                _ => symbol
            },
    }
}

