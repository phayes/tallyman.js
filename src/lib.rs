mod utils;

use cfg_if::cfg_if;
use serde_derive::Serialize;
use tallyman_rs::plurality::PluralityTally as RustPluralityTally;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
  alert("Hello, tallyman!");
}

#[wasm_bindgen]
pub struct PluralityTally {
  tally: RustPluralityTally<String, u64>,
}

#[wasm_bindgen]
impl PluralityTally {
  pub fn new(num_winners: u32) -> PluralityTally {
    return PluralityTally {
      tally: RustPluralityTally::new(num_winners),
    };
  }

  pub fn add(&mut self, vote: String) {
    self.tally.add(vote);
  }

  pub fn candidates(&self) -> JsValue {
    JsValue::from_serde(&self.tally.candidates()).unwrap()
  }

  pub fn totals(&self) -> JsValue {
    JsValue::from_serde(&self.tally.totals()).unwrap()
  }

  pub fn ranked(&self) -> JsValue {
    JsValue::from_serde(&self.tally.ranked()).unwrap()
  }
}
