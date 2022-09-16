mod structures;
mod parser;
mod lexer;

use crate::parser::*;

fn main() {

// let asmbly = r#"
// DATA R1, 0x00
// DATA R2, 0xff
// ST R1, R2
// LD R1, R1
// "#;

let asmbly = r#"
DATA R1, 0x00
INC R1
INC R1
INC R1
DEC R1
DEC R1
"#;

/*
DATA ...
ADD...


start: LABEL // add "start" to a hashmap and store the len of operations vector for this label
SUB R1,R2 // index 3 in operations vector


print:
JMP 0xCC
JMP [start] // at compile time, swap out label for the index specified in the label vector

display:
*/

    let mut parser = Parser::new(asmbly);
    //println!("{:#?}\n\n", parser);
    parser.parse();
    lexer::lex(parser.tokens)
}
