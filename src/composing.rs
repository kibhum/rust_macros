fn add_one(n: i32) -> i32 {
    n + 1
}
fn stringify(n: i32) -> String {
    n.to_string()
}
fn prefix_with(prefix: &str) -> impl Fn(String) -> String + '_ {
    move |x| format!("{}{}", prefix, x)
}

fn compose_two<FIRST, SECOND, THIRD, F, G>(f: F, g: G) -> impl Fn(FIRST) -> THIRD
where
    F: Fn(FIRST) -> SECOND,
    G: Fn(SECOND) -> THIRD,
{
    move |x| g(f(x))
}

pub fn compose() {
    let two_composed_function =
        compose_two(compose_two(add_one, stringify), prefix_with("Result: "));
}
