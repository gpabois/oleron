use std::ops::Deref;

#[derive(Clone, Copy, Default)]
pub struct Integer(i32);

impl Deref for Integer {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy)]
pub struct Number(f32);

impl Deref for Number {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy)]
pub struct Percentage(f32);

impl Deref for Percentage {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy)]
pub enum AutoOrLengthOrPercentage {
    Auto,
    Length(Length),
    Percentage(Percentage),
}

impl AutoOrLengthOrPercentage {
    pub fn zero() -> Self {
        AutoOrLengthOrPercentage::Length(Length::px(0))
    }
}

pub enum NumberOrPercentage {
    Number(Number),
    Percentage(Percentage),
}

pub enum NumberOrAngle {
    Number(Number),
    Angle(Angle<f32>),
}

#[derive(Clone, Copy)]
pub struct Dimension<Numeric, Unit> {
    pub value: Numeric,
    pub unit: Unit,
}

pub type Length = Dimension<f64, LengthUnit>;

impl Length {
    pub fn px(value: i64) -> Self {
        Self {
            value: value as f64,
            unit: LengthUnit::Px,
        }
    }
}

#[derive(Clone, Copy)]
pub enum LengthUnit {
    Em,
    Ex,
    Ch,
    Rem,
    Vw,
    Vh,
    Vmin,
    Vmax,
    Cm,
    Mm,
    Q,
    In,
    Pt,
    Pc,
    Px,
}

impl LengthUnit {
    pub fn is_relative(&self) -> bool {
        matches!(
            self,
            LengthUnit::Em
                | LengthUnit::Ex
                | LengthUnit::Ch
                | LengthUnit::Vw
                | LengthUnit::Vh
                | LengthUnit::Vmin
                | LengthUnit::Vmax
        )
    }

    pub fn is_absolute(&self) -> bool {
        matches!(
            self,
            Self::Cm | Self::Mm | Self::Q | Self::In | Self::Pt | Self::Pc | Self::Px
        )
    }
}

pub enum AngleUnit {
    Deg,
    Grad,
    Rad,
    Turn,
}

pub type Angle<Numeric> = Dimension<Numeric, AngleUnit>;

pub enum FrequencyUnit {
    Hz,
    KHz,
}

pub type Frequency<Numeric> = Dimension<Numeric, FrequencyUnit>;

pub enum ResolutionUnit {
    Dpi,
    Dpcm,
    Dppx,
}

pub type Resolution<Numeric> = Dimension<Numeric, ResolutionUnit>;

