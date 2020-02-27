#[macro_export]
macro_rules! cubie {
    ($x:expr) => {
        match $x {
            "center" => crate::cube::cubie::Center::new_boxed(),
            "corner" => crate::cube::cubie::Corner::new_boxed(),
            "edge" => crate::cube::cubie::Edge::new_boxed(),
            _ => panic!("Cubie type not found"),
        };
    };
}
