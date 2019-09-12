pub mod config {
    use std::env::{args, Args};
    #[derive(Debug)]
    pub struct Config {
        file_path: String,
        emit: bool,
        program: bool,
    }
    pub fn configure() -> Config {
        let cmd_args: Args = args();
        Config {
            file_path: String::from("/usairim/Documents/test.txt"),
            emit: true,
            program: true,
        }
    }
}
