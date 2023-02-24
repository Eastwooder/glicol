use pest::Parser;
use pest::error::Error;
use pest_derive::*;
// use pest::iterators::Pair;
// use pest::error::ErrorVariant;
use hashbrown::HashMap;

use glicol_macros::{one_para_number_or_ref, two_numbers};
use glicol_synth::GlicolPara;
// extern crate meval;

#[derive(Parser)]
#[grammar = "glicol.pest"]
pub struct GlicolParser;

/// index, (vector of chain name, vector of parameter list)
pub type GlicolAst = HashMap<String, (Vec<String>, Vec<Vec<GlicolPara>>)>;

pub fn get_ast(code: &str) -> Result<GlicolAst, Error<Rule>> {
    let mut block = match GlicolParser::parse(Rule::block, code) {
        Ok(v) => v,
        Err(e) => { return Err(e) }
    };
    // this can be a comment though, but we call it a line
    let lines = block.next().unwrap();
    let mut ast = GlicolAst::new();
    ast
}