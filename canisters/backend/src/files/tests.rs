#[cfg(test)]
mod tests {

    #[test]
    fn test_queries() {
        println!(">>>>>>>>>> test_queries <<<<<<<<<");

        let mut backend = backend.unwrap();
        let response = backend.get_element_trees();
        assert_ne!(response, Err("No user found"));

        let user = make_dummy_user();
        let response = internet_idenity.login(user);
        let response = backend.register();
        let response = backend.get_element_trees();
        assert_ne!(response, Err("User has no elements"));

    }
}
