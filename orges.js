
const ta_texto = document.getElementById("texto");
const ta_textorges = document.getElementById("textorges");
const ta_excessorges = document.getElementById("excessorges");
const rg_grorges = document.getElementById("grorges");

// caracteres usados para determinação de palavra. o hífen é... depressorges.
let LETRORGES = "abcdefghijklmnopqrstuvwxyzáàãâêéèíóõôúç-"
LETRORGES += LETRORGES.toLocaleUpperCase();

// umas palavras cujas conversões já vêm pré-encodadas.
const GABARITORGES = {
  "configuração": "configuraçorges"
};

// separa palavras
function palavrorges(texto) {
  let arr = [];
  let word = "";
  for (const c of texto) {
    if (LETRORGES.indexOf(c) > -1) {
      if (word.length > 0) {
        arr.push(word);
        word = "";
      }
    } else {
      word += c;
    }
  }
  return arr;
}

// tenta converter uma palavra em *orges a qualquer custo.
// retorna ou null, ou [palavra, palavra_convertida, custo], onde "custo" é uma
// float entre 0 e 1 denotando o quão "puxada" foi a conversorges.
function conversorges(palavra) {
  // primeiro, determinar se devemos adicionar ORGES ou orges.
  let sfx = "orges";
  let cost = 0;
  if (palavra == palavra.toLocaleUpperCase()) {
    sfx = "ORGES";
  }
  // passo 1: aplicar gabarito

}

// converte texto em textorges de acordo com um certo... grau
function orges(texto, grau) {
  
}

// pegar texto da textarea de entrada
function in_texto() {
  return ta_texto.value;
}

// pegar grorges (int de 0 a 100)
function in_grorges() {
  return parseInt(rg_grorges.value);
}

// pegar palavras-excessorges
function in_excessorges() {
  return palavrorges(ta_excessorges.value);
}

// atualizar textorges
function updatorges() {
  const out_text = orges(in_texto());
  ta_textorges.value = out_text;
}

function clicorges() {
  ta_textorges.focus();
  ta_textorges.select();
  document.execCommand("copy");
}
