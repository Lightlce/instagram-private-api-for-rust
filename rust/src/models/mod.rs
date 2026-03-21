pub mod generated;
pub mod responses;
pub mod types;

pub fn manifest_counts() -> (usize, usize) {
    generated::manifest_counts()
}

#[cfg(test)]
mod tests {
    use super::manifest_counts;

    #[test]
    fn generated_manifest_is_not_empty() {
        let (responses, types) = manifest_counts();
        assert!(responses > 0);
        assert!(types > 0);
    }
}
