#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::MehApp;



pub mod render_state {
   pub mod structs;
   pub mod vertex_library;
   pub mod vertex_package;
   pub mod test {
      pub mod test_render_pipeline;
   }
}

pub mod packages {
   pub mod time_package;
}

pub mod utility {
   pub mod macros;
}