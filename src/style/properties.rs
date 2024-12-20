pub mod initial {
    use crate::style::{display, order, visibility};

    #[derive(Default, Clone)]
    pub struct Properties {
        pub display: display::initial::Display,
        pub order: order::initial::Order,
        pub visibility: visibility::initial::Visibility
    }
}

pub mod computed {
    use crate::style::{display, order, visibility};

    pub struct Properties {
        pub display: display::computed::Display,
        pub order: order::computed::Order,
        pub visibility: visibility::computed::Visibility
    }
}

pub mod used {
    pub struct Properties {
        
    }
}