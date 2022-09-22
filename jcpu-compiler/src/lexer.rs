#![allow(dead_code, unused_imports, unused_assignments)]
use std::{any::Any, fs, iter::Peekable, slice::Iter, collections::HashMap};

use jcpuinstructions::{Instruction, JumpFlag, Register, JUMP_FLAGS};

use crate::structures::{Token, TokenType};

const MAX_RULES: usize = 13;

static RULES: [(&str, Instruction, Option<TokenType>, Option<TokenType>, usize); MAX_RULES] = [
    ("data",Instruction::DATA,Some(TokenType::Identifier),Some(TokenType::Value),2),
    ("ld",Instruction::LD,Some(TokenType::Identifier),Some(TokenType::Identifier),2),
    ("st",Instruction::ST,Some(TokenType::Identifier),Some(TokenType::Identifier),2),
    ("add",Instruction::ADD,Some(TokenType::Identifier),Some(TokenType::Identifier),1),
    ("sub",Instruction::SUB,Some(TokenType::Identifier),Some(TokenType::Identifier),1),
    ("cmp", Instruction::CMP, Some(TokenType::Identifier), Some(TokenType::Identifier),1),
    ("inc", Instruction::INC, Some(TokenType::Identifier), None,1),
    ("dec", Instruction::DEC, Some(TokenType::Identifier), None,1),
    ("jmpr", Instruction::JMPR, Some(TokenType::LabelDst), None,1),
    ("jmp", Instruction::JMP, Some(TokenType::LabelDst), None,2),
    ("jmpif", Instruction::JMPIF, Some(TokenType::LabelDst), None,2),
    ("clf", Instruction::CLF, None, None,1),
    ("hlt", Instruction::HLT, None, None,1)
];

fn rule_for_op(op: &str) -> Option<(&str, u8, Option<TokenType>, Option<TokenType>,usize)> {
    let opname = op.to_string().to_lowercase();
    //println!("op: {}", opname);

    // handle jmpif flags
    for rule in RULES.iter() {
        if rule.0 == "jmpif" && opname.contains("jmpif") {
            if let Some(flagstr) = opname.split("jmpif").last() {
                //println!("last: {}", flagstr);

                for (i, flag) in JUMP_FLAGS.iter().enumerate() {
                    if flag == &flagstr.to_lowercase().as_str() {
                        return Some((rule.0, (Instruction::JMPIF as u8) | i as u8 , rule.2.clone(), None, rule.4));
                    }
                }

                panic!("unknown jump flag on jumpif");
            };
        } else if rule.0 == opname {
            return Some((rule.0, rule.1.clone() as u8, rule.2.clone(), rule.3.clone(), rule.4 ));
        }
    }
    None
}


fn get_register(token: &Token) -> Register {
    let token_value = token.tvalue.clone();

    match token_value.to_lowercase().as_str() {
        "r1" => Register::R1,
        "r2" => Register::R2,
        "r3" => Register::R3,
        "r4" => Register::R4,
        _ => panic!(
            "Syntax error unknown Register: {}, line: {}, column: {}",
            token.tvalue, token.line, token.column
        ),
    }
}

fn get_value(token: &Token) -> u8 {
    let int_val = token.tvalue.parse::<u8>();

    if int_val.is_ok() {
        int_val.unwrap()
    } else {
        panic!(
            "Syntax error invalid u8 number value: {}, line: {}, column: {}",
            token.tvalue, token.line, token.column
        )
    }
}

