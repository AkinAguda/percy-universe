pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    /// TODO
    /// - Figure out a way to create a "provider" that can magically "provide" the universe to all components
    /// - Figure out how to subscribe a component to the state while they access data from the consumer
    /// - Need to investigate component re-rendering (basically telling the percy: "Hey, this component and all its children need to be re-rendered")
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
