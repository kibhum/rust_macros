#[cfg(test)]
mod tests {

    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_box_smart_pointers() {
        let x: Box<i32> = Box::new(50);
        #[derive(Debug)]
        struct Node {
            id: u32,
            next: Option<Box<Node>>,
        };

        let nodes: Box<Node> = Box::new(Node {
            id: 0,
            next: Some(Box::new(Node {
                id: 1,
                next: Some(Box::new(Node { id: 2, next: None })),
            })),
        });
        dbg!(nodes);
    }
    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_reference_counter() {}
}
