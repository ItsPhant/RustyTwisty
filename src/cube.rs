use std::any::Any;
use staticvec::{staticvec, StaticVec};

/// Standard colors for 6 sided twisty puzzles, plus an uninitialized value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    pub const fn new() -> Self {
        CubieColor::Uninit
    }

    /// Returns the standardized opposite color of a CubieColor instance
    pub const fn get_opposite_color(&self) -> Self {
        CubieColor::get_opposite_color_from_color(&self)
    }

    /// Returns the standardized opposite color of a CubieColor value
    pub const fn get_opposite_color_from_color(color: &Self) -> Self {
        match color {
            CubieColor::Blue => CubieColor::Green,
            CubieColor::Green => CubieColor::Blue,
            CubieColor::Orange => CubieColor::Red,
            CubieColor::Red => CubieColor::Orange,
            CubieColor::White => CubieColor::Yellow,
            CubieColor::Yellow => CubieColor::White,
            CubieColor::Uninit => CubieColor::Uninit,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq)]
pub struct CubieFace {
    pub color: CubieColor,
}

impl PartialEq for CubieFace {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}

impl CubieFace {
    pub const fn new() -> Self {
        Self {
            color: CubieColor::new(),
        }
    }

    pub const fn new_from_cubie_color(color: CubieColor) -> Self {
        Self {
            color
        }
    }

