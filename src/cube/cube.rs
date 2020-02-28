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

/// A column of cubies. Each column has a top, bottom, and center, though not
/// every column will have a center. To represent this, the center field uses
/// the type Option<&'a Box<dyn Cubie>>.
pub struct Column<'a> {
    pub center: Option<&'a Box<dyn Cubie>>,
    pub top: &'a Box<dyn Cubie>,
    pub bottom: &'a Box<dyn Cubie>,
}

impl<'a> Column<'a> {
    pub const fn new(
        top: &'a Box<dyn Cubie>,
        center: Option<&'a Box<dyn Cubie>>,
        bottom: &'a Box<dyn Cubie>,
    ) -> Self {
        Column {
            top,
            center,
            bottom,
        }
    }
}

pub enum CornerPosition {
    TopBackLeft,
    TopBackRight,
    TopFrontLeft,
    TopFrontRight,
    BottomBackLeft,
    BottomBackRight,
    BottomFrontLeft,
    BottomFrontRight,
}

pub enum ColumnPosition {
    BackLeft,
    BackMiddle,
    BackRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    FrontLeft,
    FrontMiddle,
    FrontRight,
}

pub enum RowPosition {
    TopBack,
    TopCenter,
    TopFront,
    MiddleBack,
    MiddleCenter,
    MiddleFront,
    BottomBack,
    BottomCenter,
    BottomFront,
}

