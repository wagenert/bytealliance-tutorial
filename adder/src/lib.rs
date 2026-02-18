mod bindings {
    wit_bindgen::generate!({
        path: "wit/world.wit",
    });

    use super::AdderComponent;
    export!(AdderComponent);
}

struct AdderComponent;

impl bindings::exports::docs::adder::add::Guest for AdderComponent {
    fn add(x: u32, y: u32) -> u32 {
        x + y
    }
}
