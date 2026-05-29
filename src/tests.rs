#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pigs() {
        let pigs = get_pigs();
        assert!(!pigs.is_empty());
        assert!(pigs.len() >= 20, "Should have at least 20 pigs");
    }

    #[test]
    fn test_pigs_have_valid_data() {
        let pigs = get_pigs();
        
        for (name, emoji, desc) in pigs {
            // 检查名字不为空
            assert!(!name.is_empty(), "Pig name should not be empty");
            
            // 检查emoji不为空
            assert!(!emoji.is_empty(), "Pig emoji should not be empty");
            
            // 检查描述不为空
            assert!(!desc.is_empty(), "Pig description should not be empty");
        }
    }

    #[test]
    fn test_pig_names_unique() {
        let pigs = get_pigs();
        let mut names: Vec<&str> = pigs.iter().map(|(name, _, _)| *name).collect();
        let original_len = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), original_len, "Pig names should be unique");
    }
}
