pub use cut_edges::BridgesSearch;

mod cut_edges;

/// Marker type for bridges search.
#[derive(Debug, PartialEq)]
pub enum Color {
    Gray,
    Black,
}
