fn main() {
    let numbers = (1,2,3,4,5,6,7);
    
    match numbers
    {
        (first, .. , last)  => {
            println!("first = {}, last = {}", first, last);
        }
    }
    /* err
     match numbers
     {
         (.., first, ..)  => {
             println!("first = {}", first);
         }
     }
     */
    println!("Hello, world!");
}
