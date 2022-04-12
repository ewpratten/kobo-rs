use std::fmt::Display;

/// Possible colors for use in graphics code
#[derive(Debug, Clone)]
pub enum EinkColor {
    Black,
    Gray1,
    Gray2,
    Gray3,
    Gray4,
    Gray5,
    Gray6,
    Gray7,
    Gray8,
    Gray9,
    GrayA,
    GrayB,
    GrayC,
    GrayD,
    GrayE,
    White,
}

impl Display for EinkColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EinkColor::Black => write!(f, "BLACK"),
            EinkColor::Gray1 => write!(f, "GRAY1"),
            EinkColor::Gray2 => write!(f, "GRAY2"),
            EinkColor::Gray3 => write!(f, "GRAY3"),
            EinkColor::Gray4 => write!(f, "GRAY4"),
            EinkColor::Gray5 => write!(f, "GRAY5"),
            EinkColor::Gray6 => write!(f, "GRAY6"),
            EinkColor::Gray7 => write!(f, "GRAY7"),
            EinkColor::Gray8 => write!(f, "GRAY8"),
            EinkColor::Gray9 => write!(f, "GRAY9"),
            EinkColor::GrayA => write!(f, "GRAYA"),
            EinkColor::GrayB => write!(f, "GRAYB"),
            EinkColor::GrayC => write!(f, "GRAYC"),
            EinkColor::GrayD => write!(f, "GRAYD"),
            EinkColor::GrayE => write!(f, "GRAYE"),
            EinkColor::White => write!(f, "WHITE"),
        }
    }
}
