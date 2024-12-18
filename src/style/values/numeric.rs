
pub struct Integer(i32);
pub struct Number(f32);
pub struct Percentage(f32);

pub enum NumberOrPercentage {
    Number(Number),
    Percentage(Percentage)
}

pub enum NumberOrAngle {
    Number(Number),
    Angle(Angle)
}

pub struct Dimension<Numeric, Unit> {
    value: Numeric,
    unit: Unit
}

pub type Length<Numeric> = Dimension<Numeric, LengthUnit>;

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
    Px
}

impl LengthUnit {
    pub fn is_relative(&self) -> bool {
        matches!(self, LengthUnit::Em | LengthUnit::Ex | LengthUnit::Ch | LengthUnit::Vw | LengthUnit::Vh | LengthUnit::Vmin | LengthUnit::Vmax)
    }

    pub fn is_absolute(&self) -> bool {
        matches!(self, Self::Cm | Self::Mm | Self::Q | Self::In | Self::Pt | Self::Pc | Self::Px)
    }
}

pub enum AngleUnit {
    Deg,
    Grad,
    Rad,
    Turn
}

pub type Angle<Numeric> = Dimension<Numeric, AngleUnit>;

pub enum FrequencyUnit {
    Hz,
    KHz
}

pub type Frequency<Numeric> = Dimension<Numeric, FrequencyUnit>;

pub enum ResolutionUnit {
    Dpi,
    Dpcm,
    Dppx
} 

pub type Resolution<Numeric> = Dimension<Numeric, ResolutionUnit>;