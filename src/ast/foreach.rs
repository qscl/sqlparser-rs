#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

use super::{display_comma_separated, Expr, Ident, Located, SelectItem};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "visitor", derive(Visit, VisitMut))]
pub struct LoopRange {
    pub item: Located<Ident>,
    pub range: Box<Expr>,
}

impl fmt::Display for LoopRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} in {}", self.item, self.range)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "visitor", derive(Visit, VisitMut))]
pub struct ForEach<T> {
    pub ranges: Vec<LoopRange>,
    pub body: Vec<T>,
}

impl<T: fmt::Display> fmt::Display for ForEach<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FOR EACH ")?;
        if self.ranges.len() == 1 {
            write!(f, "{}", self.ranges[0])?;
        } else {
            write!(f, "({})", display_comma_separated(&self.ranges))?;
        }
        writeln!(f, "{{")?;
        for stmt in self.body.iter() {
            write!(f, "  {},", stmt)?;
        }
        write!(f, "\n}}")
    }
}

impl ForEach<ForEachOr<Vec<Expr>>> {
    pub fn vec_display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FOR EACH ")?;
        if self.ranges.len() == 1 {
            write!(f, "{}", self.ranges[0])?;
        } else {
            write!(f, "({})", display_comma_separated(&self.ranges))?;
        }
        writeln!(f, "{{")?;
        for stmt in self.body.iter() {
            match stmt {
                ForEachOr::ForEach(fe) => {
                    write!(f, "  ")?;
                    fe.vec_display(f)?;
                }
                ForEachOr::Item(item) => write!(f, "  {},", display_comma_separated(item))?,
            };
        }
        write!(f, "\n}}")
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "visitor", derive(Visit, VisitMut))]
pub enum ForEachOr<T> {
    ForEach(ForEach<ForEachOr<T>>),
    Item(T),
}

impl<T> From<T> for ForEachOr<T> {
    fn from(item: T) -> Self {
        Self::Item(item)
    }
}

pub trait ToForEach {
    type Target;
    fn for_each(self) -> Self::Target;
}

impl<T> ToForEach for Vec<T> {
    type Target = Vec<ForEachOr<T>>;
    fn for_each(self) -> Self::Target {
        self.into_iter().map(|i| ForEachOr::Item(i)).collect()
    }
}

impl<T: fmt::Display> fmt::Display for ForEachOr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ForEach(fe) => fe.fmt(f),
            Self::Item(item) => item.fmt(f),
        }
    }
}

pub trait ConstructForEach: Sized {
    fn construct_for_each(foreach: ForEach<Self>) -> Self;
}

impl ConstructForEach for SelectItem {
    fn construct_for_each(foreach: ForEach<Self>) -> Self {
        SelectItem::ForEach(foreach)
    }
}

impl<T> ConstructForEach for ForEachOr<T> {
    fn construct_for_each(foreach: ForEach<Self>) -> Self {
        ForEachOr::ForEach(foreach)
    }
}
