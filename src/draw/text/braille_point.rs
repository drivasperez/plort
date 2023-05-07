/// Braille characters in Unicode are represented by a single byte.
/// Each bit in the byte represents a dot in the 2x4 matrix. The numbering
/// of the dots is as follows:
///
/// 1 4
/// 2 5
/// 3 6
/// 7 8
///
/// So, for the following braille character: ⠓  
/// Dots 1, 2 and 5 are raised, so the byte representation is:
/// 0001 0011
///
/// This can then be added to 0x2800 to get the Unicode code point.

/// A struct containing a 2x4 matrix of chart points,
/// which can be represented by a single braille character.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BraillePoint {
    value: u8,
}

impl BraillePoint {
    /// Set the value of a dot in the matrix.
    /// x and y are zero-indexed, and start from the bottom left.
    pub fn set(&mut self, x: usize, y: usize) {
        let offset = match (x, y) {
            (0, 0) => 7,
            (1, 0) => 8,
            (0, 1) => 3,
            (1, 1) => 6,
            (0, 2) => 2,
            (1, 2) => 5,
            (0, 3) => 1,
            (1, 3) => 4,
            _ => panic!("Invalid x and y values"),
        } - 1;

        self.value |= 1 << offset;
    }
}

impl From<u8> for BraillePoint {
    fn from(value: u8) -> Self {
        Self { value }
    }
}

impl From<BraillePoint> for char {
    fn from(point: BraillePoint) -> Self {
        std::char::from_u32(0x2800 + point.value as u32).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set() {
        let mut point = BraillePoint::default();
        point.set(0, 0);
        assert_eq!(point.value, 0b0100_0000);
        let mut point = BraillePoint::default();
        point.set(1, 0);
        assert_eq!(point.value, 0b1000_0000);
        let mut point = BraillePoint::default();
        point.set(0, 1);
        assert_eq!(point.value, 0b0000_0100);
        let mut point = BraillePoint::default();
        point.set(1, 1);
        assert_eq!(point.value, 0b0010_0000);
        let mut point = BraillePoint::default();
        point.set(0, 2);
        assert_eq!(point.value, 0b0000_0010);
        let mut point = BraillePoint::default();
        point.set(1, 2);
        assert_eq!(point.value, 0b0001_0000);
        let mut point = BraillePoint::default();
        point.set(0, 3);
        assert_eq!(point.value, 0b0000_0001);
        let mut point = BraillePoint::default();
        point.set(1, 3);
        assert_eq!(point.value, 0b0000_1000);
    }

    #[test]
    #[should_panic]
    fn test_set_x_invalid() {
        let mut point = BraillePoint::default();
        point.set(2, 0);
    }

    #[test]
    #[should_panic]
    fn test_set_y_invalid() {
        let mut point = BraillePoint::default();
        point.set(0, 4);
    }

    #[test]
    fn test_from_u8() {
        let point = BraillePoint::from(0b0000_0001);
        assert_eq!(point.value, 0b0000_0001);
    }

    #[test]
    fn test_from_braille_point() {
        let point = BraillePoint::from(0b0001_0011);
        let char = char::from(point);
        assert_eq!(char, '⠓');
    }
}
