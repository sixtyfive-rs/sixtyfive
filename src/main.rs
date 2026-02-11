// Copyright (C) 2026 Ted "pheenty" Lukin <fedorlukin2006@gmail.com>. See LICENCE for details.
use std::{env, fs, io::{self, Read}};
enum Instruction { Next, Prev, Add, Sub, Out, In, While, Done }
fn main() {
    let mut pc = 0usize;
    let mut tape = [0u8; 30000];
    let mut ptr = 0usize;
    let program: Vec<Instruction> = fs::read_to_string(
        env::args()
        .nth(1)
        .expect("Usage: sixtyfive program.65")
)       .expect("This file does not exist.")
        .split_ascii_whitespace()
        .filter_map(|i| match i {
            "65" => Some(Instruction::Next),
            "6565" => Some(Instruction::Prev),
            "656565" => Some(Instruction::Add),
            "65656565" => Some(Instruction::Sub),
            "6565656565" => Some(Instruction::Out),
            "656565656565" => Some(Instruction::In),
            "65656565656565" => Some(Instruction::While),
            "6565656565656565" => Some(Instruction::Done),
            _ => None,
})      .collect();
    while pc < program.len() {
        match program[pc] {
            Instruction::Next => {
                ptr = ptr.wrapping_add(1) % 30000;
                pc += 1
},          Instruction::Prev => {
                ptr = ptr.wrapping_sub(1) % 30000;
                pc += 1
},          Instruction::Add => {
                tape[ptr] = tape[ptr].wrapping_add(1);
                pc += 1
},          Instruction::Sub => {
                tape[ptr] = tape[ptr].wrapping_sub(1);
                pc += 1
},          Instruction::Out => {
                print!("{}", tape[ptr] as char);
                pc += 1
},          Instruction::In => {
                let mut buf = [0u8];
                io::stdin().read_exact(&mut buf).unwrap();
                tape[ptr] = buf[0];
                pc += 1
},          Instruction::While => {
                if tape[ptr] == 0 {
                    let mut whiles = 1usize;
                    while whiles != 0 {
                        pc += 1; // yes this сould crash idc
                        match program[pc] {
                            Instruction::While => whiles += 1,
                            Instruction::Done => whiles -= 1,
                            _ => {
}}}}},      Instruction::Done => {
                if tape[ptr] != 0 {
                    let mut dones = 1usize;
                    while dones != 0 {
                        pc -= 1;
                        match program[pc] {
                            Instruction::Done => dones += 1,
                            Instruction::While => dones -= 1,
                            _ => {
}}}}}}}} // 65 lines