pub fn lex(tokens: Vec<Token>) {
    let mut peekable_tokens = tokens.iter().peekable();
    let mut operations: Vec<(&str, u8, Option<Token>, Option<Token>)> = Vec::new();
    let mut addresses: HashMap<String, usize> = HashMap::new();
    let mut op_address = 0;

    // This could probably be improved, but iterate over the list and gather
    // a list of addresses from the labels
    for tok in &tokens {
        if tok.ttype == TokenType::LabelSrc {
            addresses.insert(tok.tvalue.clone(), op_address);   
        } else if let Some(op) = rule_for_op(tok.tvalue.as_str()) {
            // we only increment if we are an op and not a label src or another type of token
            // we also increment the number of op size in bytes.
            op_address += op.4;
        }

    }

println!("LAbels: {:?}", addresses);
    // Process the code line by line (imperative)
    while let Some(token) = peekable_tokens.next() {
        // Skip label sources
        if token.ttype == TokenType::LabelSrc {
            continue;
        }
        
        if let Some((opname, op, lval, rval, _opsize)) = rule_for_op(token.tvalue.as_str()) {
            /*
                If lval and rval is_some, then we expect 3 tokens:
                    the lval and correct type, the comma and then rval and correct type
                if lval is some and rval is none, we expect 1 more token and the corect type
                if lval is none and rval is some, someone is a fuckign idiot
            */


            let mut tlval: Option<&Token> = None;
            let mut trval: Option<&Token> = None;
 

            if lval.is_some() && rval.is_some() {
                tlval = peekable_tokens.next();
                let tcomm = peekable_tokens.next();
                trval = peekable_tokens.next();

                if tcomm.unwrap().ttype != TokenType::Comma {
                    panic!(
                        "Syntax error comma required to seperate arguments. line: {}, column: {}",
                        token.line,
                        (token.column + tlval.unwrap().tvalue.len())
                    );
                }

                // check for register value
                if let Some(tlval) = tlval {
                    if let Some(lval) = lval {
                        if tlval.ttype != lval {
                            panic!("syntax error was expecting token type: {:?}, but received {:?}. line: {}, column: {}", lval, tlval.ttype, tlval.line, (tlval.column - tlval.tvalue.len() - 1));
                        }
                    }
                }
                if let Some(trval) = trval {
                    if let Some(rval) = rval {
                        if trval.ttype != rval {
                            panic!("syntax error was expecting token type: {:?}, but received {:?}. line: {}, column: {}", rval, trval.ttype, trval.line, (trval.column - trval.tvalue.len() - 1));
                        }
                    }
                }

                let a = tlval.unwrap().clone();
                let b = trval.unwrap().clone();

                operations.push((opname, op.clone() as u8, Some(a), Some(b)))
            } else if lval.is_some() && rval.is_none() {
                let tlval = peekable_tokens.next();

                if let Some(tlval) = tlval {
                    if let Some(lval) = lval {
                        if tlval.ttype != lval {
                            panic!("syntax error was expecting token type: {:?}, but received {:?}. line: {}, column: {}", lval, tlval.ttype, tlval.line, (tlval.column - tlval.tvalue.len()));
                        }
                    }
                }

                if let Some(tlval) = tlval {
                    if tlval.ttype == TokenType::LabelDst {
                        let next_token = peekable_tokens.next();

                        if let Some(ntoken) = next_token {
                            match ntoken.ttype {
                                TokenType::Identifier => {
                                    /*
                                    we have two conditions:
                                    first, check if a label exists, if so, add the value to the op like the below
                                    if not, then add the identifier as a register

                                    cases:
                                    JMP $start  // identifier
                                    JMP $0xc // Value
                                    */
                                    
                                    // Let's find the address in our list
                                    if let Some(address) = addresses.get(&ntoken.tvalue) {
                                        let a = Token {
                                            ttype: TokenType::Value,
                                            tvalue: address.to_string(),
                                            line: ntoken.line,
                                            column: ntoken.column
                                       };
                                       operations.push((opname, op.clone() as u8, Some(a), None))
                                    } else {
                                        panic!("unknown address {} specified at line: {}, col: {}", &ntoken.tvalue, tlval.line, (tlval.column - tlval.tvalue.len()));   
                                    } 
                                },
                                TokenType::Value => {
                                    operations.push((opname, op.clone() as u8, Some(ntoken.clone()), None))
                                },
                                _ => panic!("write your error memssage here that we receved a garbage label")
                            }
                        } else {
                            //@TODO: error
                        }
                    } else {
                        let a = tlval.clone();

                        operations.push((opname, op.clone() as u8, Some(a), None))
                    }
                }
            } else if lval.is_none() && rval.is_none() {
                operations.push((opname, op.clone() as u8, None, None))
            } else if lval.is_none() && rval.is_some() {
                //panic!("Syntax error left value cannot be nothing, idiot...")
            }
        } else {
            panic!(
                "Syntax error, unknown operation. line: {}, column: {}",
                token.line, token.column
            );
        }

        op_address += 1;
    }
 
    // run compile function
    compile(operations)
}

// -> Vec<u8>
fn compile(vec: Vec<(&str, u8, Option<Token>, Option<Token>)>)  {
    let mut bin_operations: Vec<u8> = Vec::new();

    for op in vec.iter() {
        //println!("op: {}", op.0);
        match op.0 {
            "data" => {
                // u8|u8 packed, next byte u8
                // will panic if not register here
                let l_register = get_register(op.2.as_ref().unwrap());
                let r_value = get_value(op.3.as_ref().unwrap());

                bin_operations.push((op.1.clone() as u8) | (l_register as u8) << 2);
                bin_operations.push(r_value);
                // need to get next byte
            },
            "add" | "sub" | "ld" | "st" | "cmp" => {
                // u8|u8|u8 packed
                // will panic if not register here
                let l_register = get_register(op.2.as_ref().unwrap());
                let r_register = get_register(op.3.as_ref().unwrap());
                // println!("{:08b}", op.1);
                bin_operations.push( (op.1.clone() as u8) | (l_register as u8) << 2 | (r_register as u8) << 0 )
            },
            "jmpr" | "dec" | "inc" => {
                // u8|u8 packed
                // will panic if not register here
                let l_register = get_register(op.2.as_ref().unwrap());
                bin_operations.push( (op.1.clone() as u8) | (l_register as u8) << 2 )
            },
            // "jmp"
            "jmpif" | "jmp" => {
                let l_value = get_value(op.2.as_ref().unwrap());

                println!("jmp val: {}", l_value as u8);
                bin_operations.push(op.1.clone());
                bin_operations.push(l_value as u8);
                // jmpif 0x04
            },
            "clf" | "hlt" => bin_operations.push(op.1.clone()),
            _ => todo!()
        }
    }

    // println!("{:?}", bin_operations);
    for op in &bin_operations {
        println!("{:08b}", op)
    }

    write_file(&mut bin_operations);
}

fn write_file(raw_ops: &mut Vec<u8>) {
    fs::write("boot.img", raw_ops).expect("Unable to write file");
}
