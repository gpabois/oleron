#[derive(Default, Clone)]
pub struct BoxEdges<U> {
    pub top: U,
    pub bottom: U,
    pub left: U,
    pub right: U
}

impl<U: std::ops::Add<U, Output = U>> std::ops::Add<Self> for BoxEdges<U> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            top: self.top + rhs.top,
            bottom: self.bottom + rhs.bottom,
            left: self.left + rhs.left,
            right: self.right + rhs.right
        }
    }
}

#[derive(Default, Clone)]
pub struct BoxContent<U> {
    pub width: U,
    pub height: U
}

impl<U: std::ops::Add<U, Output = U>> std::ops::Add<Self> for BoxContent<U> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height
        }
    }
}

impl<U: std::ops::Add<U, Output = U>> std::ops::Add<BoxEdges<U>> for BoxContent<U> {
    type Output = Self;

    fn add(self, rhs: BoxEdges<U>) -> Self::Output {
        Self {
            width: self.width + rhs.left + rhs.right,
            height: self.height + rhs.top + rhs.bottom
        }
    }
}

#[derive(Default, Clone)]
pub struct Box<U> {
    pub content: BoxContent<U>,
    pub padding: BoxEdges<U>,
    pub margin: BoxEdges<U>,
    pub border: BoxEdges<U>
}

impl<U: std::ops::Add<U, Output = U>> std::ops::Add<Self> for Box<U> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            content: self.content + rhs.content,
            padding: self.padding + rhs.padding,
            margin: self.margin + rhs.margin,
            border: self.border + rhs.border

        }
    }
}

impl<U: std::ops::Add<U, Output = U> + Copy> Box<U> {
    pub fn outer(&self) -> BoxContent<U> {
        self.content.clone()
        + self.padding.clone()
        + self.margin.clone()
        + self.border.clone()
    }
}
