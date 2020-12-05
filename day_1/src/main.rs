use std::fs;

fn main() {

    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    
    let contents = contents.lines().map(|x| x.parse::<usize>().unwrap());

    let num_list: Vec<usize> = contents.clone().collect();
    for (i, n) in contents.enumerate(){
        if i != n {
            let contents_slice = (&num_list[i+1..]).iter();
            for (i2, x) in contents_slice.enumerate(){

                let contents_slice2 = (&num_list[i2+1..]).iter();

                for y in contents_slice2{
                    if n + x + y == 2020{
                        println!("{}", n * x * y);
                        return
                    }
                }
                
            }
        }
    } 
    
}