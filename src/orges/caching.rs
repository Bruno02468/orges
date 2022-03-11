//! Caching orgyfier.

use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ops::{AddAssign, RemAssign};

type CacheBlock = HashMap<String, Option<String>>;

pub(crate) struct Cachorges {
  blocks: usize,
  entries_per_block: usize,
  current_block_index: RefCell<usize>,
  owc_cache: Vec<RefCell<CacheBlock>>,
  pub hits: RefCell<usize>,
  pub misses: RefCell<usize>
}

impl Default for Cachorges {
  fn default() -> Self {
    let blocks = 4;
    let entries_per_block = 512;
    let mut v: Vec<RefCell<CacheBlock>> = Vec::with_capacity(blocks);
    for _ in [0..blocks] {
      v.push(RefCell::new(
        CacheBlock::with_capacity(entries_per_block)
      ));
    }
    Self {
      blocks,
      entries_per_block,
      current_block_index: RefCell::new(0),
      owc_cache: v,
      hits: RefCell::new(0),
      misses: RefCell::new(0)
    }
  }
}

impl Cachorges {
  /// Get from cache.
  fn cache_get(&self, word: &str) -> Option<Option<String>> {
    for block in &self.owc_cache {
      if let Some(v) = block.borrow().get(word) {
        return Some(v.clone());
      }
    }
    return None;
  }

  /// Mutable reference to current cache block.
  fn current_block_mut(&self) -> RefMut<CacheBlock> {
    return self.owc_cache.get(
      *self.current_block_index.borrow()
    ).unwrap().borrow_mut();
  }

  /// Orgify and Insert into cache.
  pub(crate) fn orges_word_case(&self, word: &str) -> Option<String> {
    if let Some(o) = self.cache_get(word) {
      self.hits.borrow_mut().add_assign(1);
      return o.to_owned();
    } else {
      self.misses.borrow_mut().add_assign(1);
      if self.current_block_mut().len() == self.entries_per_block {
        log::info!("cache swap! {}", *self.current_block_index.borrow());
        self.current_block_index.borrow_mut().add_assign(1);
        self.current_block_index.borrow_mut().rem_assign(self.blocks);
        self.current_block_mut().clear();
      }
      let o = super::orges_word_case(word);
      self.current_block_mut().insert(
        word.to_owned(),
        o.clone()
      );
      return o;
    }
  }

  pub(crate) fn orges_string(&self, txt: &str, rate: f64) -> String {
    let h = *self.hits.borrow();
    let m = *self.misses.borrow();
    log::info!(
      "cache info: {} hits, {} misses (hit rate: {}%)",
      h,
      m,
      (h as f64)/((h + m) as f64)*100.0
    );
    return super::orges_string(txt, rate, |a| self.orges_word_case(a));
  }
}
