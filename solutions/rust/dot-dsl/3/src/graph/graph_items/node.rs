#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub id: String,
    pub attrs: Vec<(String, String)>,
}

impl Node {
    pub fn new(id: &str) -> Self {
        Self { 
            id: id.to_string(), 
            attrs: Vec::new() 
        }
    }
    
    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs = attrs.iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }

    pub fn attr(&self, name: &str) -> Option<&str> {
        self.attrs.iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }
}