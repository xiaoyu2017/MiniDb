pub struct MiniDb {}

impl MiniDb {
    pub fn command_to_vec(com: &str) -> Vec<&str> {
        let mut v: Vec<&str> = Vec::new();
        for x in com.split_whitespace() {
            v.push(x);
        };
        v
    }
}