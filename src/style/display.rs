//! Implements [CSS DISPLAY 3](https://www.w3.org/TR/css-display-3/)

pub mod initial {
    pub use super::Display;
}

pub mod computed {
    pub use super::Display;
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Display(u16);

impl std::ops::BitOr<Display> for Display {
    type Output = Display;

    fn bitor(self, rhs: Display) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd<Display> for Display {
    type Output = Display;

    fn bitand(self, rhs: Display) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl Default for Display {
    fn default() -> Self {
        Self::inline()
    }
}

impl Display {
    const DISPLAY_MODE_MASK: u8 = 0b11;

    const DISPLAY_INSIDE_OUTSIDE: u8 = 0b00;
    const DISPLAY_INTERNAL: u8 = 0b1;
    const DISPLAY_BOX: u8 = 0b10;
    const DISPLAY_LISTITEM: u8 = 0b11;

    const DISPLAY_OUTSIDE_SHIFT: u8 = 2;
    const DISPLAY_OUTSIDE_MASK: u16 = 0b1100;
    const DISPLAY_OUTSIDE_BLOCK: u8 = 0b1;
    const DISPLAY_OUTSIDE_INLINE: u8 = 0b10;
    const DISPLAY_OUTSIDE_RUN_IN: u8 = 0b11;

    const DISPLAY_INSIDE_SHIFT: u8 = 4;
    const DISPLAY_INSIDE_MASK: u16 = 0b1110000;
    const DISPLAY_INSIDE_FLOW: u8 = 0b1;
    const DISPLAY_INSIDE_FLOW_ROOT: u8 = 0b10;
    const DISPLAY_INSIDE_FLEX: u8 = 0b11;
    const DISPLAY_INSIDE_GRID: u8 = 0b100;
    const DISPLAY_INSIDE_RUBY: u8 = 0b101;
    const DISPLAY_INSIDE_TABLE: u8 = 0b110;

    const DISPLAY_INTERNAL_SHIFT: u8 = 2;
    const DISPLAY_INTERNAL_MASK: u16 = 0b111100;
    const DISPLAY_INTERNAL_TABLE_ROW_GROUP: u8 = 0b1;
    const DISPLAY_INTERNAL_TABLE_HEADER_GROUP: u8 = 0b10;
    const DISPLAY_INTERNAL_TABLE_FOOTER_GROUP: u8 = 0b11;
    const DISPLAY_INTERNAL_TABLE_ROW: u8 = 0b100;
    const DISPLAY_INTERNAL_TABLE_CELL: u8 = 0b101;
    const DISPLAY_INTERNAL_TABLE_COLUMN_GROUP: u8 = 0b110;
    const DISPLAY_INTERNAL_TABLE_COLUMN: u8 = 0b111;
    const DISPLAY_INTERNAL_TABLE_CAPTION: u8 = 0b1000;
    const DISPLAY_INTERNAL_RUBY_BASE: u8 = 0b1001;
    const DISPLAY_INTERNAL_RUBY_TEXT: u8 = 0b1010;
    const DISPLAY_INTERNAL_RUBY_BASE_CONTAINER: u8 = 0b1011;
    const DISPLAY_INTERNAL_RUBY_TEXT_CONTAINER: u8 = 0b1100;

    const DISPLAY_BOX_SHIFT: u8 = 2;
    const DISPLAY_BOX_MASK: u16 = 0b0011;
    const DISPLAY_BOX_CONTENTS: u8 = 0b1;
    const DISPLAY_NONE_CONTENTS: u8 = 0b10;

    fn kind(&self) -> DisplayKind {
        match (self.0 as u8) & Self::DISPLAY_MODE_MASK {
            Self::DISPLAY_INSIDE_OUTSIDE => DisplayKind::InsideOutside,
            Self::DISPLAY_LISTITEM => DisplayKind::Listitem,
            Self::DISPLAY_INTERNAL => DisplayKind::Internal,
            Self::DISPLAY_BOX => DisplayKind::Box,
            _ => unreachable!()
        }
    }

    pub fn is_block_box(&self) -> bool {
        self.outer() == Some(DisplayOutside::Block) && self.inner() == Some(DisplayInside::Flow)
    }

    pub fn is_inline_box(&self) -> bool {
        self.outer() == Some(DisplayOutside::Inline) && self.inner() == Some(DisplayInside::Flow)
    }

    /// Get the outer display, if any
    pub fn outer(&self) -> Option<DisplayOutside> {
        matches!(self.kind(), DisplayKind::InsideOutside)
        .then(|| {
            let raw = ((self.0 & Self::DISPLAY_OUTSIDE_MASK) >> Self::DISPLAY_OUTSIDE_SHIFT) as u8;
            if raw == 0 {
                None
            } else {
                DisplayOutside::try_from(raw).ok()
            }
        }).flatten()
    }

    pub fn set_outer(&mut self, outer: DisplayOutside) {
        self.0 |= outer.into_display().0
    }

    /// Get the inner display
    /// 
    /// ```spec
    /// If a <display-outside> value is specified but <display-inside> is omitted, 
    /// the element’s inner display type defaults to flow.
    /// ```
    pub fn inner(&self) -> Option<DisplayInside> {
        let inner = matches!(self.kind(), DisplayKind::InsideOutside)
        .then(|| {
            let raw = ((self.0 & Self::DISPLAY_INSIDE_MASK) >> Self::DISPLAY_OUTSIDE_SHIFT) as u8;
            if raw == 0 {
                None
            } else {
                DisplayInside::try_from(raw).ok()
            }
        }).flatten();

        if self.outer().is_some() && inner.is_none() {
            return Some(DisplayInside::Flow)
        }

        inner
    }

    pub fn set_inner(&mut self, inner: DisplayInside) {
        self.0 |= inner.into_display().0
    }

    /// Get the internal display
    pub fn internal(&self) -> Option<DisplayInternal> {
        matches!(self.kind(), DisplayKind::Internal)
        .then(|| {
            let raw = ((self.0 & Self::DISPLAY_INTERNAL_MASK) >> Self::DISPLAY_INTERNAL_SHIFT) as u8;
            if raw == 0 {
                None
            } else {
                DisplayInternal::try_from(raw).ok()
            }
        }).flatten()        
    }

    /// Get the box display
    pub fn r#box(&self) -> Option<DisplayBox> {
        matches!(self.kind(), DisplayKind::Box)
        .then(|| {
            let raw = ((self.0 & Self::DISPLAY_BOX_MASK) >> Self::DISPLAY_BOX_SHIFT) as u8;
            if raw == 0 {
                None
            } else {
                DisplayBox::try_from(raw).ok()
            }
        }).flatten()
    }

    pub fn listitem(&self) -> Option<DisplayListitem> {
        matches!(self.kind(), DisplayKind::Listitem)
        .then(|| {
            let inner = self.inner();
            if inner == Some(DisplayInside::Flow) {
                return Some(DisplayListitem(self.0))
            } else if inner == Some(DisplayInside::FlowRoot) {
                return Some(DisplayListitem(self.0))
            } else {
                return None
            }
        })
        .flatten()
    }

    const fn flow() -> Self {
        DisplayInside::Flow.into_display()
    } 
    const fn flow_root() -> Self {
        DisplayInside::FlowRoot.into_display()
    }
    const fn table() -> Self {
        DisplayInside::Table.into_display()
    }
    const fn flex() -> Self {
        DisplayInside::Flex.into_display()
    }
    const fn grid() -> Self {
        DisplayInside::Grid.into_display()
    }
    const fn ruby() -> Self {
        DisplayInside::Ruby.into_display()
    }
    const fn block() -> Self {
        DisplayOutside::Block.into_display()
    }
    const fn inline() -> Self {
        DisplayOutside::Inline.into_display()
    }
    const fn run_in() -> Self {
        DisplayOutside::RunIn.into_display()
    }

}

pub struct DisplayListitem(u16);

impl DisplayListitem {
    pub fn outer(&self) -> Option<DisplayOutside> {
        let raw = ((self.0 & Display::DISPLAY_OUTSIDE_MASK) >> Display::DISPLAY_OUTSIDE_SHIFT) as u8;
        if raw == 0 {
            None
        } else {
            DisplayOutside::try_from(raw).ok()
        }
    }
}

#[repr(u8)]
pub enum DisplayKind {
    InsideOutside = Display::DISPLAY_INSIDE_OUTSIDE,
    Listitem = Display::DISPLAY_LISTITEM,
    Internal = Display::DISPLAY_INTERNAL,
    Box = Display::DISPLAY_BOX
}

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
/// <display-inside>   = flow | flow-root | table | flex | grid | ruby
pub enum DisplayInside {
    ///
    /// ```spec
    /// The element lays out its contents using flow layout (block-and-inline layout).
    ///
    /// If its outer display type is inline or run-in, 
    /// and it is participating in a block or inline formatting context, 
    /// then it generates an inline box.
    ///
    /// Otherwise it generates a block container box.
    ///
    /// Depending on the value of other properties (such as position, float, or overflow) 
    /// and whether it is itself participating in a block or inline formatting context, 
    /// it either establishes a new block formatting context for its contents 
    /// or integrates its contents into its parent formatting context. 
    /// See CSS2.1 Chapter 9. [CSS2] A block container that establishes a new block formatting context is considered to have a used inner display type of flow-root.
    /// ```
    Flow = Display::DISPLAY_INSIDE_FLOW,
    ///
    /// ```spec
    /// The element generates a block container box, and lays out its contents using flow layout. 
    /// It always establishes a new block formatting context for its contents. [CSS2]
    /// ````
    FlowRoot = Display::DISPLAY_INSIDE_FLOW_ROOT,
    /// ```spec
    /// The element generates a principal table wrapper box that establishes a block formatting context, and which contains an additionally-generated table grid box that establishes a table formatting context. [CSS2]
    /// ```
    Table = Display::DISPLAY_INSIDE_TABLE,
    /// ```spec
    /// The element generates a principal flex container box and establishes a flex formatting context. [CSS-FLEXBOX-1]
    /// ```
    Flex = Display::DISPLAY_INSIDE_FLEX,
    /// ```spec
    /// The element generates a principal grid container box, and establishes a grid formatting context. [CSS-GRID-1]
    ///
    /// (Grids using subgrid might not generate a new grid formatting context; see [CSS-GRID-2] for details.)
    /// ```
    Grid = Display::DISPLAY_INSIDE_GRID,
    Ruby = Display::DISPLAY_INSIDE_RUBY
}

impl TryFrom<u8> for DisplayInside {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            Display::DISPLAY_INSIDE_FLOW => Ok(Self::Flow),
            Display::DISPLAY_INSIDE_FLOW_ROOT => Ok(Self::FlowRoot),
            Display::DISPLAY_INSIDE_TABLE => Ok(Self::Table),
            Display::DISPLAY_INSIDE_FLEX => Ok(Self::Flex),
            Display::DISPLAY_INSIDE_GRID => Ok(Self::Grid),
            Display::DISPLAY_INSIDE_RUBY => Ok(Self::Ruby),
            _ => Err(())
        }
    }
}

