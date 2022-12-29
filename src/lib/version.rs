pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32
}

impl Version {
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}