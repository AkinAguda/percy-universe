pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    /// TODO
    /// - Figure out a way to create a "provider" that can magically "provide" the universe to all components
    /// - Figure out how to subscribe a component to the state while they access data from the consumer
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
