fn is_unique_naive(s:String) -> bool {
    let mut is_unique = true;
    for (i,a) in s.chars().enumerate() {
        for b in s.chars().skip(i+1) {
            if a == b {
                is_unique = false;
                break;
            }
        }
    }
    is_unique
}

#[test]
fn test_is_unique_naive() {
    assert_eq!(true,is_unique_naive(String::from("abcdef")));
    assert_eq!(false, is_unique_naive(String::from("abab")));
    assert_eq!(true, is_unique_naive(String::from("Aa")));
}