impl DisplayInside {
    const fn into_u16(self) -> u16 {
        self as u16
    }

    const fn into_display(self) -> Display {
        Display((self.into_u16() << Display::DISPLAY_INSIDE_SHIFT as u16) | Display::DISPLAY_OUTSIDE_INLINE as u16)
    }
}

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
/// <display-outside>  = block | inline | run-in
pub enum DisplayOutside {
    Block = Display::DISPLAY_OUTSIDE_BLOCK,
    Inline = Display::DISPLAY_OUTSIDE_INLINE,
    RunIn = Display::DISPLAY_OUTSIDE_RUN_IN
}

impl TryFrom<u8> for DisplayOutside {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            Display::DISPLAY_OUTSIDE_BLOCK => Ok(Self::Block),
            Display::DISPLAY_OUTSIDE_INLINE => Ok(Self::Inline),
            Display::DISPLAY_OUTSIDE_RUN_IN => Ok(Self::RunIn),
            _ => Err(())
        }
    }
}

impl DisplayOutside {
    const fn into_u16(self) -> u16 {
        self as u16
    }

    const fn into_display(self) -> Display {
        Display((self.into_u16() << Display::DISPLAY_OUTSIDE_SHIFT as u16) | Display::DISPLAY_OUTSIDE_INLINE as u16)
    }
}

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
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

