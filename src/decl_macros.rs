// MACRO CAPTURES
/*

block: a block expression, i.e. statements between curly braces
expr: an expression, a very wide variety of things within Rust
ident: an identifier or keyword. For example, the start of a function
declaration (fn hello) has a keyword followed by an identifier, and we
can capture them both by using ident twice
item: things like structs, enums, imports ('use declarations')…
lifetime: a Rust lifetime ('a)
literal: a literal, like a number or a character
meta: the content of an attribute, so Clone or rename = "true". You get
a good idea of what an attribute might contain in later chapters
pat: a pattern, 1 | 2 | 3 is one example
pat_param: similar to pat, except it can have | as a separator. So the
rule ($first:pat_param | $second:ident) will work, but
($first:pat | $second:ident) tells you that | is not allowed after
pat. This also means you need to do some extra work to parse 1 | 2 |
3 with pat_param (as it see 3 separate tokens instead of one)
path: a path, things like ::A::B::C, or `Self::method
stmt: a statement, for example an assignment (let foo = "bar")
tt: a TokenTree, see the above explanation
ty: a type, e.g. String
vis: a visibility modifier, pub comes to mind

*/

/* expr
    matches to a valid rust expression
    "hello".to_string(), vec![1, 2, 3], 1 + 2, 1
*/

/* stmt
    matches to a rust statement
    let x = 5, x.push(1), return Some(x)
*/

/* ident
    matches to a rust identifier
    variable name, function name, module name
*/

/* ty
    matches to a rust type
    i32, Vec<String>, Option<T>
*/

/* path
    matches to a rust path
    std::collections::HashMap
*/

// REPITITION SPECIFIER

// * - match zero or more repititions
// + - match one or more repititions
// ? - Match zero or one repetition

#[cfg(test)]

mod test {
    // macro_rules! mad_skills {
    //     // ($x:expr) => {
    //     //     format!("You sent an expression: {}", $x)
    //     // };
    //     ($x:ty) => {
    //         match stringify!($x) {
    //             "i32" => "You sent an i32 type".to_string(),
    //             _ => "You sent an something else".to_string(),
    //         }
    //     };
    // }

    macro_rules! my_vec {
        ($($x:expr),+) => {
           { let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )+
            temp_vec
        }
        };
    }
    // declaration of a declarative
    macro_rules! my_vec_a{
        // (matcher) ⇒ (transcriber)
    () => [Vec::new()];
    // (matcher) ⇒ (transcriber)
    (make an empty vec) => (Vec::new());
    // (matcher) ⇒ (transcriber)
    {$x:expr} => {{
    let mut v = Vec::new();
    v.push($x);
    v
    }};
    // (matcher) ⇒ (transcriber)
    [$($x:expr),+] => ({
    let mut v = Vec::new();
    $( v.push($x);)+
    v
    }
    )
    }

    #[test]
    fn test_declarative_macros() {
        let mut x: Vec<i32> = vec![1];
        let mut y: Vec<i32> = my_vec!(4);
        dbg!(y);
        // let some_var: String = mad_skills!(i32);
        // dbg!(some_var);
        let empty: Vec<i32> = my_vec_a![];
        dbg!(empty);
        let also_empty: Vec<i32> = my_vec_a!(make an empty vec);
        dbg!(also_empty);
        let three_numbers = my_vec_a!(1, 2, 3);
        dbg!(three_numbers);
    }
}
