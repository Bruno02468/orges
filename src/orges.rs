//! Conversion logic.

use lazy_static::lazy_static;
use rand::Rng;
use std::collections::HashMap;

pub(crate) mod caching;

lazy_static! {
  static ref SPECIALS: HashMap<String, Option<String>> = {
    let mut m = HashMap::new();
    let mut nom = |s: &str| m.insert(s.to_owned(), None);
    nom("isso");
    nom("isto");
    nom("esse");
    nom("esses");
    nom("essa");
    nom("essas");
    nom("este");
    nom("estes");
    nom("esta");
    nom("estas");
    nom("aquilo");
    nom("ante");
    nom("após");
    nom("contra");
    nom("desde");
    nom("entre");
    nom("para");
    nom("perante");
    nom("sobre");
    nom("atrás");
    nom("sempre");
    nom("nunca");
    nom("quando");
    nom("qual");
    nom("comigo");
    nom("contigo");
    nom("conosco");
    nom("consigo");
    nom("você");
    nom("vocês");
    nom("lhes");
    nom("convosco");
    nom("nosso");
    nom("nossa");
    nom("nossos");
    nom("nossas");
    nom("vosso");
    nom("vossos");
    nom("vossa");
    nom("vossas");
    nom("minha");
    nom("minhas");
    nom("teus");
    nom("tuas");
    nom("seus");
    nom("suas");
    nom("aquele");
    nom("aqueles");
    nom("aquela");
    nom("aquelas");
    nom("naquele");
    nom("naqueles");
    nom("naquela");
    nom("naquelas");
    nom("naquilo");
    nom("nisto");
    nom("nisso");
    nom("então");
    nom("nessa");
    nom("nesse");
    let mut ye = |s: &str, v: &str| m.insert(s.to_owned(), Some(v.to_owned()));
    ye("borges", "borges™");
    ye("deus", "borges");
    ye("deuses", "borges");
    return m;
  };
  static ref ENDINGS: HashMap<String, String> = {
    let mut m = HashMap::new();
    let mut ye = |s: &str, v: &str| m.insert(s.to_owned(), v.to_owned());
    ye("ção", "çorges");
    ye("çao", "çorges");
    ye("ções", "çorges");
    ye("çoes", "çorges");
    ye("dade", "dorges");
    ye("dado", "dorges");
    ye("ss", "ssorges");
    ye("ç", "çorges");
    ye("iou", "iorges");
    return m;
  };
  static ref VOWELS: Vec<char> = {
    return vec![
      'a', 'e', 'i', 'o', 'u', 'y',
      'á', 'ã', 'â', 'à',
      'í', 'î', 'ì', 'ï', 'ĩ',
      'õ', 'ó', 'ô', 'ò', 'ö',
      'ú', 'ù', 'ũ', 'ü'
    ];
  };
  static ref CONSONANTS: Vec<char> = {
    return vec![
      'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r',
      's', 't', 'v', 'w', 'x', 'z',
      'ç'
    ];
  };
  static ref LETTERS: Vec<char> = {
    return VOWELS.iter().chain(CONSONANTS.iter()).map(|c| *c).collect();
  };
  static ref LETTERS_UPPER: Vec<char> = {
    return LETTERS.iter().map(
      |c| c.to_uppercase().to_string().chars().nth(0).unwrap()
    ).collect();
  };
}

