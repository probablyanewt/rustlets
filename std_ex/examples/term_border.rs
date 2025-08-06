use std_ex::term::{Border, Printer};

const HASH_PRINTER: Printer = Printer::new().set_border(Border::Hash).done();
const LINE_PRINTER: Printer = Printer::new().set_border(Border::Line).done();
const DOUBLE_LINE_PRINTER: Printer = Printer::new().set_border(Border::DoubleLine).done();

fn main() {
    HASH_PRINTER.println("Hash test");
    LINE_PRINTER.println("Line test");
    DOUBLE_LINE_PRINTER.println("Double line test");
    LINE_PRINTER.println("What happens with a really really really really really really really really really really really really really really really really really really really really really really really really really really really really really really really really really really really really really really really long string test");
}
