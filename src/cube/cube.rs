use crate::cube::cubie::Cubie;

pub struct Face<'a> {
    pub elements: [&'a Box<dyn Cubie>; 9],
}

impl<'a> Face<'a> {
    pub const fn new_from_array(arr: [&'a Box<dyn Cubie>; 9]) -> Self {
        Self {
            elements: arr,
        }
    }
}

pub enum FaceKind {
    Top,
    Left,
    Right,
    Front,
    Back,
    Bottom,
}

/// A row of cubies. Each row has a left, right, and center, though the center
/// middle row has the turning mechanism instead. For this reason, center is an
/// Option<&'a Box<dyn Cubie>>.
pub struct Row<'a> {
    pub center: Option<&'a Box<dyn Cubie>>,
    pub left: &'a Box<dyn Cubie>,
    pub right: &'a Box<dyn Cubie>,
}

impl<'a> Row<'a> {
    pub const fn new(
        left: &'a Box<dyn Cubie>,
        center: Option<&'a Box<dyn Cubie>>,
        right: &'a Box<dyn Cubie>,
    ) -> Self {
        Row {
            left,
            center,
            right,
        }
    }

    pub fn has_center(self) -> bool {
        self.center.is_some()
    }
}

pub struct Cube {
    pub elements: [Box<dyn Cubie>; 26],
}

#[macro_use]
macro_rules! initialize_cube_face {
    ($o:expr, $x:expr) => {
        Face::new_from_array([
            &$o.elements[$x[0]],
            &$o.elements[$x[1]],
            &$o.elements[$x[2]],
            &$o.elements[$x[3]],
            &$o.elements[$x[4]],
            &$o.elements[$x[5]],
            &$o.elements[$x[6]],
            &$o.elements[$x[7]],
            &$o.elements[$x[8]],
        ])
    };
}

impl Cube {
    /// Initializes a 3x3 Cube with elements in the form of an array with
    /// elements in three slices in the following order:
    ///
    /// left to right, back to front, and top to bottom
    ///
    ///  Top     Middle    Bottom
    /// 0 1 2    9 10 11  17 18 19
    /// 3 4 5   12    13  20 21 22
    /// 6 7 8   14 15 16  23 24 25
    ///
    /// where 0 would be the top left corner cubie in the back.
    pub fn new() -> Self {
        Self {
            elements: [
                cubie!("corner"), // Top slice (9 cubies)
                cubie!("edge"),
                cubie!("corner"),
                cubie!("edge"),
                cubie!("center"),
                cubie!("edge"),
                cubie!("corner"),
                cubie!("edge"),
                cubie!("corner"),
                cubie!("edge"), // Middle slice (8 cubies)
                cubie!("center"),
                cubie!("edge"),
                cubie!("center"),
                cubie!("center"),
                cubie!("edge"),
                cubie!("center"),
                cubie!("edge"),
                cubie!("corner"), // Bottom slice (9 cubies)
                cubie!("edge"),
                cubie!("corner"),
                cubie!("edge"),
                cubie!("corner"),
                cubie!("edge"),
                cubie!("corner"),
                cubie!("edge"),
                cubie!("corner"),
            ],
        }
    }

    pub const fn corners(&self) -> [&Box<dyn Cubie>; 8] {
        [
            &self.elements[0],
            &self.elements[2],
            &self.elements[6],
            &self.elements[8],
            &self.elements[17],
            &self.elements[19],
            &self.elements[23],
            &self.elements[25],
        ]
    }

    pub const fn rows(&self) -> [Row; 9] {
        [
            Row {
                left: &self.elements[0],
                center: Some(&self.elements[1]),
                right: &self.elements[2],
            },
            Row {
                left: &self.elements[3],
                center: Some(&self.elements[4]),
                right: &self.elements[5],
            },
            Row {
                left: &self.elements[6],
                center: Some(&self.elements[7]),
                right: &self.elements[8],
            },
            Row {
                left: &self.elements[9],
                center: Some(&self.elements[10]),
                right: &self.elements[11],
            },
            Row {
                left: &self.elements[12],
                center: None,
                right: &self.elements[13],
            },
            Row {
                left: &self.elements[14],
                center: Some(&self.elements[15]),
                right: &self.elements[16],
            },
            Row {
                left: &self.elements[17],
                center: Some(&self.elements[18]),
                right: &self.elements[19],
            },
            Row {
                left: &self.elements[20],
                center: Some(&self.elements[21]),
                right: &self.elements[22],
            },
            Row {
                left: &self.elements[23],
                center: Some(&self.elements[24]),
                right: &self.elements[25],
            },
        ]
    }

    pub const fn face(&self, s: FaceKind) -> Face {
        match s {
            FaceKind::Top => {
                initialize_cube_face!(&self, [0, 1, 2, 3, 4, 5, 6, 7, 8])
            }
            FaceKind::Left => {
                initialize_cube_face!(&self, [0, 3, 6, 9, 12, 14, 17, 20, 23])
            }
            FaceKind::Right => {
                initialize_cube_face!(&self, [2, 5, 8, 11, 13, 16, 19, 22, 25])
            }
            FaceKind::Front => {
                initialize_cube_face!(&self, [6, 7, 8, 14, 15, 16, 23, 24, 25])
            }
            FaceKind::Back => {
                initialize_cube_face!(&self, [0, 1, 2, 9, 10, 11, 17, 18, 19])
            }
            FaceKind::Bottom => initialize_cube_face!(&self, [
                17, 18, 19, 20, 21, 22, 23, 24, 25
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::cubie::Corner as CornerCubie;

    #[test]
    fn cube_init() {
        let c = Cube::new();

        assert_eq!(c.elements.len(), 26);
    }

    #[test]
    fn get_face_array() {
        let c = Cube::new();

        let cf: Face = initialize_cube_face!(c, [0, 1, 2, 3, 4, 5, 6, 7, 8]);

        assert_eq!(cf.elements.len(), 9);
        let cubie: &Box<dyn Cubie> = &c.elements[0];
        let cornercubie: &CornerCubie =
            match cubie.as_any().downcast_ref::<CornerCubie>() {
                Some(i) => i,
                None => panic!("cubie isn't a CornerCubie!!"),
            };

        let cubie2: &Box<dyn Cubie> = cf.elements[0];
        let cornercubie2: &CornerCubie =
            match cubie2.as_any().downcast_ref::<CornerCubie>() {
                Some(i) => i,
                None => panic!("cubie2 isn't a CornerCubie!!"),
            };

        assert_eq!(cornercubie.faces, cornercubie2.faces);
    }
}
