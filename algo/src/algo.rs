use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Algo {
    pub r#type: AlgoType,
}

#[derive(Debug, Clone, Copy)]
pub enum AlgoType {
    Blake3,
    Default,
}

impl Algo {
    pub fn new(r#type: AlgoType) -> Self {
        Self { r#type }
    }

    pub fn hash(&self, v: String) -> String {
        match self.r#type {
            AlgoType::Blake3 => {
                let hash = blake3::hash(v.as_bytes());
                hash.to_string()
            }
            AlgoType::Default => {
                let mut hasher = DefaultHasher::new();
                v.hash(&mut hasher);
                hasher.finish().to_string()
            }
        }
    }

    pub fn get_name(&self) -> &str {
        match self.r#type {
            AlgoType::Blake3 => "blake3",
            AlgoType::Default => "default",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_hash() {
        let algo = Algo::new(AlgoType::Default);
        let hash = algo.hash("hello".to_string());
        assert_eq!(hash, "16156531084128653017");
    }

    #[test]
    fn test_blake3_hash() {
        let algo = Algo::new(AlgoType::Blake3);
        let hash = algo.hash("hello".to_string());

        assert_eq!(
            hash,
            "ea8f163db38682925e4491c5e58d4bb3506ef8c14eb78a86e908c5624a67200f"
        );
    }
}
