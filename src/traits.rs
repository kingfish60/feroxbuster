//! collection of all traits used
use crate::filters::{
    LinesFilter, RegexFilter, SimilarityFilter, SizeFilter, StatusCodeFilter, WildcardFilter,
    WordsFilter,
};
use crate::response::FeroxResponse;
use anyhow::Result;
use crossterm::style::{style, Stylize};
use serde::Serialize;
use std::any::Any;
use std::fmt::{self, Debug, Display, Formatter};

// references:
//   https://dev.to/magnusstrale/rust-trait-objects-in-a-vector-non-trivial-4co5
//   https://stackoverflow.com/questions/25339603/how-to-test-for-equality-between-trait-objects

/// FeroxFilter trait; represents different types of possible filters that can be applied to
/// responses
pub trait FeroxFilter: Debug + Send + Sync {
    /// Determine whether or not this particular filter should be applied or not
    fn should_filter_response(&self, response: &FeroxResponse) -> bool;

    /// delegates to the FeroxFilter-implementing type which gives us the actual type of self
    fn box_eq(&self, other: &dyn Any) -> bool;

    /// gives us `other` as Any in box_eq
    fn as_any(&self) -> &dyn Any;
}

impl Display for dyn FeroxFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        if let Some(filter) = self.as_any().downcast_ref::<LinesFilter>() {
            write!(f, "Line count: {}", style(filter.line_count).cyan())
        } else if let Some(filter) = self.as_any().downcast_ref::<WordsFilter>() {
            write!(f, "Word count: {}", style(filter.word_count).cyan())
        } else if let Some(filter) = self.as_any().downcast_ref::<SizeFilter>() {
            write!(f, "Response size: {}", style(filter.content_length).cyan())
        } else if let Some(filter) = self.as_any().downcast_ref::<RegexFilter>() {
            write!(f, "Regex: {}", style(&filter.raw_string).cyan())
        } else if let Some(filter) = self.as_any().downcast_ref::<WildcardFilter>() {
            if filter.dynamic != u64::MAX {
                write!(f, "Dynamic wildcard: {}", style(filter.dynamic).cyan())
            } else {
                write!(f, "Static wildcard: {}", style(filter.size).cyan())
            }
        } else if let Some(filter) = self.as_any().downcast_ref::<StatusCodeFilter>() {
            write!(f, "Status code: {}", style(filter.filter_code).cyan())
        } else if let Some(filter) = self.as_any().downcast_ref::<SimilarityFilter>() {
            write!(
                f,
                "Pages similar to: {}",
                style(&filter.original_url).cyan()
            )
        } else {
            write!(f, "Filter: {self:?}")
        }
    }
}

/// implementation of PartialEq, necessary long-form due to "trait cannot be made into an object"
/// error when attempting to derive PartialEq on the trait itself
impl PartialEq for Box<dyn FeroxFilter> {
    /// Perform a comparison of two implementors of the FeroxFilter trait
    fn eq(&self, other: &Box<dyn FeroxFilter>) -> bool {
        self.box_eq(other.as_any())
    }
}

/// FeroxSerialize trait; represents different types that are Serialize and also implement
/// as_str / as_json methods
pub trait FeroxSerialize: Serialize {
    /// Return a String representation of the object, generally the human readable version of the
    /// implementor
    fn as_str(&self) -> String;

    /// Return an NDJSON representation of the object
    fn as_json(&self) -> Result<String>;
}
