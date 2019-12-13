use std::fmt;
use std::ops;

/* uses trait, struct, display implemenation for struct, recursive fibonacci
 * uses new constructor
 */

//struct Fib has a trait implitation for Fibanocci
struct Fib{
    input: u32,
}

//trait Fibanocci calls get_value which is defined further below
trait Fibanocci {
    fn get_value(&self) -> u32;
}

//
impl Fib{

    fn new(x:u32) -> Fib {
        Fib{
            input: x,
        }
    }

}

impl ops::Add for Fib {
    type Output = Self;
    fn add(self, other:Self) -> Self {
        Self{
            input: (self.input + other.input),
        }
    }
}

impl fmt::Display for Fib {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The fibanocci of the {}th element is {}", self.input, self.get_value())
    }
}

impl Fibanocci for Fib {
    fn get_value(&self) -> u32 {
        fib(self.input)
    }
}

//function that performs fibanocci computation
fn fib(n: u32) ->  u32 {
   if n == 0 {
       1
   } else if n == 1{
       1
   } else {
       fib(n-1) + fib(n-2)
   }
}

fn main(){
    for i in 0..10{
        println!("fibanocci of {} is {}", i, fib(i));
    }
    let g = Fib {
        input : 7,
    };
    println!("\n\n{}\n", g);

    assert_eq!(Fib::new(9).get_value(), 55);
    let k = Fib::new(3);
    println!("{}", g + k);
}
