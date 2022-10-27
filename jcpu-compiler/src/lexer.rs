#![allow(dead_code, unused_imports, unused_assignments)]
use std::{any::Any, fs, iter::Peekable, slice::Iter, collections::HashMap, fmt::format};

use jcpuinstructions::{Instruction, JumpFlag, Register, JUMP_FLAGS};

use crate::structures::{Token, TokenType};

type OpType = (&'static str, Instruction, Vec<TokenType>, Vec<TokenType>, usize);

fn rule_for_op(op: &str) -> Option<(&str, u8, Vec<TokenType>, Vec<TokenType>,usize)> {
    //@TODO make l/r values vectors of options to have more options per token
    let rules: Vec<OpType> = Vec::from([
        ("data",Instruction::DATA,vec![TokenType::Identifier], vec![TokenType::Value],2),
        ("ld",Instruction::LD,vec![TokenType::Identifier],vec![TokenType::Identifier],2),
        ("st",Instruction::ST,vec![TokenType::Identifier],vec![TokenType::Identifier],2),
        ("add",Instruction::ADD,vec![TokenType::Identifier],vec![TokenType::Identifier],1),
        ("sub",Instruction::SUB,vec![TokenType::Identifier],vec![TokenType::Identifier],1),
        ("cmp", Instruction::CMP, vec![TokenType::Identifier], vec![TokenType::Identifier, TokenType::Value],2),
        ("inc", Instruction::INC, vec![TokenType::Identifier], vec![],1),
        ("pop", Instruction::POP, vec![TokenType::Identifier], vec![],1),
        ("push", Instruction::PUSH, vec![TokenType::Identifier], vec![],1),
        ("dec", Instruction::DEC, vec![TokenType::Identifier], vec![],1),
        ("jmpr", Instruction::JMPR, vec![TokenType::LabelDst], vec![],1),

        ("int", Instruction::INT, vec![TokenType::Value], vec![],2),
        ("jmp", Instruction::JMP, vec![TokenType::LabelDst], vec![],2),
        ("jmpif", Instruction::JMPIF, vec![TokenType::LabelDst], vec![],2),
        ("sf", Instruction::SF, vec![TokenType::Value], vec![], 2),
        ("cli", Instruction::CLI, vec![], vec![],1),
        ("clf", Instruction::CLF, vec![], vec![],1),
        ("crf", Instruction::CRF, vec![], vec![],1),
        ("hlt", Instruction::HLT, vec![], vec![],1)
    ]);
    let opname = op.to_string().to_lowercase();
    //println!("op: {}", opname);

    // handle jmpif flags
    for rule in rules.iter() {
        if rule.0 == "jmpif" && opname.contains("jmpif") {
            if let Some(flagstr) = opname.split("jmpif").last() {
                //println!("last: {}", flagstr);
                for (i, flag) in JUMP_FLAGS.iter().enumerate() {
                    if flag == &flagstr.to_lowercase().as_str() {
                        return Some((rule.0, (Instruction::JMPIF as u8) | i as u8 , rule.2.clone(), vec![], rule.4));
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

pub fn lex(tokens: Vec<Token>, output_path: String){
    let mut peekable_tokens = tokens.iter().peekable();
    let mut operations: Vec<(&str, u8, Option<Token>, Option<Token>)> = Vec::new();
    let mut addresses: HashMap<String, usize> = HashMap::new();
    let mut debug_ops: Vec<String> = Vec::new();
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

    op_address = 0;

    // Process the code line by line (imperative)
    while let Some(token) = peekable_tokens.next() {
        // Skip label sources
        if token.ttype == TokenType::LabelSrc {
            //addresses.insert(token.tvalue.clone(), op_address);
            debug_ops.push(format!("{}: {}:", op_address, token.tvalue));
            continue;
        }

        if let Some((opname, op, left_values, right_values, _opsize)) = rule_for_op(token.tvalue.as_str()) {
            /*
                If lval and rval is_some, then we expect 3 tokens:
                    the lval and correct type, the comma and then rval and correct type
                if lval is some and rval is none, we expect 1 more token and the corect type
                if lval is none and rval is some, someone is a fuckign idiot
            */

            let mut left_token_option: Option<&Token> = None;
            let mut right_token_option: Option<&Token> = None;


            if !left_values.is_empty() && !right_values.is_empty() {

                left_token_option = peekable_tokens.next();
                let tcomm = peekable_tokens.next();
                right_token_option = peekable_tokens.next();

                if tcomm.unwrap().ttype != TokenType::Comma {
                    panic!(
                        "Syntax error comma required to seperate arguments. line: {}, column: {}",
                        token.line,
                        (token.column + left_token_option.unwrap().tvalue.len())
                    );
                }

                // check for register value
                if let Some(left_token) = left_token_option {
                    if !left_values.contains(&left_token.ttype) {
                        panic!("syntax error was expecting token type: {:?}, but received {:?}. line: {}, column: {}", left_values, left_token.ttype, left_token.line, (left_token.column - left_token.tvalue.len() - 1));
                    }
                }

                if let Some(right_token) = right_token_option {
                    if !right_values.contains(&right_token.ttype)  {
                        panic!("syntax error was expecting token type: {:?}, but received {:?}. line: {}, column: {}", right_values, right_token.ttype, right_token.line, (right_token.column - right_token.tvalue.len() - 1));
                    }
                }

                let a = left_token_option.unwrap().clone();
                // if a.ttype == TokenType::Value {
                //     // check if ttype is identifier or value
                //     // if value add sf instruction
                //     //
                //     let sf_1 = Token {
                //         ttype: TokenType::Value,
                //         tvalue: String::from("64"),
                //         line: right_token_option.unwrap().line,
                //         column: right_token_option.unwrap().column
                //     };
                //     op_address += 1;
                //     operations.push(("sf", 0b00000010, Some(sf_1), None));
                // }

                let b = right_token_option.unwrap().clone();
                if opname == "cmp" {
                    if b.ttype == TokenType::Value {
                        // check if ttype is identifier or value
                        // if value add sf instruction
                        let sf_2 = Token {
                            ttype: TokenType::Value,
                            tvalue: String::from("32"),
                            line: right_token_option.unwrap().line,
                            column: right_token_option.unwrap().column
                        };

                        debug_ops.push(format!("{}: {} {}", op_address, "SF".to_string(), &sf_2.tvalue));
                        operations.push(("sf", 0b00000010, Some(sf_2), None));
                        op_address += 2;

                    }

                    debug_ops.push(format!("{}: {} {}, {}", op_address, &opname.to_uppercase(), &a.tvalue, &b.tvalue));
                    operations.push((opname, op.clone() as u8, Some(a), Some(b)));

                    op_address += 1;
                    debug_ops.push(format!("{}: {}", op_address, "CRF".to_string()));
                    operations.push(("crf", 0b00000100, None, None))
                } else {
                    debug_ops.push(format!("{}: {} {}, {}", op_address, &opname.to_uppercase(), &a.tvalue, &b.tvalue));
                    operations.push((opname, op.clone() as u8, Some(a), Some(b)))
                }

            } else if !left_values.is_empty() && right_values.is_empty() {
                let tlval = peekable_tokens.next();

                if let Some(left_token) = left_token_option {
                    if !left_values.contains(&left_token.ttype) {
                        panic!("syntax error was expecting token type: {:?}, but received {:?}. line: {}, column: {}", left_values, left_token.ttype, left_token.line, (left_token.column - left_token.tvalue.len() - 1));
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

                                        //@FIXME: Duplicate code
                                        if opname == "jmpif" {
                                            let f = Vec::from(JUMP_FLAGS);
                                            let index= (op.clone() & 0b00001111) as usize;
                                            let instruction_flags = f[index];
                                            let ins = format!("{}{}",opname.to_uppercase(), instruction_flags.to_uppercase());
                                            debug_ops.push(format!("{}: {} ${}", op_address, ins, ntoken.tvalue));
                                        } else {
                                            debug_ops.push(format!("{}: {} ${}", op_address, opname.to_uppercase(), ntoken.tvalue));
                                        }
                                        operations.push((opname, op.clone() as u8, Some(a), None))
                                    } else {
                                        panic!("unknown address {} specified at line: {}, col: {}", &ntoken.tvalue, tlval.line, (tlval.column - tlval.tvalue.len()));
                                    }
                                },
                                TokenType::Value => {
                                    //@FIXME: Duplicate code
                                    if opname == "jmpif" {
                                        let f = Vec::from(JUMP_FLAGS);
                                        let index= (op.clone() & 0b00001111) as usize;
                                        let instruction_flags = f[index];
                                        let ins = format!("{}{}",opname.to_uppercase(), instruction_flags.to_uppercase());
                                        debug_ops.push(format!("{}: {} ${}", op_address, ins, ntoken.tvalue));
                                    } else {
                                        debug_ops.push(format!("{}: {} ${}", op_address, opname.to_uppercase(), ntoken.tvalue));
                                    }
                                    operations.push((opname, op.clone() as u8, Some(ntoken.clone()), None))
                                },
                                _ => panic!("write your error memssage here that we receved a garbage label")
                            }
                        } else {
                            //@TODO: error
                        }
                    } else {
                        let a = tlval.clone();

                        debug_ops.push(format!("{}: {} {}", op_address, opname.to_uppercase(), a.tvalue));
                        operations.push((opname, op.clone() as u8, Some(a), None))
                    }
                }
            } else if left_values.is_empty() && right_values.is_empty() {
                debug_ops.push(format!("{}: {}", op_address, opname.to_uppercase()));
                operations.push((opname, op.clone() as u8, None, None))
            } else if left_values.is_empty() && !right_values.is_empty() {
                //panic!("Syntax error left value cannot be nothing, idiot...")
            }

            op_address += _opsize;
        } else {
            panic!(
                "Syntax error, unknown operation. line: {}, column: {}",
                token.line, token.column
            );
        }

    }

    //run compile function
    // println!("{:#?}", addresses);
    // println!("DEBUG \n {:#?}", debug_ops);
    compile(operations, output_path);
    write_debug_file(debug_ops)
}

fn compile(vec: Vec<(&str, u8, Option<Token>, Option<Token>)>, output_path: String)  {
    let mut bin_operations: Vec<u8> = Vec::new();

    //println!("COMPILED OPS: {:#?}", vec);

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
            "add" | "sub" | "ld" | "st" => {
                // u8|u8|u8 packed
                // will panic if not register here
                let l_register = get_register(op.2.as_ref().unwrap());
                let r_register = get_register(op.3.as_ref().unwrap());
                // println!("{:08b}", op.1);
                bin_operations.push( (op.1.clone() as u8) | (l_register.clone() as u8) << 2 | (r_register.clone() as u8) << 0 );
            },
            "cmp" => {
                let l_token = op.2.as_ref().unwrap();
                let r_token = op.3.as_ref().unwrap();
                let r_val: u8;

                if r_token.ttype == TokenType::Identifier {
                    r_val = get_register(r_token) as u8;
                } else {
                    r_val = get_value(r_token);
                }

                // println!("{:08b}", op.1);
                bin_operations.push( (op.1.clone() as u8) | (get_register(l_token) as u8) << 2 );
                // cmp can compare a register to a register or a value
                bin_operations.push(r_val);
            },
            "jmpr" | "dec" | "inc" | "push" | "pop" => {
                // u8|u8 packed
                // will panic if not register here
                let t_operation = op.2.as_ref().unwrap();

                if t_operation.ttype == TokenType::Identifier {
                    let l_register = get_register(op.2.as_ref().unwrap());
                    bin_operations.push( (op.1.clone() as u8) | (l_register as u8) << 2 );

                } else if t_operation.ttype == TokenType::Value {
                    let l_value = get_value(op.2.as_ref().unwrap());
                    bin_operations.push( (op.1.clone() as u8) | (l_value as u8) );
                }
            },
            // "jmp"
            "jmpif" | "jmp" | "int" | "sf" => {
                let l_value = get_value(op.2.as_ref().unwrap());

                bin_operations.push(op.1.clone());
                bin_operations.push(l_value as u8);
                // jmpif 0x04
            },
            "clf" | "hlt" | "cli" | "crf" => {
                bin_operations.push(op.1.clone());
            },
            _ => todo!()
        }
    }

    //println!("{:?}", bin_operations);
    for op in &bin_operations {
        println!("{:08b}", op)
    }
    write_file(&mut bin_operations, output_path);
}

fn write_debug_file(debug_instructions: Vec<String>) {
    fs::write("instructions.d", debug_instructions.join("\n")).expect("Unable to write file");
}

fn write_file(raw_ops: &mut Vec<u8>, output_path: String) {
    fs::write(&output_path, raw_ops).expect("Unable to write file");
    println!("compile success, output: {}", output_path);
}
