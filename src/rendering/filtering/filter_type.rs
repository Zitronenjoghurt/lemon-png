#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum FilterType {
    #[default]
    None = 0,
    Sub = 1,
    Up = 2,
    Average = 3,
    Paeth = 4,
}

impl TryFrom<u8> for FilterType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Sub),
            2 => Ok(Self::Up),
            3 => Ok(Self::Average),
            4 => Ok(Self::Paeth),
            _ => Err(()),
        }
    }
}