/// Orgesfy a single word. None means keep it. Expects lower case.
pub fn orges_word(word: &str) -> Option<String> {
  // is it in the exceptions?
  if let Some(r) = SPECIALS.get(word) {
    return r.to_owned();
  }
  // less than 3 letters? is it orges already?
  let len = word.len();
  if len <= 3 || word.ends_with("orges") {
    return None;
  }
  // special endings?
  for (k, s) in ENDINGS.iter() {
    if word.ends_with(k) {
      let mut pfx: String = word.chars()
      .take(word.len()-k.len())
      .collect();
      pfx.push_str(s);
      return Some(pfx);
    }
  }
  // ends in two consonants?
  let last2: String = word.chars().rev().take(2).collect();
  if last2.chars().all(|c| CONSONANTS.contains(&c)) {
    match (last2.chars().nth(0).unwrap(), last2.chars().nth(1).unwrap()) {
      ('s', 's') => {
        return Some(format!("{}orges", word));
      }
      (_, 's') => {
        let mut oword = word.to_owned();
        oword.pop();
        return Some(format!("{}orges", oword));
      }
      (_, _) => {
        return Some(format!("{}orges", word));
      }
    }
  }
  // we'll need this later
  let mut oword = word.to_owned();
  // things ending in substrings of orges
  let mut orges = "orges".to_owned();
  let mut suff = "".to_owned();
  loop {
    if orges.len() == 0 { break; }
    if word.ends_with(&orges) {
      let revsuff: String = suff.chars().rev().collect();
      oword.push_str(&revsuff);
      return Some(oword);
    }
    suff.push(orges.pop().unwrap());
  }
  // apply the general method - kudos to Paulo Rubens©
  // characters popped from the word
  let mut popd = "".to_owned();
  let mut c: char;
  let mut round: usize = 0;
  loop {
    let co = oword.pop();
    if co.is_none() {
      // word is all vowels? what?
      return Some(format!("{}orges", word));
    }
    c = co.unwrap();
    popd.push(c);
    match round {
      // round zero: popping last consonants
      0 => {
        if VOWELS.contains(&c) {
          // first vowel popped, time for round 1!
          round = 1;
          continue;
        } else {
          // still popping last cluster of consonants, keep going.
          continue;
        }
      },
      // round 1: pop the last cluster of vowels
      1 => {
        if VOWELS.contains(&c) {
          // just popped a vowel, keep going.
          continue;
        } else {
          // just popped a consonant, after the last cluster of vowels.
          // first, check for e/i for the g/c cases!
          // last popped vowel:
          let lpv = popd.chars().rev().nth(1).unwrap();
          match (c, lpv) {
            ('c', 'e') | ('c', 'i') => {
              oword.push_str("çorges");
              return Some(oword);
            },
            ('g', 'e') | ('g', 'i') => {
              oword.push_str("jorges");
              return Some(oword);
            },
            (_, _) => {}
          }
          // not c/g? put the consonant back!
          oword.push(c);
          // is the last popped vowel u? if so, put it back.
          if lpv == 'u' || lpv == 'i' {
            oword.push(lpv);
          }
          // then, add orges... and we're done!
          oword.push_str("orges");
          return Some(oword);
        }
      },
      _ => {
        panic!("round?!");
      }
    }
  }
}

/// Orgesfy a word whilst preserving case.
pub fn orges_word_case(word: &str) -> Option<String> {
  let lower = word.to_lowercase();
  if let Some(orged) = orges_word(&lower) {
    let mut res: String = "".to_owned();
    let mut orgbuf: String = orged.chars().rev().collect();
    let mut oword: String = word.chars().rev().collect();
    let num_lower = word.chars().filter(|c| c.is_lowercase()).count();
    let num_upper = word.len() - num_lower;
    loop {
      // caractere da palavra orgificada
      let bo = orgbuf.pop();
      if bo.is_none() {
        // cabô
        break;
      }
      let b = bo.unwrap();
      let bu: String = b.to_uppercase().collect();
      // caractere da palavra original
      let co = oword.pop(); 
      let mut done = false;
      if let Some(c) = co {
        // ainda pegamos caracteres da palavra original.
        // é correspondência?
        if c.to_lowercase().eq(b.to_lowercase()) {
          // sim.,preservar case.
          if c.is_uppercase() {
            res.push_str(&bu);
          } else {
            res.push(b);
          }
        } else {
          done = true;
        }
      } else {
        done = true;
      }
      if done {
        // acabou a palavra original. hora de botar orges.
        if num_upper <= 1 {
          // só tinha um upper ou menos. orges vai ser minúsculo e dane-se.
          res.push(b);
        } else if num_upper == word.len() {
          // palavra era toda upper. vai ser tudo upper.
          res.push_str(&bu);
        } else {
          // palavra era mixed case. vamos randomizar...
          // PROPORCIONALMENTE!
          /* deu ruim clan
          let chance: f64 = (num_upper as f64)/(word.len() as f64);
          if rand::thread_rng().gen_bool(chance) {
            res.push_str(&bu);
          } else {
            res.push(b);
          }*/
          res.push(b);
        }
      }
    }
    return Some(res);
  } else {
    return None;
  }
}

/// Orgesfy an entire string with a proportion.
pub fn orges_string<F>(
  txt: &str,
  rate: f64,
  owc: F
) -> String where F: Fn(&str) -> Option<String> {
  let mut outbuf = "".to_owned();
  let mut wordbuf = "".to_owned();
  let do_word = |
    s: &mut String,
    o: &mut String,
    c: Option<char>
  | {
    let orged = owc(&s);
    if let Some(os) = orged {
      if rand::thread_rng().gen_bool(rate) {
        o.push_str(&os);
      } else {
        o.push_str(&s);
      }
    } else {
      o.push_str(&s);
    }
    if let Some(cx) = c { o.push(cx); }
    s.clear();
  };
  for c in txt.chars() {
    if LETTERS.contains(&c) || LETTERS_UPPER.contains(&c) {
      wordbuf.push(c);
    } else {
      do_word(&mut wordbuf, &mut outbuf, Some(c));
    }
  }
  do_word(&mut wordbuf, &mut outbuf, None);
  return outbuf;
}
