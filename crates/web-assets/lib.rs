//include!(concat!(env!("OUT_DIR"), "/templates.rs"));
//pub use templates::statics as files;

include!(concat!(env!("OUT_DIR"), "/static_files.rs"));
pub use statics as files;
