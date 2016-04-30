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
    pub fn read_tape(&mut self, input: String) -> usize {
        let mut count = 0;
        for c in input.chars() {
            match c {
                '+' | '-' | '>' | '<' | '[' | ']' | '.' | ',' => {
                    self.prog.push(c as u8);
                    count += 1;
                },
                _ => {}
            }
        }
        count
    }

    /// Read next bf instruction
    fn read_next_instruction(&mut self) {
        match self.prog[self.ip] {
            b'+' => {
                self.mem[self.dp] += 1;
                self.ip += 1;
            },
            b'-' => {
                self.mem[self.dp] -= 1;
                self.ip += 1;
            },
            b'<' => {
                self.dp -= 1;
                self.ip += 1;
            },
            b'>' => {
                self.dp += 1;
                self.ip += 1;
            },
            b'[' => {
                if self.mem[self.dp] != 0 {
                    self.ip += 1;
                } else {
                    while self.prog[self.ip] != b']' {
                        self.ip += 1;
                    }
                    self.ip += 1;
                }
            },
            b']' => {
                if self.mem[self.dp] != 0 {
                    while self.prog[self.ip] != b'[' {
                        self.ip -= 1;
                    }
                }
                self.ip += 1;
            },
            b'.' => {
                print!("{}", self.mem[self.dp] as char);
                self.ip += 1;
            },
            _ => {}
        }
    }

    /// Run brainfuck program
    pub fn run(&mut self) {
        self.ip = 0;
        self.dp = 0;
        while self.ip < self.prog.len() {
            self.read_next_instruction();
        }
    }
}

fn main() {
    let mut a = Interpreter::new();
    let tape = "1 ++++++++++\
      2 [\
      3  >+++++++\
      4  >++++++++++\
      5  >+++\
      6  >+\
      7  <<<<-\
      8 ] inicializační cyklus nastaví potřebné hodnoty buněk\
      9 >++. výpis 'H'\
     10 >+. výpis 'e'\
     11 +++++++. 'l'\
     12 . 'l'\
     13 +++. 'o'\
     14 >++. mezera\
     15 <<+++++++++++++++. 'W'\
     16 >. 'o'\
     17 +++. 'r'\
     18 ------. 'l'\
     19 --------. 'd'\
     20 >+. '!'\
     21 >. nová řádka";
    a.read_tape(tape.to_string());
    a.run();
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
    let mut bfi = Interpreter::new();
    let ret = bfi.read_tape("abcd*()@+-[]<>.,".to_string());
    assert!(bfi.prog == b"+-[]<>.,");
    assert!(ret == 8);
}

#[cfg(test)]
#[test]
fn test_read_instruction() {
    let mut bfi = Interpreter::new();
    bfi.read_tape("+".to_string());
    bfi.read_next_instruction();
    assert!(bfi.mem[0] == 1); 

    let mut bfi = Interpreter::new();
    let prog = "++-".to_string();
    let len = prog.len();
    bfi.read_tape(prog);
    for _ in 0..len {
        bfi.read_next_instruction();
    }
    assert!(bfi.mem[0] == 1); 
    
    let mut bfi = Interpreter::new();
    let prog = ">+".to_string();
    let len = prog.len();
    bfi.read_tape(prog);
    for _ in 0..len {
        bfi.read_next_instruction();
    }
    assert!(bfi.mem[1] == 1); 

    let mut bfi = Interpreter::new();
    let prog = ">><+".to_string();
    let len = prog.len();
    bfi.read_tape(prog);
    for _ in 0..len {
        bfi.read_next_instruction();
    }
    assert!(bfi.mem[1] == 1); 

    let mut bfi = Interpreter::new();
    bfi.read_tape("[+++>>>]+++++".to_string());
    bfi.read_next_instruction();
    assert!(bfi.ip == 8); 

    let mut bfi = Interpreter::new();
    bfi.read_tape("++[+-><]+".to_string());
    bfi.mem[0] = 2;
    bfi.ip = 7;
    bfi.read_next_instruction();
    assert!(bfi.ip == 3); 
}
