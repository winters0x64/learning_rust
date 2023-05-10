fn main(){

    // Stack Allocation

   /*  Data allocated on the stack can be copied
    This process is also called "Move", Data types
    with a fixed size are allocated on the stack
    They have "copy trait" that is they  can be used even after
    assigning themt o another variable.
    
    let x = 5;
    let y = x;

    println!("x is {}, y is {}",x,y); */ 


    // Heap Allocation

    /* Now the data that is allocated on the heap 
    that is the data which has a dynamic size
    such as a user input string, we can't determine
    the size of the data hence we use Heap allocation
    Heap allocation is costly and time consuming so 
    rust cleans all the data allocated on the heap(aka drop method) once it goes
    out of scop or the ownership of the data is transfered to another
    variable or function. 

    If we compile we'll get the following error
    error: could not compile `ownership` due to previous error
    */

    /* let x = String::from("Hello");
    let y = x;
    println!("x is {}, y is {}",x,y); */

    let a = "Hello";

    first_word(a);

}

fn first_word(s: String) {
    let bytes = s.as_bytes();

    println!("{}",bytes);
}