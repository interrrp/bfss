pub struct Header {
    pub address: String,
}

impl Header {
    pub fn from_brainfuck(contents: &str) -> Self {
        let mut header = Header {
            address: "".to_string(),
        };
        for line in contents.lines() {
            // [key=value]
            if !line.starts_with("[") || !line.contains("=") || !line.ends_with("]") {
                continue;
            }

            let mut parts = line.split("=");
            let key = parts.next();
            let value = parts.next();

            if !key.is_some() || !value.is_some() {
                continue;
            }

            let key = key.unwrap().replace("[", "").replace("]", "");
            let value = value.unwrap().replace("[", "").replace("]", "");

            if key == "address" {
                header.address = value.to_string();
            }
        }
        header
    }
}
