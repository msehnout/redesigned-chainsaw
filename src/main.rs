/// Struct representing the interpreter state
/// 
/// # todo list
/// * implement it
pub struct Interpreter {
    /// Instruction pointer
    ip: u64,
    /// Data pointer
    dp: u64,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    #[should_panic]
    fn test_sth() {
        panic!("Test");
    }
}
