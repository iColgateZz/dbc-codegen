pub mod rust;

trait Generator {
    fn buf_mut(&mut self) -> &mut String;

    fn prepend_line(&mut self, text: &str) {
        let buf = self.buf_mut();
        buf.insert(0, '\n');
        buf.insert_str(0, text);
    }
    
    fn write_indent(&mut self, indent: usize) {
        let buf = self.buf_mut();
        for _ in 0..indent {
            buf.push(' ');
        }
    }

    fn write_line(&mut self, indent: usize, text: &str) {
        self.write_indent(indent);
        let buf = self.buf_mut();
        buf.push_str(text);
        buf.push('\n');
    }

    fn write_newline(&mut self) {
        let buf = self.buf_mut();
        buf.push('\n');
    }
}
