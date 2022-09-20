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

// let asmbly = r#"
// DATA R1, 0x00
// INC R1
// INC R1
// INC R1c
// DEC R1
// DEC R1
// "#;

// let asmbly = r#"
// STOP:
// DATA R1, 0x01
// DATA R2, 0x10
// JMPZ $STOP
// "#;

let asmbly = r#"
START:
    DATA R1, 2
    DATA R2, 2
    DATA R4, 2
    DATA R3, 1
ADDER:
    ADD R1, R2
    CMP R4, R3
    JMPIFZ $END
    DEC R4
    JMP $ADDER
END:
    HLT
"#;

    let mut parser = Parser::new(asmbly);
    parser.parse();
    //println!("{:?}", parser.labels);
    println!("{:#?}", parser.tokens);
    lexer::lex(parser.tokens, parser.labels)
}
