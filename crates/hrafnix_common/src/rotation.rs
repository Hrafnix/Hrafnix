/// A rotation constrained to quarter-turn increments.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Rotation {
    /// No rotation.
    #[default]
    Degrees0 = 0,
    /// A 90-degree rotation.
    Degrees90 = 90,
    /// A 180-degree rotation.
    Degrees180 = 180,
    /// A 270-degree rotation.
    Degrees270 = 270,
}

impl Rotation {
    /// Returns the rotation in degrees.
    #[must_use]
    pub const fn degrees(self) -> u16 {
        match self {
            Self::Degrees0 => 0,
            Self::Degrees90 => 90,
            Self::Degrees180 => 180,
            Self::Degrees270 => 270,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Rotation;

    #[test]
    fn quarter_turns_return_their_degrees() {
        for (rotation, degrees) in [
            (Rotation::Degrees0, 0),
            (Rotation::Degrees90, 90),
            (Rotation::Degrees180, 180),
            (Rotation::Degrees270, 270),
        ] {
            assert_eq!(rotation.degrees(), degrees);
        }
    }

    #[test]
    fn default_is_unrotated() {
        assert_eq!(Rotation::default(), Rotation::Degrees0);
    }
}
