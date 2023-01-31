pub struct HostsEntry {
    pub ip: String,
    pub names: Vec<String>,
}

impl HostsEntry {
    pub fn new(ip: &str, first_name: &str) -> Self {
        HostsEntry {
            ip: ip.to_string(),
            names: vec![first_name.to_string()],
        }
    }

    pub fn from_string(str: String) -> Self {
        let mut he = HostsEntry {
            ip: String::new(),
            names: vec![],
        };

        for (i, token) in str.split(' ').enumerate() {
            if i == 0 {
                he.ip = token.to_string();
            } else {
                he.names.push(token.to_string());
            }
        }
        he
    }
}
