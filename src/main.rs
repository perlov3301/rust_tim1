fn main() {
    // println!("Hello, world1");
    println!("Hello, world1");
    let  mut x: u32 = 4;
    println!("muttable x  is {}", x); 
    { 
        println!("interium scope1");
        let x = 2;
        println!("let immut x is {}", x);
        println!("end of interium scope1");
    }
    {
        println!("interium scope2");
        let x = x - 2;
        println!("let immut sum of undeclared x and -2 is {}", x);
        println!("end of interium scope2");

        
    }
    {
        println!("interium scope3");
        println!("undeclared x is {}",x);
        x = x +10;
        println!(" undeclared sum x + 10 is {}",x);

    }
    println!("x after interium scopes is  {}", x);
    let x = x + 1;
    println!("let imm x = of x and 1 is {}", x);
    let x = "hello world";
    println!("let x immut is {}(new variable>new type", x);
}
