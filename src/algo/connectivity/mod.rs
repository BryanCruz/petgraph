pub use cut_edges::CutEdgesSearch;
pub use cut_vertices::CutVerticesSearch;

mod cut_edges;
mod cut_vertices;

/// Marker type for bridges search.
#[derive(Debug, PartialEq)]
pub enum Color {
    Gray,
    Black,
}
