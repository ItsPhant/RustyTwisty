use std::any::Any;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CubieColor {
    Blue,
    Green,
    Orange,
    Red,
    White,
    Yellow,
    Uninit,
}

impl CubieColor {
    pub fn new() -> Self {
        CubieColor::Uninit
    }
}

#[derive(Clone, Debug, Eq)]
pub struct CubieFace {
    pub color: CubieColor,
}

impl PartialEq for CubieFace {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}

impl CubieFace {
    pub fn new() -> Self {
        Self {
            color: CubieColor::new(),
        }
    }
}

/// A cubie is an individual "subcube" of a twisty cube puzzle. A cubie can have
/// from one to three colored faces depending on if it is a corner cubie (3
/// colored faces), edge cubie (2 colored faces), or center cubie (1 colored
/// face).
///
/// In order to allow cubies objects to be coerced to the type representing
/// their cubie kind, this trait provides a method which all cubie types
/// implement to cast it as Any.
///
/// # Examples
///
/// ```
/// let a: Box<dyn Cubie> = CenterCubie::new();
/// let b: &CenterCubie =
///     match a.as_any().downcast_ref::<CenterCubie>() {
///     Some(b) => b,
///     None => panic!("&b isn't a CenterCubie!!"),
/// };
/// ```
pub trait Cubie {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone, Debug, Eq)]
pub struct CenterCubie {
    pub faces: Vec<CubieFace>
}

impl PartialEq for CenterCubie {
    fn eq(&self, other: &Self) -> bool {
        self.faces == other.faces
    }
}

impl Cubie for CenterCubie {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CenterCubie {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            faces: vec![CubieFace::new(); 1]
        })
    }

    pub fn get_faces(&self) -> &Vec<CubieFace> {
        &self.faces
    }
}

#[derive(Clone, Debug, Eq)]
pub struct CornerCubie {
    pub faces: Vec<CubieFace>
}

impl PartialEq for CornerCubie {
    fn eq(&self, other: &Self) -> bool {
        self.faces == other.faces
    }
}

impl Cubie for CornerCubie {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CornerCubie {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            faces: vec![CubieFace::new(); 3]
        })
    }

    pub fn get_faces(&self) -> &Vec<CubieFace> {
        &self.faces
    }
}

#[derive(Clone, Debug, Eq)]
pub struct EdgeCubie {
    pub faces: Vec<CubieFace>
}

impl PartialEq for EdgeCubie {
    fn eq(&self, other: &Self) -> bool {
        self.faces == other.faces
    }
}

impl Cubie for EdgeCubie {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl EdgeCubie {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            faces: vec![CubieFace::new(); 2]
        })
    }

    pub fn get_faces(&self) -> &Vec<CubieFace> {
        &self.faces
    }
}

#[macro_export]
macro_rules! cubie {
    ($x:expr) => {
        match $x {
            "center" => CenterCubie::new(),
            "corner" => CornerCubie::new(),
            "edge" => EdgeCubie::new(),
            _ => panic!("Cubie type not found"),
        };
    }
}

pub struct CubeFace<'a> {
    pub elements: [&'a Box<dyn Cubie>; 9]
}

pub enum CubeFaceKind {
    Top,
    Left,
    Right,
    Front,
    Back,
    Bottom,
}

pub struct Cube {
    elements: [Box<dyn Cubie>; 26],
}

#[macro_export]
macro_rules! get_face_array {
    ($o:expr, $x:expr) => {
        [
            &$o.elements[$x[0]],
            &$o.elements[$x[1]],
            &$o.elements[$x[2]],
            &$o.elements[$x[3]],
            &$o.elements[$x[4]],
            &$o.elements[$x[5]],
            &$o.elements[$x[6]],
            &$o.elements[$x[7]],
            &$o.elements[$x[8]],
        ]
    }
}

