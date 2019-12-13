fn factorial(n:i32) -> i32 {
    if n > 1 {
        n * factorial(n -1)
    } else {
        1
    }
}

fn main(){
    assert_eq!(720, factorial(6));
}
