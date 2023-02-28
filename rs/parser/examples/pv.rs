// use pest::Parser;
// use pest::iterators::Pairs;
use glicol_parser::*;

fn main() {
    println!("{:?}", get_ast(r#"o: seq 60 >> sampler \cb 60 >> pv 0.1 1.0"#));
    // println!("{:?}", get_ast("o: envperc 1.0 2.0"));
    // get_ast(input);
    // let line = GlicolParser::parse(Rule::block, input);
    // println!("{:?}", line);
    // }
}