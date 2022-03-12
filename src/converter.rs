//! COnverter UI.

use web_sys::{InputEvent, HtmlTextAreaElement, HtmlInputElement};
use yew::{html, Component, TargetCast};
use crate::orges::caching::Cachorges;

pub(crate) struct Converter {
  in_text: String,
  out_text: String,
  rate: f64,
  caching: Cachorges
}

#[derive(Debug)]
pub(crate) enum ConverterMsg {
  TextChanged(String),
  RateChanged(f64),
  Redo
}

impl Converter {
  fn update_inner(&mut self) {
    //self.out_text = crate::orges::orges_string(&self.in_text, self.rate, crate::orges::orges_word_case);
    self.out_text = self.caching.orges_string(&self.in_text, self.rate);
  }
}

impl Component for Converter {
  type Message = ConverterMsg;
  type Properties = ();

  fn create(_ctx: &yew::Context<Self>) -> Self {
    return Self {
      in_text: "".to_owned(),
      out_text: "".to_owned(),
      rate: 1.0,
      caching: Cachorges::default()
    }
  }

  fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
    match msg {
      ConverterMsg::TextChanged(s) => {
        if self.in_text != s {
          self.in_text = s;
          self.update_inner();
          return true;
        }
      },
      ConverterMsg::RateChanged(x) => {
        if self.rate != x {
          self.rate = x;
          self.update_inner();
          return true;
        }
      },
      ConverterMsg::Redo => {
        self.update_inner();
        return true;
      }
    }
    return false;
  }

  fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
    let ic = ctx.link().callback(|e: InputEvent| {
      let tgt: HtmlTextAreaElement = (*e).target_unchecked_into();
      let s = HtmlTextAreaElement::value(&tgt);
      return ConverterMsg::TextChanged(s);
    });
    let rc = ctx.link().callback(|e: InputEvent| {
      let tgt: HtmlInputElement = (*e).target_unchecked_into();
      let s: String = HtmlInputElement::value(&tgt);
      let x: f64 = s.parse().unwrap_or(0.0);
      return ConverterMsg::RateChanged(x/100.0);
    });
    let re = ctx.link().callback(|_| {
      return ConverterMsg::Redo;
    });
    return html! {
      <div class="converter">
        <center>
          <h1>{ "orges" }</h1>
          { "eu não sei por que fiz isso." }
        </center>
        <br/>
        <input
          class="converter-range"
          type="range"
          min="0"
          max="100"
          value={ format!("{}", self.rate*100.0) }
          step="1"
          oninput={ rc }
        />
        <br/>
        {
          format!(
            "{}: {}%",
            if self.rate > 0.99 { "taxorges" } else { "taxa" },
            (self.rate*100.0) as usize
          )
        }
        <br/>
        <button
          class="converter-btn"
          onclick={ re }
        >
          { "refazer" }
        </button>
        <br/>
        <textarea
          class="converter-text"
          oninput={ ic }
        >
        </textarea>
        <br/>
        <textarea
          class="converter-text"
          readonly=true
          value={ self.out_text.clone() }
        >
        </textarea>
      </div>
    };
  }
}