impl DisplayInternal {
    const fn into_u16(self) -> u16 {
        self as u16
    }

    const fn into_display(self) -> Display {
        Display((self.into_u16() << Display::DISPLAY_INTERNAL_SHIFT as u16) | Display::DISPLAY_INTERNAL as u16)
    }
}

impl TryFrom<u8> for DisplayInternal {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            Display::DISPLAY_INTERNAL_TABLE_ROW_GROUP => Ok(Self::TableRowGroup),
            Display::DISPLAY_INTERNAL_TABLE_HEADER_GROUP => Ok(Self::TableHeaderGroup),
            Display::DISPLAY_INTERNAL_TABLE_FOOTER_GROUP => Ok(Self::TableFooterGroup),
            Display::DISPLAY_INTERNAL_TABLE_ROW => Ok(Self::TableRow),
            Display::DISPLAY_INTERNAL_TABLE_CELL => Ok(Self::TableCell),
            Display::DISPLAY_INTERNAL_TABLE_COLUMN_GROUP => Ok(Self::TableColumnGroup),
            Display::DISPLAY_INTERNAL_TABLE_COLUMN => Ok(Self::TableColumn),
            Display::DISPLAY_INTERNAL_TABLE_CAPTION => Ok(Self::TableCaption),
            Display::DISPLAY_INTERNAL_RUBY_BASE => Ok(Self::RubyBase),
            Display::DISPLAY_INTERNAL_RUBY_TEXT => Ok(Self::RubyText),
            Display::DISPLAY_INTERNAL_RUBY_BASE_CONTAINER => Ok(Self::RubyBaseContainer),
            Display::DISPLAY_INTERNAL_RUBY_TEXT_CONTAINER => Ok(Self::RubyTextContainer),
            _ => Err(())
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DisplayBox {
    Contents = Display::DISPLAY_BOX_CONTENTS,
    None = Display::DISPLAY_NONE_CONTENTS
}

impl DisplayBox {
    const fn into_u16(self) -> u16 {
        self as u16
    }

    const fn into_display(self) -> Display {
        Display((self.into_u16() << Display::DISPLAY_BOX_SHIFT as u16) | Display::DISPLAY_BOX as u16)
    }
}


impl TryFrom<u8> for DisplayBox {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            Display::DISPLAY_BOX_CONTENTS => Ok(Self::Contents),
            Display::DISPLAY_NONE_CONTENTS => Ok(Self::None),
            _ => Err(())
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DisplayLegacy {
    InlineBlock, // inline flow-root
    InlineTable, // inline table
    InlineFlex, // inline flex
    InlineGrid // inline grid
}

impl DisplayLegacy {
    const fn into_display(self) -> Display {
        match self {
            DisplayLegacy::InlineBlock => Display(DisplayInside::FlowRoot.into_u16() | DisplayOutside::Block.into_u16()),
            DisplayLegacy::InlineTable => Display(DisplayInside::Table.into_u16() | DisplayOutside::Inline.into_u16()),
            DisplayLegacy::InlineFlex => Display(DisplayInside::Flex.into_u16() | DisplayOutside::Inline.into_u16()),
            DisplayLegacy::InlineGrid => Display(DisplayInside::Grid.into_u16() | DisplayOutside::Inline.into_u16()),
        }
    }
}