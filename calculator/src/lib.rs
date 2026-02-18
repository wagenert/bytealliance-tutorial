mod bindings {
    use super::Component;
    wit_bindgen::generate!({
            world: "calculator",
            with: {"docs:adder/add@0.1.0": generate},
    });
    export!(Component);
}

struct Component;

impl bindings::exports::docs::calculator::calculate::Guest for Component {
    fn eval_expression(expr: String) -> u32 {
        let mut parts = expr.split_whitespace();
        let first = parts.next().unwrap().parse::<u32>().unwrap();
        let _op = parts.next().unwrap();
        let second = parts.next().unwrap().parse::<u32>().unwrap();
        bindings::docs::adder::add::add(first, second)
    }
}