/// A 3x3 Twisty Puzzle Cube
///
/// Models each individual "subcube" known as a cubie and provides methods to
/// manage its data.
///
/// # Examples
/// ```
/// use rustytwisty::cube::{cube::*, cubie::*};
///
/// let cube = Cube::new();
///
/// let row = Cube::row(&cube, RowPosition::TopCenter);
/// let column = Cube::column(&cube, ColumnPosition::MiddleRight);
/// let corner = Cube::corner(&cube, CornerPosition::TopBackRight);
/// ```
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

    pub const fn corner_raw(&self, pos: usize) -> &Box<dyn Cubie> {
        match pos {
            0 => &self.elements[0],
            1 => &self.elements[2],
            2 => &self.elements[6],
            3 => &self.elements[8],
            4 => &self.elements[17],
            5 => &self.elements[19],
            6 => &self.elements[23],
            7 => &self.elements[25],
            _ => panic!("Exceeded corner count"),
        }
    }

    pub const fn corner(&self, pos: CornerPosition) -> &Box<dyn Cubie> {
        match pos {
            CornerPosition::TopBackLeft => Cube::corner_raw(&self, 0),
            CornerPosition::TopBackRight => Cube::corner_raw(&self, 1),
            CornerPosition::TopFrontLeft => Cube::corner_raw(&self, 2),
            CornerPosition::TopFrontRight => Cube::corner_raw(&self, 3),
            CornerPosition::BottomBackLeft => Cube::corner_raw(&self, 4),
            CornerPosition::BottomBackRight => Cube::corner_raw(&self, 5),
            CornerPosition::BottomFrontLeft => Cube::corner_raw(&self, 6),
            CornerPosition::BottomFrontRight => Cube::corner_raw(&self, 7),
        }
    }

    pub const fn corners(&self) -> [&Box<dyn Cubie>; 8] {
        [
            Cube::corner_raw(&self, 0),
            Cube::corner_raw(&self, 1),
            Cube::corner_raw(&self, 2),
            Cube::corner_raw(&self, 3),
            Cube::corner_raw(&self, 4),
            Cube::corner_raw(&self, 5),
            Cube::corner_raw(&self, 6),
            Cube::corner_raw(&self, 7),
        ]
    }

    pub const fn row_raw(&self, pos: usize) -> Row {
        match pos {
            0 => Row {
                left: &self.elements[0],
                center: Some(&self.elements[1]),
                right: &self.elements[2],
            },
            1 => Row {
                left: &self.elements[3],
                center: Some(&self.elements[4]),
                right: &self.elements[5],
            },
            2 => Row {
                left: &self.elements[6],
                center: Some(&self.elements[7]),
                right: &self.elements[8],
            },
            3 => Row {
                left: &self.elements[9],
                center: Some(&self.elements[10]),
                right: &self.elements[11],
            },
            4 => Row {
                left: &self.elements[12],
                center: None,
                right: &self.elements[13],
            },
            5 => Row {
                left: &self.elements[14],
                center: Some(&self.elements[15]),
                right: &self.elements[16],
            },
            6 => Row {
                left: &self.elements[17],
                center: Some(&self.elements[18]),
                right: &self.elements[19],
            },
            7 => Row {
                left: &self.elements[20],
                center: Some(&self.elements[21]),
                right: &self.elements[22],
            },
            8 => Row {
                left: &self.elements[23],
                center: Some(&self.elements[24]),
                right: &self.elements[25],
            },
            _ => panic!("Exceeded row count"),
        }
    }

    pub const fn row(&self, pos: RowPosition) -> Row {
        match pos {
            RowPosition::TopBack => Cube::row_raw(&self, 0),
            RowPosition::TopCenter => Cube::row_raw(&self, 1),
            RowPosition::TopFront => Cube::row_raw(&self, 2),
            RowPosition::MiddleBack => Cube::row_raw(&self, 3),
            RowPosition::MiddleCenter => Cube::row_raw(&self, 4),
            RowPosition::MiddleFront => Cube::row_raw(&self, 5),
            RowPosition::BottomBack => Cube::row_raw(&self, 6),
            RowPosition::BottomCenter => Cube::row_raw(&self, 7),
            RowPosition::BottomFront => Cube::row_raw(&self, 8),
        }
    }

    pub const fn rows(&self) -> [Row; 9] {
        [
            Cube::row_raw(&self, 0),
            Cube::row_raw(&self, 1),
            Cube::row_raw(&self, 2),
            Cube::row_raw(&self, 3),
            Cube::row_raw(&self, 4),
            Cube::row_raw(&self, 5),
            Cube::row_raw(&self, 6),
            Cube::row_raw(&self, 7),
            Cube::row_raw(&self, 8),
        ]
    }

    pub const fn column_raw(&self, pos: usize) -> Column {
        match pos {
            0 => Column {
                top: &self.elements[0],
                center: Some(&self.elements[9]),
                bottom: &self.elements[18],
            },
            1 => Column {
                top: &self.elements[1],
                center: Some(&self.elements[10]),
                bottom: &self.elements[19],
            },
            2 => Column {
                top: &self.elements[2],
                center: Some(&self.elements[11]),
                bottom: &self.elements[20],
            },
            3 => Column {
                top: &self.elements[3],
                center: Some(&self.elements[12]),
                bottom: &self.elements[21],
            },
            4 => Column {
                top: &self.elements[4],
                center: None,
                bottom: &self.elements[21],
            },
            5 => Column {
                top: &self.elements[5],
                center: Some(&self.elements[13]),
                bottom: &self.elements[22],
            },
            6 => Column {
                top: &self.elements[6],
                center: Some(&self.elements[14]),
                bottom: &self.elements[23],
            },
            7 => Column {
                top: &self.elements[7],
                center: Some(&self.elements[15]),
                bottom: &self.elements[24],
            },
            8 => Column {
                top: &self.elements[8],
                center: Some(&self.elements[16]),
                bottom: &self.elements[25],
            },
            _ => panic!("Exceeded column count"),
        }
    }

    pub const fn column(&self, pos: ColumnPosition) -> Column {
        match pos {
            ColumnPosition::BackLeft => Cube::column_raw(&self, 0),
            ColumnPosition::BackMiddle => Cube::column_raw(&self, 1),
            ColumnPosition::BackRight => Cube::column_raw(&self, 2),
            ColumnPosition::MiddleLeft => Cube::column_raw(&self, 3),
            ColumnPosition::MiddleCenter => Cube::column_raw(&self, 4),
            ColumnPosition::MiddleRight => Cube::column_raw(&self, 5),
            ColumnPosition::FrontLeft => Cube::column_raw(&self, 6),
            ColumnPosition::FrontMiddle => Cube::column_raw(&self, 7),
            ColumnPosition::FrontRight => Cube::column_raw(&self, 8),
        }
    }

    pub const fn columns(&self) -> [Column; 9] {
        [
            Cube::column_raw(&self, 0),
            Cube::column_raw(&self, 1),
            Cube::column_raw(&self, 2),
            Cube::column_raw(&self, 3),
            Cube::column_raw(&self, 4),
            Cube::column_raw(&self, 5),
            Cube::column_raw(&self, 6),
            Cube::column_raw(&self, 7),
            Cube::column_raw(&self, 8),
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
