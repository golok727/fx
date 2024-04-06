pub fn init_core(init: i32) -> i32 {
    println!("Initializing Core");
    init
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_core() {
        let stat = init_core(727);
        assert_eq!(stat, 727)
    }
}
