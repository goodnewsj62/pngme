mod chunk_type;
mod chunk;
mod png;


fn main() {
    let c =  [1,2,3,4];
    

    println!("{:?} {:?}",  c.iter().cloned().collect::<Vec<u8>>(),  c)
}


