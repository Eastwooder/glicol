use pest::Parser;
use pest::error::Error;
use pest_derive::*;
// use pest::iterators::Pair;
// use pest::error::ErrorVariant;
use hashbrown::HashMap;

// use glicol_macros::{one_para_number_or_ref, two_numbers};
use glicol_synth::GlicolPara;


#[derive(Parser)]
#[grammar = "glicol.pest"]
pub struct GlicolParser;

pub type GlicolAst = HashMap<String, (Vec<String>, Vec<Vec<GlicolPara>>)>;

pub fn get_ast(code: &str) -> Result<GlicolAst, Error<Rule>> {
    let mut block = match GlicolParser::parse(Rule::block, code) {
        Ok(v) => v,
        Err(e) => { return Err(e) }
    };

    let mut sentences = block.next().unwrap();

    let mut ast = HashMap::new();

    for sentence in sentences.into_inner() {
        match sentence.as_rule() {
            Rule::sentence => {
                println!("a new track parsed {:?}", sentence.as_str());
                let mut key = "";
                let mut chain_node_names = vec![];
                let mut chain_paras = vec![];
                for track_component in sentence.into_inner() {
                    match track_component.as_rule() {
                        Rule::reference => {
                            println!("the name of the track is {:?}", track_component.as_str());
                            key = track_component.as_str();
                        },
                        Rule::chain => {
                            let chain = track_component;
                            for node_pair in chain.into_inner() {
                                let node = node_pair.into_inner().next().unwrap();
                                match node.as_rule() {
                                    Rule::sin => {
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("sin");
                                        chain_paras.push(vec![GlicolPara::Number(paras.as_str().parse::<f32>().unwrap())]);
                                    },
                                    _ => {}
                                }
                                
                            }
                        },
                        _ => {}
                    };
                }
                ast.insert(
                    key.to_owned(), 
                    (
                        chain_node_names.iter_mut().map(|x|x.to_owned()).collect::<Vec<String>>(), 
                        chain_paras
                    )
                );
            },
            
            _ => {}
        };
    }


    Ok(ast)
}