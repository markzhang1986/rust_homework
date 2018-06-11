#![allow(unused_parens)]

fn main() {
    let x: u16 = 10u16;
    //let y = if x > 0 { "greater" } else { "less" };
    let y = 20; /* Last binding will be discard */

    println!("x {}, y {}\n", x, y);


    /* Array and Vector */
    let array = [0u32; 5];
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(233);
    let v2: Vec<i32> = vec![1, 2, 233];


    print!("Array and Loop test\n");
    for ele in &v1 {
        println!("{}", ele);
    }

    println!("{:?}", array);
    
    /* If is an expression statement */
    println!("v1 = {} and v2 is {} {}", v1[1], v2[2], 
             if (x > 0) {
                 10
             } else {
                 20
             }
            );

    /* Loop */
    let mut i: i32 = 0;
    while (i < 3) {
        i += 1;
        println!("{}", i);


    }


}
