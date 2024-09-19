#[test]
fn test_generate_account() {
    use crate::utilities::create_account;
    create_account();
}

#[test]
fn test_get_details() {
    use crate::utilities::get_details;
    get_details();
}

// #[test]
// fn test_delete_account() {
//     use crate::utilities::delete_account;
//     assert!(delete_account());
// }

#[test]
fn test_retrieve_messages() {
    use crate::utilities::retrieve_messages;
    let l = retrieve_messages().unwrap().len();
    assert_eq!(l, 0)
}
