mod traits;
pub mod search;
pub mod dominator;
pub mod order;
pub mod adjacency_list;
pub mod adjacency_matrix;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate bit_set;

pub use adjacency_list::AdjacencyList;
pub use adjacency_matrix::AdjacencyMatrix;

pub use traits::Graph as GraphTrait;
pub use traits::AdjacencyGraph as AdjacencyGraphTrait;
pub use traits::IncidenceGraph as IncidenceGraphTrait;
pub use traits::BidirectionalGraph as BidirectionalGraphTrait;
pub use traits::VertexListGraph as VertexListGraphTrait;
pub use traits::EdgeListGraph as EdgeListGraphTrait;
pub use traits::MutableGraph as MutableGraphTrait;
pub use traits::AdjacencyMatrixGraph as AdjacencyMatrixGraphTrait;
