use super::{column_parser, Column};

#[test]
fn test_fixed_and_displaced() {
    let i = "x:yz";
    let (remaining, got) = column_parser(i).expect("parser");
    let want = Column {
        fixed: Some('x'),
        displaced: Some(vec!['y', 'z']),
    };
    assert!(remaining.is_empty());
    assert_eq!(got.fixed, want.fixed);
    assert_eq!(got.displaced, want.displaced);
}

#[test]
fn test_fixed_only() {
    let i = "x:";
    let (remaining, got) = column_parser(i).expect("parser");
    let want = Column {
        fixed: Some('x'),
        displaced: None,
    };
    assert!(remaining.is_empty());
    assert_eq!(got.fixed, want.fixed);
    assert_eq!(got.displaced, want.displaced);
}

#[test]
fn test_displaced_only() {
    let i = ":yz";
    let (remaining, got) = column_parser(i).expect("parser");
    let want = Column {
        fixed: None,
        displaced: Some(vec!['y', 'z']),
    };
    assert!(remaining.is_empty());
    assert_eq!(got.fixed, want.fixed);
    assert_eq!(got.displaced, want.displaced);
}
