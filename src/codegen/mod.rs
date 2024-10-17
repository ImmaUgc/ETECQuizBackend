mod powershell;

pub trait Generator {
    fn write_line(&mut self, line: String);
}