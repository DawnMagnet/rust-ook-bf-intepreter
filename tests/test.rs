#[cfg(test)]
mod tests {
    use rob::*;

    #[test]
    fn test_brainfuck_to_bytecode() {
        let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let expected = "Hello World!\n";
        let bytecode = brainfuck_to_bytecode(code);
        let mut interpreter = Intepreter::new(bytecode, 50000, 50000);
        let output = interpreter.run();
        assert_eq!(output, expected);
    }
}
