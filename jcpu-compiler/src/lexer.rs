#![allow(dead_code, unused_imports, unused_assignments)]
use std::{any::Any, fs, iter::Peekable, slice::Iter};

use jcpuinstructions::{Instruction, JumpFlag, Register};
use regex::internal::Inst;

use crate::structures::{Token, TokenType};

const MAX_RULES: usize = 9;

static RULES: [(&str, Instruction, Option<TokenType>, Option<TokenType>); MAX_RULES] = [
    (
        "data",
        Instruction::DATA,
        Some(TokenType::Identifier),
        Some(TokenType::Value),
    ),
    (
        "ld",
        Instruction::LD,
        Some(TokenType::Identifier),
        Some(TokenType::Value),
    ),
    (
        "st",
        Instruction::ST,
        Some(TokenType::Identifier),
        Some(TokenType::Value),
    ),
    (
        "add",
        Instruction::ADD,
        Some(TokenType::Identifier),
        Some(TokenType::Identifier),
    ),
    (
        "sub",
        Instruction::SUB,
        Some(TokenType::Identifier),
        Some(TokenType::Identifier),
    ),
    ("jmpr", Instruction::JMPR, Some(TokenType::Identifier), None),
    ("jmp", Instruction::JMP, Some(TokenType::Value), None),
    ("jmpif", Instruction::JMPIF, Some(TokenType::Value), None),
    ("prnt", Instruction::PRNT, Some(TokenType::Identifier), None),
];

fn rule_for_op(op: &str) -> Option<(&str, Instruction, Option<TokenType>, Option<TokenType>)> {
    // handle jmpif flags
    let mut opname = op.to_string();
    if opname.contains("JMPIF") {
        let jmp_flags = String::new();
        opname = "jmpif".to_string();

        if let Some(flags) = op.split("JMPIF").last() {
            println!("Jmp flags: {} ", flags);
        }
    }

    for rule in RULES.iter() {
        if rule.0 == opname.to_lowercase() {
            return Some((rule.0, rule.1.clone(), rule.2.clone(), rule.3.clone()));
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
    let mut operations: Vec<(&str, Instruction, Option<Token>, Option<Token>)> = Vec::new();

    // Process the code line by line (imperative)
    while let Some(token) = peekable_tokens.next() {
        if let Some((opname, op, lval, rval)) = rule_for_op(token.tvalue.as_str()) {
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
                } else {
                    panic!(
                        "Syntax error was expecting token type: {:?}, but received {:?}",
                        lval,
                        tlval.unwrap().ttype
                    );
                }

                if let Some(trval) = trval {
                    if let Some(rval) = rval {
                        if trval.ttype != rval {
                            panic!("syntax error was expecting token type: {:?}, but received {:?}. line: {}, column: {}", rval, trval.ttype, trval.line, (trval.column - trval.tvalue.len() - 1));
                        }
                    }
                } else {
                    panic!(
                        "Syntax error was expecting token type: {:?}, but received {:?}",
                        rval,
                        trval.unwrap().ttype
                    );
                }

                let a = tlval.unwrap().clone();
                let b = trval.unwrap().clone();

                operations.push((opname, op.clone(), Some(a), Some(b)))
            } else if lval.is_some() && rval.is_none() {
                let tlval = peekable_tokens.next();

                if let Some(tlval) = tlval {
                    if let Some(lval) = lval {
                        if tlval.ttype != lval {
                            panic!("syntax error was expecting token type: {:?}, but received {:?}. line: {}, column: {}", lval, tlval.ttype, tlval.line, (tlval.column - tlval.tvalue.len()));
                        }
                    }
                } else {
                    panic!(
                        "Syntax error was expecting token type: {:?}, but received {:?}",
                        lval,
                        tlval.unwrap().ttype
                    );
                }

                let a = tlval.unwrap().clone();

                operations.push((opname, op.clone(), Some(a), None))
            } else if lval.is_none() && rval.is_some() {
                panic!("Syntax error left value cannot be nothing, idiot...")
            }
        } else {
            panic!(
                "Syntax error, unknown operation. line: {}, column: {}",
                token.line, token.column
            );
        }
    }

    //println!("ops: \n {:?}", operations);
    // run compile function
    compile(operations)
}

// -> Vec<u8>
fn compile(vec: Vec<(&str, Instruction, Option<Token>, Option<Token>)>) {
    let mut bin_operations: Vec<u8> = Vec::new();

    for op in vec.iter() {
        let mut opname = op.0.clone();
        let mut jmp_flags = String::new();

        if op.0.contains("jmpif") {
            opname = "jmpif";
            if let Some(flags) = op.0.split("jmpif").last() {
                jmp_flags = flags.to_string();
            }
        }

        match opname {
            "data" | "ld" | "st" => {
                // u8|u8 packed, next byte u8
                // will panic if not register here
                let l_register = get_register(op.2.as_ref().unwrap());
                let r_value = get_value(op.3.as_ref().unwrap());

                bin_operations.push((op.1.clone() as u8) << 4 | (l_register as u8) << 2);
                bin_operations.push(r_value);
                // need to get next byte
            }
            "add" | "sub" => {
                // u8|u8|u8 packed
                // will panic if not register here
                let l_register = get_register(op.2.as_ref().unwrap());
                let r_register = get_register(op.3.as_ref().unwrap());

                bin_operations.push(
                    (op.1.clone() as u8) << 4 | (l_register as u8) << 2 | (r_register as u8) << 0,
                )
            }
            "prnt" | "jmp" | "jmpr" => {
                // u8|u8 packed
                // will panic if not register here
                let l_register = get_register(op.2.as_ref().unwrap());
                bin_operations.push((op.1.clone() as u8) << 4 | (l_register as u8) << 2)
            }
            "jmpif" => {
                // jmpif 0x04,
                println!("{}", jmp_flags);
            }
            _ => todo!(),
        }
    }

    // println!("{:?}", bin_operations);
    for op in &bin_operations {
        println!("{:08b}", op)
    }

    write_file(0x41, &mut bin_operations);
}

fn write_file(boot_flag: u8, raw_ops: &mut Vec<u8>) {
    raw_ops.insert(0, boot_flag);
    fs::write("boot.img", raw_ops).expect("Unable to write file");
}

/*
Add line column (+ token length) for erros
refactor to use jcpu instructions lib

write instructions to file
add a function to write to file
 - pass a boot flag a 1 byte value as arg
 - then write to file
*/
