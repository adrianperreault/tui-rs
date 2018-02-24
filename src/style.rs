#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightMagenta,
    LightCyan,
    White,
    Rgb(u8, u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Modifier {
    Blink,
    Bold,
    CrossedOut,
    Faint,
    Framed,
    Invert,
    Italic,
    NoBlink,
    NoBold,
    NoCrossedOut,
    NoFaint,
    NoInvert,
    NoItalic,
    NoUnderline,
    Reset,
    Underline,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineType {
    Reset,
    Regular,
    RegularRounded,
    Heavy,
    Double,
    DashDouble,
    DashTriple,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub modifier: Modifier,
    pub line_type: LineType,
}

impl Default for Style {
    fn default() -> Style {
        Style {
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::Reset,
            line_type: LineType::Regular,
        }
    }
}

impl Style {
    pub fn reset(&mut self) {
        self.fg = Color::Reset;
        self.bg = Color::Reset;
        self.modifier = Modifier::Reset;
        self.line_type = LineType::Reset;
    }

    pub fn fg(mut self, color: Color) -> Style {
        self.fg = color;
        self
    }
    pub fn bg(mut self, color: Color) -> Style {
        self.bg = color;
        self
    }
    pub fn modifier(mut self, modifier: Modifier) -> Style {
        self.modifier = modifier;
        self
    }
    pub fn line_type(mut self, line_type: LineType) -> Style {
        self.line_type = line_type;
        self
    }
}
