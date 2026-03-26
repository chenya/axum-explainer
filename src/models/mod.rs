pub mod content;
pub mod types;
pub mod view;

pub use content::*;
pub use types::*;
pub use view::TailwindColored; // trait must be in scope to call its methods
