
#[macro_export]
macro_rules! log {

    ($input: expr) => {
        {

        fn type_of<T>(_: &T) -> String {
            return format!("{}", std::any::type_name::<T>());
        }
        use web_sys::console::{log_1};
        let dir =  std::file!();
        let line = std::line!();
        let goal = $input;
        log_1(&format!(r#"
            {:#?},line:{:#?}
            type:{:#?}
            {:#?}"#,
            dir,line,
            type_of(&goal),&goal).into());
        }
    }
}
