fn main() {
    let mut v = vec![1, 2, 3];
    unsafe {
        *v.as_mut_ptr() = 10;
    }
    println!("Vector v: {:?}", v); // Vector is properly updated
    
    //Or
    let mut v2 = vec![1,2,3];
    v2[0] = 10; //Safer approach using vector indexing
    println!("Vector v2: {:?}",v2); 
} 