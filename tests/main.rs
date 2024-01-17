#[test]
fn expand_tilde_test() {
    let expected = simple_home_dir::home_dir().unwrap().join("foo");
    let resulted = simple_expand_tilde::expand_tilde("~/foo").unwrap();
    assert_eq!(resulted, expected)
}
