#[derive(Clone, Debug)]
pub enum SizeType {
    B,
    KB,
    MB,
    GB,
}

impl SizeType {
    pub fn values() -> Vec<SizeType> {
        vec![SizeType::B, SizeType::KB, SizeType::MB, SizeType::GB]
    }

    pub fn name(&self) -> String {
        match self {
            SizeType::B => "B",
            SizeType::KB => "KB",
            SizeType::MB => "MB",
            SizeType::GB => "GB",
        }.to_string()
    }

    pub fn value(&self, size_in_bytes: &u64) -> u64 {
        let size = *size_in_bytes;
        match self {
            SizeType::B => size,
            SizeType::KB => size * 1_000,
            SizeType::MB => size * 1_000_000,
            SizeType::GB => size * 1_000_000_000,
        }
    }
}