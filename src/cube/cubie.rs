use staticvec::{staticvec, StaticVec};
use std::any::Any;

/// Standard colors for 6 sided twisty puzzles, plus an uninitialized value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Blue,
    Green,
    Orange,
    Red,
    White,
    Yellow,
    Uninit,
}

impl Color {
    pub const fn new() -> Self {
        Color::Uninit
    }

    /// Returns the standardized opposite color of a Color instance
    pub const fn opposite_color(&self) -> Self {
        Color::opposite_color_from_color(&self)
    }

    /// Returns the standardized opposite color of a Color value
    pub const fn opposite_color_from_color(color: &Self) -> Self {
        match color {
            Color::Blue => Color::Green,
            Color::Green => Color::Blue,
            Color::Orange => Color::Red,
            Color::Red => Color::Orange,
            Color::White => Color::Yellow,
            Color::Yellow => Color::White,
            Color::Uninit => Color::Uninit,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq)]
pub struct Face {
    pub color: Color,
}

impl PartialEq for Face {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}

impl Face {
    pub const fn new() -> Self {
        Self {
            color: Color::new(),
        }
    }

    pub const fn new_from_cubie_color(color: Color) -> Self {
        Self {
            color,
        }
    }

    pub const fn opposite_color(&self) -> Color {
        Color::opposite_color(&self.color)
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
/// use rustytwisty::cube::cubie::*;
///
/// let a: Box<dyn Cubie> = Center::new_boxed();
/// let b: &Center =
///     match a.as_any().downcast_ref::<Center>() {
///     Some(b) => b,
///     None => panic!("&b isn't a Center!!"),
/// };
/// ```
pub trait Cubie {
    fn as_any(&self) -> &dyn Any;
}

pub trait BuildCubie {
    fn new() -> Self;
}

#[derive(Clone, Debug, Eq)]
pub struct Center {
    pub faces: StaticVec<Face, 1>,
}

impl PartialEq for Center {
    fn eq(&self, other: &Self) -> bool {
        self.faces == other.faces
    }
}

impl Cubie for Center {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl BuildCubie for Center {
    fn new() -> Self {
        Self {
            faces: staticvec![Face::new(); 1],
        }
    }
}

impl Center {
    pub fn new_boxed() -> Box<Self> {
        Box::new(Self {
            faces: staticvec![Face::new(); 1],
        })
    }

    pub const fn new_from_array(arr: [Face; 1]) -> Self {
        Self {
            faces: StaticVec::new_from_const_array(arr),
        }
    }

    pub fn new_boxed_from_array(arr: [Face; 1]) -> Box<Self> {
        Box::new(Self {
            faces: StaticVec::new_from_array(arr),
        })
    }

    pub fn new_from_vec(vec: Vec<Face>) -> Self {
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

    pub fn new_boxed_from_vec(vec: Vec<Face>) -> Box<Self> {
        if vec.len() == 0 {
            Box::new(Self::new())
        } else {
            let mut v = vec.clone();
            v.truncate(1);

            Box::new(Self {
                faces: StaticVec::from(v),
            })
        }
    }

    pub const fn faces(&self) -> &StaticVec<Face, 1> {
        &self.faces
    }
}

#[derive(Clone, Debug, Eq)]
pub struct Corner {
    pub faces: StaticVec<Face, 3>,
}

impl PartialEq for Corner {
    fn eq(&self, other: &Self) -> bool {
        self.faces == other.faces
    }
}

impl Cubie for Corner {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl BuildCubie for Corner {
    fn new() -> Self {
        Self {
            faces: staticvec![Face::new(); 3],
        }
    }
}

impl Corner {
    pub fn new_boxed() -> Box<Self> {
        Box::new(Self {
            faces: staticvec![Face::new(); 3],
        })
    }

    pub const fn new_from_array(arr: [Face; 3]) -> Self {
        Self {
            faces: StaticVec::new_from_const_array(arr),
        }
    }

    pub fn new_boxed_from_array(arr: [Face; 3]) -> Box<Self> {
        Box::new(Self {
            faces: StaticVec::new_from_array(arr),
        })
    }

    pub fn new_from_vec(vec: Vec<Face>) -> Self {
        let l = vec.len();
        if l == 0 {
            Self::new()
        } else if (l > 0) && (l < 3) {
            Self {
                faces: staticvec![vec[0]; 3],
            }
        } else {
            let mut v = vec.clone();
            v.truncate(1);

            Self {
                faces: StaticVec::from(v),
            }
        }
    }

    pub fn new_boxed_from_vec(vec: Vec<Face>) -> Box<Self> {
        let l = vec.len();
        if l == 0 {
            Box::new(Self::new())
        } else if (l > 0) && (l < 3) {
            Box::new(Self {
                faces: staticvec![vec[0]; 3],
            })
        } else {
            let mut v = vec.clone();
            v.truncate(3);

            Box::new(Self {
                faces: StaticVec::from(v),
            })
        }
    }

    pub const fn faces(&self) -> &StaticVec<Face, 3> {
        &self.faces
    }
}

#[derive(Clone, Debug, Eq)]
pub struct Edge {
    pub faces: StaticVec<Face, 2>,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.faces == other.faces
    }
}

impl Cubie for Edge {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl BuildCubie for Edge {
    fn new() -> Self {
        Self {
            faces: staticvec![Face::new(); 2],
        }
    }
}

impl Edge {
    pub fn new_boxed() -> Box<Self> {
        Box::new(Self {
            faces: staticvec![Face::new(); 2],
        })
    }

    pub const fn new_from_array(arr: [Face; 2]) -> Self {
        Self {
            faces: StaticVec::new_from_const_array(arr),
        }
    }

    pub fn new_boxed_from_array(arr: [Face; 2]) -> Box<Self> {
        Box::new(Self {
            faces: StaticVec::new_from_array(arr),
        })
    }

    pub fn new_from_vec(vec: Vec<Face>) -> Self {
        let l = vec.len();
        if l == 0 {
            Self::new()
        } else if (l > 0) && (l < 2) {
            Self {
                faces: staticvec![vec[0]; 2],
            }
        } else {
            let mut v = vec.clone();
            v.truncate(1);

            Self {
                faces: StaticVec::from(v),
            }
        }
    }

    pub fn new_boxed_from_vec(vec: Vec<Face>) -> Box<Self> {
        let l = vec.len();
        if l == 0 {
            Box::new(Self::new())
        } else if (l > 0) && (l < 2) {
            Box::new(Self {
                faces: staticvec![vec[0]; 2],
            })
        } else {
            let mut v = vec.clone();
            v.truncate(1);

            Box::new(Self {
                faces: StaticVec::from(v),
            })
        }
    }

    pub const fn faces(&self) -> &StaticVec<Face, 2> {
        &self.faces
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cubie_color() {
        // Inequality
        let c1 = (Color::Blue, Color::Green);

        assert!(c1.0 != c1.1);

        // Equality
        let c2 = (Color::Blue, Color::Blue);

        assert_eq!(c2.0, c2.1);

        // Init
        let c3 = (Color::new(), Color::Uninit);

        assert_eq!(c3.0, c3.1)
    }

    #[test]
    fn center_cubie() {
        let a: Box<dyn Cubie> = cubie!("center");
        let b: &Center = match a.as_any().downcast_ref::<Center>() {
            Some(b) => b,
            None => panic!("&b isn't a Center!!"),
        };
        assert_eq!(b.faces()[0], Face {
            color: Color::Uninit
        });
    }

    #[test]
    fn new_from_array() {
        let a = Center::new_from_array([Face::new(); 1]);

        // new_from_array works with good value
        assert_eq!(a.faces.len(), 1);
    }

    #[test]
    fn new_from_vec() {
        let a = Center::new_boxed_from_vec(vec![Face::new(); 1]);

        // new_from_vec works with good value
        assert_eq!(a.faces.len(), 1);

        let a = Center::new_boxed_from_vec(vec![Face::new(); 10]);

        // too big value is truncated
        assert_eq!(a.faces.len(), 1);

        let a = Center::new_boxed_from_vec(vec![Face::new(); 0]);

        // too little value means values are added onto faces
        assert_eq!(a.faces.len(), 1);

        let a = Corner::new_boxed_from_vec(vec![Face::new(); 1]);

        // values added on to correct cubie face number
        assert_eq!(a.faces.len(), 3);

        let a = Corner::new_boxed_from_vec(vec![Face::new(); 4]);

        // truncation works for larger cubie types
        assert_eq!(a.faces.len(), 3);
    }

    #[test]
    fn cubie_macro() {
        let a: Box<dyn Cubie> = cubie!("corner");
        let b: &Corner = match a.as_any().downcast_ref::<Corner>() {
            Some(b) => b,
            None => panic!("&b isn't a Corner!!"),
        };
        assert_eq!(b.faces().len(), 3);
    }
}
