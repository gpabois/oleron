pub enum Color {
    ColorBase(ColorBase),
    CurrentColor,
    SystemColor(SystemColor),
    DeviceCmyk(DeviceCmyk),
    LightDark(LightDark),
}

pub enum ColorBase {
    HexColor(HexColor),
    ColorFunction(ColorFunction),
    NamedColor(NamedColor),
    ColorMix(ColorMix),
}

pub struct HexColor(String);

pub enum ColorFunction {
    Rgba(Rgba),
    Hsla(Hsla),
    Hwba(Hwba),
    Laba(Lab),
}

pub struct Lch {
    pub from: Option<Color>,
    pub lightness: Option<NumberOrPercentage>,
    pub chroma: Option<NumberOrPercentage>,
    pub hue: Option<NumberOrAngle>,
    pub alpha: Option<NumberOrPercentage>,
}

pub struct Lab {
    pub l: Option<NumberOrPercentage>,
    pub a: Option<NumberOrPercentage>,
    pub b: Option<NumberOrPercentage>,
    pub alpha: Option<NumberOrPercentage>,
}

pub struct Hwba {
    pub hue: Option<NumberOrAngle>,
    pub whiteness: Option<Percentage>,
    pub blackness: Option<Percentage>,
    pub alpha: Option<Percentage>,
}

pub struct Hsla {
    pub hue: Option<Angle>,
    pub saturation: Option<Percentage>,
    pub lightness: Option<Percentage>,
    pub alpha: Option<Percentage>,
}

pub struct Rgba {
    pub red: Option<NumberOrPercentage>,
    pub blue: Option<NumberOrPercentage>,
    pub green: Option<NumberOrPercentage>,
    pub alpha: Option<Percentage>,
}

pub struct NamedColor;

pub struct SystemColor;

pub struct DeviceCmyk;

pub struct LightDark;

