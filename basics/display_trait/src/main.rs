use std::fmt;

struct Object{
    x:u32,
}

impl Object{
    fn new(y:u32) -> Object{
        Object{
            x:y,
        }
    }
}

impl fmt::Display for Object{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "value of x is {}", self.x)
    }
}

fn main(){
    let a = Object::new(2);
    println!("{}", a);
}