impl Cube {
    /// Initializes a 3x3 Cube with elements in the form of an array with
    /// elements in three slices in the following order:
    ///
    /// left to right, back to front, and top to bottom
    ///
    ///  Top     Middle    Bottom
    /// 0 1 2    9 10 11  18 19 20
    /// 3 4 5   12 13 14  21 22 23
    /// 6 7 8   15 16 17  24 25 26
    ///
    /// where 0 would be the top left corner cubie in the back.
    pub fn new() -> Self {
        Self {
            elements: [
                cubie!("corner"), cubie!("edge"),   cubie!("corner"),
                cubie!("edge"),   cubie!("center"), cubie!("edge"),
                cubie!("corner"), cubie!("edge"),   cubie!("corner"),
                cubie!("edge"),   cubie!("center"), cubie!("edge"),
                cubie!("center"),                   cubie!("center"),
                cubie!("edge"),   cubie!("center"), cubie!("edge"),
                cubie!("corner"), cubie!("edge"),   cubie!("corner"),
                cubie!("edge"),   cubie!("corner"), cubie!("edge"),
                cubie!("corner"), cubie!("edge"),   cubie!("corner"),
            ]
        }
    }

    pub fn get_corners(&self) -> [&Box<dyn Cubie>; 8] {
        [
            &self.elements[0], &self.elements[2], &self.elements[6],
            &self.elements[8], &self.elements[17], &self.elements[19],
            &self.elements[23], &self.elements[25],
        ]
    }

    pub fn get_face(&self, s: CubeFaceKind) -> CubeFace {
        match s {
            CubeFaceKind::Top => {
                CubeFace {
                    elements:
                        get_face_array!(&self,
                                        [0, 1, 2, 3, 4, 5, 6, 7, 8]),
                }
            },
            CubeFaceKind::Left => {
                CubeFace {
                    elements:
                        get_face_array!(&self,
                                        [0, 3, 6, 9, 12, 14, 17, 20, 23]),
                }
            },
            CubeFaceKind::Right => {
                CubeFace {
                    elements:
                        get_face_array!(&self,
                                        [2, 5, 8, 11, 13, 16, 19, 22, 25])
                }
            },
            CubeFaceKind::Front => {
                CubeFace {
                    elements:
                        get_face_array!(&self,
                                        [6, 7, 8, 14, 15, 16, 23, 24, 25]),
                }
            },
            CubeFaceKind::Back => {
                CubeFace {
                    elements:
                        get_face_array!(&self,
                                        [0, 1, 2, 9, 10, 11, 17, 18, 19]),
                }
            },
            CubeFaceKind::Bottom => {
                CubeFace {
                    elements:
                        get_face_array!(&self,
                                        [17, 18, 19, 20, 21, 22, 23, 24, 25]),
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cubie_color() {
        // Inequality
        let c1 = (CubieColor::Blue, CubieColor::Green);

        assert!(c1.0 != c1.1);

        // Equality
        let c2 = (CubieColor::Blue, CubieColor::Blue);

        assert_eq!(c2.0, c2.1);

        // Init
        let c3 = (CubieColor::new(), CubieColor::Uninit);

        assert_eq!(c3.0, c3.1)
    }

    #[test]
    fn center_cubie() {
        let a: Box<dyn Cubie> = cubie!("center");
        let b: &CenterCubie =
            match a.as_any().downcast_ref::<CenterCubie>() {
            Some(b) => b,
            None => panic!("&b isn't a CenterCubie!!"),
        };
        assert_eq!(b.get_faces()[0], CubieFace { color: CubieColor::Uninit });
    }

    #[test]
    fn cube_init() {
        let c = Cube::new();

        assert_eq!(c.elements.len(), 26);
    }

    #[test]
    fn cubie_macro() {
        let a: Box<dyn Cubie> = cubie!("corner");
        let b: &CornerCubie =
            match a.as_any().downcast_ref::<CornerCubie>() {
            Some(b) => b,
            None => panic!("&b isn't a CornerCubie!!"),
        };
        assert_eq!(b.get_faces().len(), 3);
    }

    #[test]
    fn get_face_array() {
        let c = Cube::new();

        let cf: [&Box<dyn Cubie>; 9] =
            get_face_array!(c, [0, 1, 2, 3, 4, 5, 6, 7, 8]);

        assert_eq!(cf.len(), 9);
        let cubie: &Box<dyn Cubie> = &c.elements[0];
        let cornercubie: &CornerCubie =
            match cubie.as_any().downcast_ref::<CornerCubie>() {
            Some(i) => i,
            None => panic!("cubie isn't a CornerCubie!!"),
        };

        let cubie2: &Box<dyn Cubie> = cf[0];
        let cornercubie2: &CornerCubie =
            match cubie2.as_any().downcast_ref::<CornerCubie>() {
            Some(i) => i,
            None => panic!("cubie2 isn't a CornerCubie!!"),
        };

        assert_eq!(cornercubie.faces, cornercubie2.faces);
    }
}
