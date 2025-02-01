#[derive(Clone)]
pub struct Log {
    mode: String,
}

impl Log {
    pub fn init(mode: String) -> Log {
        Log { mode }
    }

    pub fn debug(&self, msg: &str) {
        let accepted: Vec<String> = vec!["debug".to_string()];
        if accepted.contains(&self.mode) {
            self.out(&format!("D {}", msg));
        }
    }

    pub fn info(&self, msg: &str) {
        let accepted: Vec<String> = vec!["debug".to_string(), "info".to_string()];
        if accepted.contains(&self.mode) {
            self.out(&format!("I {}", msg));
        }
    }

    pub fn warn(&self, msg: &str) {
        let accepted: Vec<String> =
            vec!["debug".to_string(), "info".to_string(), "warn".to_string()];
        if accepted.contains(&self.mode) {
            self.out(&format!("W {}", msg));
        }
    }

    pub fn err(&self, msg: &str) {
        let accepted: Vec<String> = vec![
            "debug".to_string(),
            "info".to_string(),
            "warn".to_string(),
            "error".to_string(),
        ];
        if accepted.contains(&self.mode) {
            self.out(&format!("E {}", msg));
        }
    }

    pub fn fatal(&self, msg: &str) {
        let accepted: Vec<String> = vec![
            "debug".to_string(),
            "info".to_string(),
            "warn".to_string(),
            "error".to_string(),
        ];
        if accepted.contains(&self.mode) {
            self.out(&format!("FATAL {} \nProgram cannot continue!", msg));
        }
        std::process::exit(2);
    }

    fn out(&self, msg: &str) {
        println!("{}", msg);
    }
}
