use std::fmt::{
    Debug,
    Display,
    Formatter,
    Result
};

/// The Cursor enum represents the possible css cursors, see https://www.w3schools.com/cssref/pr_class_cursor.asp for details.
/// It ignores the naming conventions for enum constants so that the constants can be converted more easily to their corresponding
/// css values.
#[derive(Debug,PartialEq)]
#[allow(non_camel_case_types)]
pub enum Cursor {

    ALIAS,
    ALL_SCROLL,
    AUTO,
    CELL,
    CONTEXT_MENU,
    COLL_RESIZE,
    COPY,
    CROSSHAIR,
    DEFAULT,
    E_RESIZE,
    EW_RESIZE,
    GRAB,
    GRABBING,
    HELP,
    MOVE,
    N_RESIZE,
    NE_RESIZE,
    NESW_RESIZE,
    NS_RESIZE,
    NW_RESIZE,
    NWSE_RESIZE,
    NO_DROP,
    NONE,
    NOT_ALLOWED,
    POINTER,
    PROGRESS,
    ROW_RESIZE,
    S_RESIZE,
    SE_RESIZE,
    SW_RESIZE,
    TEXT,
    URL(String),
    VERTICAL_TEXT,
    W_RESIZE,
    WAIT,
    ZOOM_IN,
    ZOOM_OUT,
    INITIAL,
    INHERIT
}

impl Cursor {

    pub fn to_css_value(&self) -> String {
        match self {
            Cursor::URL(_csv) => {
                panic!("Should add support for url cursors...");
            },
            normal => {
                format!("{}", normal)
            }
        }
    }
}

impl Display for Cursor {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let debug = format!("{:?}", self);
        let result = debug.replace("_", "-").to_lowercase();
        write!(f, "{}", result)
    }
}