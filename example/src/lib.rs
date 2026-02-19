mod bindings {
    use super::Component;
    wit_bindgen::generate!({ generate_all });
    export!(Component);
}

struct Component;

impl bindings::exports::wasi::cli::run::Guest for Component {
    #[doc = "/ Run the program."]
    #[allow(async_fn_in_trait)]
    fn run() -> Result<(), ()> {
        bindings::docs::calculator::calculate::eval_expression("1 + 2");
        Ok(())
    }
}
