//! Init the yew app.

pub mod orges;
mod converter;

use converter::Converter;

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::start_app::<Converter>();
}
