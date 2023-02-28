pub use pest::Parser;
pub use pest::error::Error;
pub use pest_derive::*;
pub use pest::iterators::Pair;
// use pest::error::ErrorVariant;
use hashbrown::HashMap;
use evalexpr::*;

// use glicol_macros::{one_para_number_or_ref, two_numbers};
pub use glicol_synth::GlicolPara;

#[derive(Parser)]
#[grammar = "glicol.pest"]
pub struct GlicolParser;

pub type GlicolAst = HashMap<String, (Vec<String>, Vec<Vec<GlicolPara>>)>;

pub fn get_ast(code: &str) -> Result<GlicolAst, Error<Rule>> {
    let mut block = match GlicolParser::parse(Rule::block, code) {
        Ok(v) => v,
        Err(e) => { return Err(e) }
    };

    let sentences = block.next().unwrap();

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

                                    Rule::mul => {
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("mul");
                                        chain_paras.push(vec![GlicolPara::Number(paras.as_str().parse::<f32>().unwrap())]);
                                    },

                                    Rule::vocoder => {
                                        println!("node {:?}", node.as_str());
                                        let mut params = node.into_inner();
                                        let para_a = params.next().unwrap();
                                        let para_b = params.next().unwrap();
                                        
                                        chain_node_names.push("vocoder");
                                        chain_paras.push(vec![
                                            GlicolPara::Number(para_a.as_str().parse::<f32>().unwrap()),
                                            GlicolPara::Number(para_b.as_str().parse::<f32>().unwrap()),
                                        ]);
                                    },
                                    Rule::sampler => {
                                        // paras of samplers
                                        let mut params = node.into_inner();
                                        let name = params.next().unwrap();
                                        let trigger = eval(params.next().unwrap().as_str()).unwrap().as_number().unwrap() as f32;
                                        // let start = params.next().unwrap();
                                        // let end = params.next().unwrap();
                                        // let attack = params.next().unwrap();
                                        // let decay = params.next().unwrap();
                                        // let ps = params.next().unwrap();
                                        // let ts = params.next().unwrap();

                                        // println!("trigger {:?}", trigger.as_str());
                                        chain_node_names.push("sampler");
                                        chain_paras.push(vec![
                                            GlicolPara::SampleSymbol(name.as_str().to_owned()),
                                            GlicolPara::Number(trigger),
                                            // GlicolPara::Number(start.as_str().parse::<f32>().unwrap()),
                                            // GlicolPara::Number(end.as_str().parse::<f32>().unwrap()),
                                            // GlicolPara::Number(attack.as_str().parse::<f32>().unwrap())
                                            // GlicolPara::Number(decay.as_str().parse::<f32>().unwrap())
                                            // GlicolPara::Number(ps.as_str().parse::<f32>().unwrap()),
                                            // GlicolPara::Number(ts.as_str().parse::<f32>().unwrap()),
                                        ]);
                                    },
                                    Rule::seq => {
                                        let mut event = Vec::<(f32, GlicolPara)>::new();
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("seq");
                                        // to do, more than a symbol
                                        // should be an event that contains time and note
                                        // GlicolPara::Symbol(paras.as_str())
                                        let compounds: Vec<_> = paras.collect();
                                        // one bar will firstly be divided here
                                        let compounds_num = compounds.len();
                                        println!("compounds_num {:?}", compounds_num);
                                        for (i, compound) in compounds.into_iter().enumerate() {
                                            let relative_time_base = i as f32 /compounds_num as f32;

                                            println!("compound {:?}", compound.as_str());
                                            let elements: Vec<_> = compound.into_inner().collect();
                                            let elements_n = elements.len();
                                            
                                            for (j, element) in elements.into_iter().enumerate() {
                                                let relative_time_sub = 1./ compounds_num as f32 * j as f32 / elements_n as f32;
                                                let e = element; //.into_inner().next().unwrap();
                                                let time = relative_time_sub + relative_time_base;
                                                match e.as_rule() {
                                                    Rule::midi => {
                                                        event.push( (time, GlicolPara::Number(e.as_str().parse::<f32>().unwrap()) ));
                                                        println!("int {:?}", e.as_str());
                                                    },
                                                    Rule::rest => {
                                                        println!("rest {:?}", e.as_str());
                                                        // event.push( (time , GlicolPara::Number(0.0) ));
                                                    },
                                                    Rule::note_ref => {
                                                        println!("ref {:?}", e.as_str());
                                                        event.push( (time, GlicolPara::Reference(e.as_str().to_owned()) ));
                                                    },
                                                    _=> unimplemented!()
                                                }
                                            }
                                        }
                                        chain_paras.push(vec![GlicolPara::Sequence(event)]);
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