    pub const fn get_opposite_color(&self) -> CubieColor {
        CubieColor::get_opposite_color(&self.color)
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
/// use rustytwisty::cube::*;
///
/// let a: Box<dyn Cubie> = CenterCubie::new_boxed();
/// let b: &CenterCubie =
///     match a.as_any().downcast_ref::<CenterCubie>() {
///     Some(b) => b,
///     None => panic!("&b isn't a CenterCubie!!"),
/// };
/// ```
pub trait Cubie {
    fn as_any(&self) -> &dyn Any;
}

pub trait CubieHelper {
    fn new() -> Self;
}

#[derive(Clone, Debug, Eq)]
pub struct CenterCubie {
    pub faces: StaticVec<CubieFace, 1>
}

impl PartialEq for CenterCubie {
    fn eq(&self, other: &Self) -> bool {
        self.faces == other.faces
    }
}

impl const Cubie for CenterCubie {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl const CubieHelper for CenterCubie {
    fn new() -> Self {
        Self {
            faces: staticvec![CubieFace::new(); 1],
        }
    }
}

impl CenterCubie {
    pub fn new_boxed() -> Box<Self> {
        Box::new(Self {
            faces: staticvec![CubieFace::new(); 1],
        })
    }

    pub const fn new_from_array(arr: [CubieFace; 1]) -> Self {
        Self {
            faces: StaticVec::new_from_const_array(arr),
        }
    }

    pub fn new_boxed_from_array(arr: [CubieFace; 1]) -> Box<Self> {
        Box::new(Self {
            faces: StaticVec::new_from_array(arr),
        })
    }

    pub fn new_from_vec(vec: Vec<CubieFace>) -> Self {
        if vec.len() == 0 {
            Self::new()
        } else {
            let mut v = vec.clone();
            v.truncate(1);

            Self {
                faces: StaticVec::from(v),
            }
        }
    }

    pub fn new_boxed_from_vec(vec: Vec<CubieFace>) -> Box<Self> {
        if vec.len() == 0 {
            Box::new(Self::new())
        } else {
            let mut v = vec.clone();
            v.truncate(1);

            Box::new(Self {
                faces: StaticVec::from(v)
            })
        }
    }

    pub const fn get_faces(&self) -> &StaticVec<CubieFace, 1> {
        &self.faces
    }
}

#[derive(Clone, Debug, Eq)]
pub struct CornerCubie {
    pub faces: StaticVec<CubieFace, 3>
}

impl PartialEq for CornerCubie {
    fn eq(&self, other: &Self) -> bool {
        self.faces == other.faces
    }
}

impl const Cubie for CornerCubie {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl const CubieHelper for CornerCubie {
    fn new() -> Self {
        Self {
            faces: staticvec![CubieFace::new(); 3]
        }
    }
}

impl CornerCubie {
    pub fn new_boxed() -> Box<Self> {
        Box::new(Self {
            faces: staticvec![CubieFace::new(); 3]
        })
    }

    pub const fn new_from_array(arr: [CubieFace; 3]) -> Self {
        Self {
            faces: StaticVec::new_from_const_array(arr),
        }
    }

    pub fn new_boxed_from_array(arr: [CubieFace; 3]) -> Box<Self> {
        Box::new(Self {
            faces: StaticVec::new_from_array(arr),
        })
    }

    pub fn new_from_vec(vec: Vec<CubieFace>) -> Self {
        let l = vec.len();
        if l == 0 {
            Self::new()
        } else if (l > 0) && (l < 3) {
            Self {
                faces: staticvec![vec[0]; 3]
            }
        } else {
            let mut v = vec.clone();
            v.truncate(1);

            Self {
                faces: StaticVec::from(v),
            }
        }
    }

    pub fn new_boxed_from_vec(vec: Vec<CubieFace>) -> Box<Self> {
        let l = vec.len();
        if l == 0 {
            Box::new(Self::new())
        } else if (l > 0) && (l < 3) {
            Box::new(Self {
                faces: staticvec![vec[0]; 3]
            })
        } else {
            let mut v = vec.clone();
            v.truncate(3);

            Box::new(Self {
                faces: StaticVec::from(v)
            })
        }
    }

    pub const fn get_faces(&self) -> &StaticVec<CubieFace, 3> {
        &self.faces
    }
}

#[derive(Clone, Debug, Eq)]
pub struct EdgeCubie {
    pub faces: StaticVec<CubieFace, 2>
}

impl PartialEq for EdgeCubie {
    fn eq(&self, other: &Self) -> bool {
        self.faces == other.faces
    }
}

impl const Cubie for EdgeCubie {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl const CubieHelper for EdgeCubie {
    fn new() -> Self {
        Self {
            faces: staticvec![CubieFace::new(); 2],
        }
    }
}

impl EdgeCubie {
    pub fn new_boxed() -> Box<Self> {
        Box::new(Self {
            faces: staticvec![CubieFace::new(); 2],
        })
    }

    pub const fn new_from_array(arr: [CubieFace; 2]) -> Self {
        Self {
            faces: StaticVec::new_from_const_array(arr),
        }
    }

    pub fn new_boxed_from_array(arr: [CubieFace; 2]) -> Box<Self> {
        Box::new(Self {
            faces: StaticVec::new_from_array(arr),
        })
    }

    pub fn new_from_vec(vec: Vec<CubieFace>) -> Self {
        let l = vec.len();
        if l == 0 {
            Self::new()
        } else if (l > 0) && (l < 2) {
            Self {
                faces: staticvec![vec[0]; 2]
            }
        } else {
            let mut v = vec.clone();
            v.truncate(1);

            Self {
                faces: StaticVec::from(v),
            }
        }
    }

    pub fn new_boxed_from_vec(vec: Vec<CubieFace>) -> Box<Self> {
        let l = vec.len();
        if l == 0 {
            Box::new(Self::new())
        } else if (l > 0) && (l < 2) {
            Box::new(Self {
                faces: staticvec![vec[0]; 2]
            })
        } else {
            let mut v = vec.clone();
            v.truncate(1);

            Box::new(Self {
                faces: StaticVec::from(v),
            })
        }
    }

    pub const fn get_faces(&self) -> &StaticVec<CubieFace, 2> {
        &self.faces
    }
}

#[macro_use]
macro_rules! cubie {
    ($x:expr) => {
        match $x {
            "center" => CenterCubie::new_boxed(),
            "corner" => CornerCubie::new_boxed(),
            "edge" => EdgeCubie::new_boxed(),
            _ => panic!("Cubie type not found"),
        };
    }
}

pub struct CubeFace<'a> {
    pub elements: [&'a Box<dyn Cubie>; 9]
}

impl<'a> CubeFace<'a> {
    pub const fn new_from_array(arr: [&'a Box<dyn Cubie>; 9]) -> Self {
        Self {
            elements: arr
        }
    }
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

#[macro_use]
macro_rules! initialize_cube_face {
    ($o:expr, $x:expr) => {
        CubeFace::new_from_array([
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

    pub const fn get_corners(&self) -> [&Box<dyn Cubie>; 8] {
        [
            &self.elements[0], &self.elements[2], &self.elements[6],
            &self.elements[8], &self.elements[17], &self.elements[19],
            &self.elements[23], &self.elements[25],
        ]
    }

    pub fn get_face(&self, s: CubeFaceKind) -> CubeFace {
        match s {
            CubeFaceKind::Top => {
                initialize_cube_face!(&self,
                                      [0, 1, 2, 3, 4, 5, 6, 7, 8])
            },
            CubeFaceKind::Left => {
                initialize_cube_face!(&self,
                                      [0, 3, 6, 9, 12, 14, 17, 20, 23])
            },
            CubeFaceKind::Right => {
                initialize_cube_face!(&self,
                                      [2, 5, 8, 11, 13, 16, 19, 22, 25])
            },
            CubeFaceKind::Front => {
                initialize_cube_face!(&self,
                                      [6, 7, 8, 14, 15, 16, 23, 24, 25])
            },
            CubeFaceKind::Back => {
                initialize_cube_face!(&self,
                                      [0, 1, 2, 9, 10, 11, 17, 18, 19])
            },
            CubeFaceKind::Bottom => {
                initialize_cube_face!(&self,
                                      [17, 18, 19, 20, 21, 22, 23, 24, 25])
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
    fn new_from_array() {
        let a = CenterCubie::new_from_array([CubieFace::new(); 1]);

        // new_from_array works with good value
        assert_eq!(a.faces.len(), 1);
    }

    #[test]
    fn new_from_vec() {
        let a = CenterCubie::new_boxed_from_vec(vec![CubieFace::new(); 1]);

        // new_from_vec works with good value
        assert_eq!(a.faces.len(), 1);

        let a = CenterCubie::new_boxed_from_vec(vec![CubieFace::new(); 10]);

        // too big value is truncated
        assert_eq!(a.faces.len(), 1);

        let a = CenterCubie::new_boxed_from_vec(vec![CubieFace::new(); 0]);

        // too little value means values are added onto faces
        assert_eq!(a.faces.len(), 1);

        let a = CornerCubie::new_boxed_from_vec(vec![CubieFace::new(); 1]);

        // values added on to correct cubie face number
        assert_eq!(a.faces.len(), 3);

        let a = CornerCubie::new_boxed_from_vec(vec![CubieFace::new(); 4]);

        // truncation works for larger cubie types
        assert_eq!(a.faces.len(), 3);
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

        let cf: CubeFace =
            initialize_cube_face!(c, [0, 1, 2, 3, 4, 5, 6, 7, 8]);

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
