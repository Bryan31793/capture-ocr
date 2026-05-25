

pub struct SocketConfig<'a> {
    path: &'a str,
    program: &'a str,
    process_path: &'a str,
}   

impl<'a> Default for SocketConfig<'a> {
    fn default() -> Self {
        Self {
            path: "/tmp/my_socket_bryan",
            program: "python3",
            process_path: "/home/bryan/capture-ocr/ocr/main.py",
        }
    }
}

impl<'a> SocketConfig<'a> {
    pub fn path(&self) -> &str {
        self.path
    }

    pub fn program(&self) -> &str {
        self.program
    }

    pub fn process_path(&self) -> &str {
        self.process_path
    }
}