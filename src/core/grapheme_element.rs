use super::utils;
use std::{error::Error, fmt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphemeElementError {
    AlreadyChildOfSomeone,
    AlreadyMyChild,
}

impl fmt::Display for GraphemeElementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GraphemeElementError::AlreadyChildOfSomeone => {
                write!(f, "Element is already a child of some element.")
            }
            GraphemeElementError::AlreadyMyChild => {
                write!(f, "Element is already a child of this group.")
            }
        }
    }
}

impl Error for GraphemeElementError {}

pub struct GraphemeElementInfo<'a> {
    /// Used to sort which element is drawn first. A lower precedence means it will be drawn first
    /// and appear on the bottom
    pub precedence: i32,

    /// The parent of this element
    pub parent: Option<&'a dyn GraphemeElement>,

    // /// The plot this element belongs to
    // pub plot: Option<&Plot>,
    /// Whether update() needs to be called before render()
    pub needs_update: bool,

    // /// Custom event listeners
    // event_listeners: HashMap<String, Box<dyn Fn()>>,
    /// Children of this element
    pub children: Vec<Box<dyn GraphemeElement>>,

    /// Whether this element is visible
    pub visible: bool,
}

/// A component of Grapheme that supports a update() function, which prepares it for the rendering
/// stage, and a render() function which renders the element to a GraphemeCanvas. It also has
/// children, used for grouping elements together.
pub trait GraphemeElement {
    fn get_element_info(&self) -> &GraphemeElementInfo;
    fn get_element_info_mut(&mut self) -> &mut GraphemeElementInfo;
    fn add(
        &mut self,
        mut element: Box<dyn GraphemeElement>,
    ) -> Result<(), (GraphemeElementError, Box<dyn GraphemeElement>)> {
        if let None = element.get_element_info().parent {
            Err((GraphemeElementError::AlreadyChildOfSomeone, element))
        }
        /*else if self.has_child(element, true) {
            (GraphemeElementError::AlreadyMyChild, element)
        } */
        else {
            element.get_element_info_mut().parent = Some(self);
            // element.setPlot(self.get_element_info().plot);
            self.get_element_info_mut().children.push(element);
            Ok(())
        }
    }
}
