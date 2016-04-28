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
    fn new() -> Interpreter {
        Interpreter {
            ip: 0,
            dp: 0,
            mem: Box::new([0u8; MEMORY_SIZE]),
            prog: Vec::new(),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut a = Interpreter::new();
    a.mem[0] = 1;
    assert!(a.mem[0] != 0);
}

#[cfg(test)]
mod test {
    #[test]
    #[should_panic]
    fn test_sth() {
        panic!("Test");
    }
}
