mod structures;
mod parser;
mod lexer;

use crate::parser::*;

// fn parse_number(s:&str) -> u8 {
//     let mut out: u8 = 0;
//     // minus one for zero index?
//     let mut num_chars = s.len();
//     let base = 10;
//
//     for c in s.chars() {
//         let n = match c {
//             '0' => 0,
//             '1' => 1,
//             '2' => 2,
//             '3' => 3,
//             '4' => 4,
//             '5' => 5,
//             '6' => 6,
//             '7' => 7,
//             '8' => 8,
//             '9' => 9,
//             _ => todo!()
//         };
//
//         // ???
//         // 100 = 1
//         // len = 3 base = 10 num = 1
//         out += u8::pow(base, (num_chars - 1) as u32) * n;
//         num_chars -= 1;
//     }
//
//     out
//
// }
// let v = parse_number("10");
// println!("v:{}", v);

fn main() {

    let asmbly = r#"
DATA R1, 2
JMPIFA 0x03
DATA R2, 3
ADD R1, R2
PRNT R2
DATA R1, 10
ADD R1, R2
PRNT R2
"#;

    let mut parser = Parser::new(asmbly);
    //println!("{:#?}\n\n", parser);
    parser.parse();
    lexer::lex(parser.tokens)
}
