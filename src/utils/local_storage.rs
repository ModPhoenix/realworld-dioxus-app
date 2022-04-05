pub fn set_item(key: &str, value: &str) {
    let window = web_sys::window().expect("window");

    window
        .local_storage()
        .unwrap()
        .unwrap()
        .set_item(key, value)
        .unwrap();
}

pub fn get_item(key: &str) -> Option<String> {
    let window = web_sys::window().expect("window");

    window
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item(key)
        .unwrap()
}
