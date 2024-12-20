pub mod initial {
    use crate::style::{display, order, visibility, margin, padding, border};

    #[derive(Default, Clone)]
    pub struct Properties {
        // CSS Display 3
        pub display: display::initial::Display,
        pub order: order::initial::Order,
        pub visibility: visibility::initial::Visibility,
        // CSS BOX 3
        pub margin: margin::initial::Margin,
        pub padding: padding::initial::Padding,
        pub border: border::initial::Border
    }
}

pub mod computed {
    use crate::style::{display, order, visibility, margin, padding, border};

    #[derive(Default, Clone)]
    pub struct Properties {
        // CSS Display 3
        pub display: display::computed::Display,
        pub order: order::computed::Order,
        pub visibility: visibility::computed::Visibility,
        // CSS BOX 3
        pub margin: margin::computed::Margin,
        pub padding: padding::computed::Padding,
        pub border: border::computed::Border
    }
}

pub mod used {
    pub struct Properties {
        
    }
}