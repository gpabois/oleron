pub struct Display(u8);

impl std::ops::BitOr<Display> for Display {
    type Output = Display;

    fn bitor(self, rhs: Display) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Display {
    pub const DISPLAY_MODE_MASK: u8 = 0b11;

    pub const DISPLAY_INSIDE_OUTSIDE: u8 = 0b00;
    pub const DISPLAY_INTERNAL: u8 = 0b1;
    pub const DISPLAY_BOX: u8 = 0b10;
    pub const DISPLAY_LISTITEM: u8 = 0b11;

    pub const DISPLAY_OUTSIDE_SHIFT: u8 = 2;
    pub const DISPLAY_OUTSIDE_MASK: u8 = 0b1100;
    pub const DISPLAY_OUTSIDE_BLOCK: u8 = 0b1;
    pub const DISPLAY_OUTSIDE_INLINE: u8 = 0b10;
    pub const DISPLAY_OUTSIDE_RUN_IN: u8 = 0b11;

    pub const DISPLAY_INSIDE_SHIFT: u8 = 4;
    pub const DISPLAY_INSIDE_MASK: u8 = 0b1110000;
    pub const DISPLAY_INSIDE_FLOW: u8 = 0b1;
    pub const DISPLAY_INSIDE_FLOW_ROOT: u8 = 0b10;
    pub const DISPLAY_INSIDE_FLEX: u8 = 0b11;
    pub const DISPLAY_INSIDE_GRID: u8 = 0b100;
    pub const DISPLAY_INSIDE_RUBY: u8 = 0b101;
    pub const DISPLAY_INSIDE_TABLE: u8 = 0b110;

    pub const DISPLAY_INTERNAL_SHIFT: u8 = 2;
    pub const DISPLAY_INTERNAL_MASK: u8 = 0b111100;
    pub const DISPLAY_INTERNAL_TABLE_ROW_GROUP: u8 = 0b1;
    pub const DISPLAY_INTERNAL_TABLE_HEADER_GROUP: u8 = 0b10;
    pub const DISPLAY_INTERNAL_TABLE_FOOTER_GROUP: u8 = 0b11;
    pub const DISPLAY_INTERNAL_TABLE_ROW: u8 = 0b100;
    pub const DISPLAY_INTERNAL_TABLE_CELL: u8 = 0b101;
    pub const DISPLAY_INTERNAL_TABLE_COLUMN_GROUP: u8 = 0b110;
    pub const DISPLAY_INTERNAL_TABLE_COLUMN: u8 = 0b111;
    pub const DISPLAY_INTERNAL_TABLE_CAPTION: u8 = 0b1000;
    pub const DISPLAY_INTERNAL_RUBY_BASE: u8 = 0b1001;
    pub const DISPLAY_INTERNAL_RUBY_TEXT: u8 = 0b1010;
    pub const DISPLAY_INTERNAL_RUBY_BASE_CONTAINER: u8 = 0b1011;
    pub const DISPLAY_INTERNAL_RUBY_TEXT_CONTAINER: u8 = 0b1100;

    pub const DISPLAY_BOX_SHIFT: u8 = 2;
    pub const DISPLAY_BOX_MASK: u8 = 0b0011;
    pub const DISPLAY_BOX_CONTENTS: u8 = 0b1;
    pub const DISPLAY_NONE_CONTENTS: u8 = 0b10;

}


impl From<DisplayInside> for Display {
    fn from(value: DisplayInside) -> Self {
        Self(((value as u8) << Display::DISPLAY_INSIDE_SHIFT) | Display::DISPLAY_OUTSIDE_INLINE)
    }
}

impl From<DisplayOutside> for Display {
    fn from(value: DisplayOutside) -> Self {
        Self(((value as u8) << Display::DISPLAY_OUTSIDE_SHIFT) | Display::DISPLAY_OUTSIDE_INLINE)
    }
}

#[repr(u8)]
pub enum DisplayInside {
    Flow = Display::DISPLAY_INSIDE_FLOW,
    FlowRoot = Display::DISPLAY_INSIDE_FLOW_ROOT,
    Table = Display::DISPLAY_INSIDE_TABLE,
    Flex = Display::DISPLAY_INSIDE_FLEX,
    Grid = Display::DISPLAY_INSIDE_GRID,
    Ruby = Display::DISPLAY_INSIDE_RUBY
}

#[repr(u8)]
pub enum DisplayOutside {
    Block = Display::DISPLAY_OUTSIDE_BLOCK,
    Inline = Display::DISPLAY_OUTSIDE_INLINE,
    RunIn = Display::DISPLAY_OUTSIDE_RUN_IN
}

#[repr(u8)]
pub enum DisplayInternal {
    TableRowGroup = Display::DISPLAY_INTERNAL_TABLE_ROW_GROUP,
    TableHeaderGroup = Display::DISPLAY_INTERNAL_TABLE_HEADER_GROUP,
    TableFooterGroup = Display::DISPLAY_INTERNAL_TABLE_FOOTER_GROUP,
    TableRow = Display::DISPLAY_INTERNAL_TABLE_ROW,
    TableCell = Display::DISPLAY_INTERNAL_TABLE_CELL,
    TableColumnGroup = Display::DISPLAY_INTERNAL_TABLE_COLUMN_GROUP,
    TableColumn = Display::DISPLAY_INTERNAL_TABLE_COLUMN,
    TableCaption = Display::DISPLAY_INTERNAL_TABLE_CAPTION,
    RubyBase = Display::DISPLAY_INTERNAL_RUBY_BASE,
    RubyText = Display::DISPLAY_INTERNAL_RUBY_TEXT,
    RubyBaseContainer = Display::DISPLAY_INTERNAL_RUBY_BASE_CONTAINER,
    RubyTextContainer = Display::DISPLAY_INTERNAL_RUBY_TEXT_CONTAINER
}

#[repr(u8)]
pub enum DisplayBox {
    Contents = Display::DISPLAY_BOX_CONTENTS,
    None = Display::DISPLAY_NONE_CONTENTS
}

pub enum DisplayLegacy {
    InlineBlock, // inline flow-root
    InlineTable, // inline table
    InlineFlex, // inline flex
    InlineGrid // inline grid
}