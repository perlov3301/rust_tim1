fn main() {
    // println!("Hello, world1");
    println!("Hello, world1");
    let  x: u32 = 4;
    println!("x immut is {}", x); 
    { 
        println!("interium scope1");
        println!("x without definition is {}", x);
        let x = 2;
        println!("immut x is {}", x);
        println!("end of interium scope1");
    }
    {
        println!("interium scope2");
        println!("sum of undefined x and -2");
        let x = x - 2;
        println!("immut sum is {}", x);
        println!("end of interium scope2");

        
    }
    
    let x = x + 1;
    println!("imm sum of x and 1 is {}", x);
    let x = 8;
    println!("x immut is {}", x);
}
