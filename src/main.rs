/// Struct representing the interpreter state
/// 
/// # todo list
/// * implement it
pub struct Interpreter {
    /// Instruction pointer
    ip: usize,
    /// Data pointer
    dp: usize,
    /// Brainfuck machine memory
    mem: Box<[u8]>,
    /// Source code
    prog: Vec<u8>,
}

/// Minimal memory size according to [Wikipedia](https://en.wikipedia.org/wiki/Brainfuck)
const MEMORY_SIZE: usize = 30000;

impl Interpreter {
    /// Create new interpreter with empty program and default memory size.
    pub fn new() -> Interpreter {
        Interpreter {
            ip: 0,
            dp: 0,
            mem: Box::new([0u8; MEMORY_SIZE]),
            prog: Vec::new(),
        }
    }

    /// Read tape with program (where 'tape' is represented by string)
    /// Return number of valid brainfuck characters
    pub fn read_tape(&self, input: String) -> usize {
        for c in input.chars() {
            //println!("{}", c);
            match c {
                '+' | '-' | '>' | '<' | '[' | ']' | '.' | ',' => {
                    //println!("bf code");
                },
                _ => {
                    //println!("non bf");
                }
            }
        }
        0
    }

    /// Read next bf instruction
    fn read_next_instruction(&self) {
    }

    /// Run brainfuck program
    pub fn run(&self) {
    }
}

fn main() {
    println!("Hello, world!");
    let mut a = Interpreter::new();
    a.mem[0] = 1;
    assert!(a.mem[0] != 0);
    a.read_tape("Ahoj +++>++>+".to_string());
}

#[cfg(test)]
#[test]
#[should_panic]
fn test_sth() {
    panic!("Test");
}

#[cfg(test)]
#[test]
fn test_tape_reading() {
    let bfi = Interpreter::new();
    bfi.read_tape("abcd*()@+-[]<>.,".to_string());
    assert!(bfi.prog == b"+-[]<>.,");
}

#[cfg(test)]
#[test]
fn test_read_instruction() {
    let bfi = Interpreter::new();
    bfi.read_tape("+".to_string());
    bfi.read_next_instruction();
    assert!(bfi.mem[0] == 1); 

    let bfi = Interpreter::new();
    bfi.read_tape("++-".to_string());
    bfi.read_next_instruction();
    assert!(bfi.mem[0] == 1); 
    
    let bfi = Interpreter::new();
    bfi.read_tape(">+".to_string());
    bfi.read_next_instruction();
    assert!(bfi.mem[1] == 1); 

    let bfi = Interpreter::new();
    bfi.read_tape(">><+".to_string());
    bfi.read_next_instruction();
    assert!(bfi.mem[1] == 1); 

    let bfi = Interpreter::new();
    bfi.read_tape("[+++>>>]+++++".to_string());
    bfi.read_next_instruction();
    assert!(bfi.ip == 8); 

    let mut bfi = Interpreter::new();
    bfi.read_tape("++[+-><]+".to_string());
    bfi.ip = 7;
    bfi.read_next_instruction();
    assert!(bfi.ip == 3); 
}
