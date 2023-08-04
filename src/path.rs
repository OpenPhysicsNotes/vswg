use std::str::FromStr;

use std::ops::Div;

/// A sequence of path elements
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathVec {
    elements: Vec<PathElement>,
}

impl PathVec {
    pub fn new() -> PathVec {
        PathVec {
            elements: Vec::new(),
        }
    }
    pub fn push(&mut self, element: PathElement) {
        self.elements.push(element);
    }
    pub fn parent(&self) -> Option<Self> {
        if self.elements.len() == 0 {
            return None;
        }
        let mut new_elements = self.elements.clone();
        new_elements.pop();
        Some(PathVec {
            elements: new_elements,
        })
    }

    pub fn inverse(&self) -> Self {
        // TODO better
        let elements = self.elements.iter()
            .rev()
            .map(|e| match e {
                PathElement::None => unimplemented!(),
                PathElement::Current => unimplemented!(),
                PathElement::Sup => unimplemented!(),
                PathElement::Name(_) => PathElement::Sup,
            })
            .collect::<Vec<PathElement>>();

        Self {
            elements,
        }
    }

    pub fn uri(&self) -> String {
        if self.elements.is_empty() {
            return ".".into();
        }

        self.elements.iter()
            .map(|element| element.element_repr())
            .collect::<Vec<String>>()
            .join("/")
    }

    pub fn uri_dir(&self) -> String {
        let uri = self.uri();
        if uri.ends_with("/") {
            uri
        } else {
            uri + "/"
        }
    }

    pub fn pieces(&self) -> Vec<String> {
        self.elements.iter()
            .map(|element| element.element_repr())
            .collect::<Vec<String>>()
    }
}

impl FromStr for PathVec {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut path_vec = PathVec::new();
        for element in s.split("/") {
            path_vec.push(PathElement::from_str(element)?);
        }
        Ok(path_vec)
    }
}

impl std::ops::Div<PathVec> for &PathVec {
    type Output = PathVec;
    fn div(self, rhs: PathVec) -> Self::Output {
        let mut new_elements = self.elements.clone();
        new_elements.extend(rhs.elements);
        PathVec {
            elements: new_elements,
        }
    }
}

impl Div<&str> for &PathVec {
    type Output = PathVec;
    fn div(self, rhs: &str) -> Self::Output {
        let mut new_elements = self.elements.clone();
        new_elements.push(PathElement::from_str(rhs).unwrap());
        PathVec {
            elements: new_elements,
        }
    }
}

impl Div<String> for &PathVec {
    type Output = PathVec;
    fn div(self, rhs: String) -> Self::Output {
        self / rhs.as_str()
    }
}

/// An element of a path
///
/// This is either a name, or a special element like `.` or `..`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathElement {
    None,
    Current,
    Sup,
    Name(String),
}

impl PathElement {
    pub fn element_repr(&self) -> String {
        match self {
            PathElement::None => "".into(),
            PathElement::Current => ".".into(),
            PathElement::Sup => "..".into(),
            PathElement::Name(name) => name.clone(),
        }
    }
}

impl From<PathElement> for PathVec {
    fn from(element: PathElement) -> Self {
        let mut path_vec = PathVec::new();
        path_vec.push(element);
        path_vec
    }
}

impl FromStr for PathElement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(PathElement::None),
            "." => Ok(PathElement::Current),
            ".." => Ok(PathElement::Sup),
            _ => Ok(PathElement::Name(s.into())),
        }
    }
}