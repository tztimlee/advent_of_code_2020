use std::fs;
use std::str::Lines;

fn main() {
    let contents = fs::read_to_string("input2")
        .expect("Something went wrong reading the file");

    let a = tree_count(contents.lines(), 1, 1);
    let b = tree_count(contents.lines(), 3, 1);
    let c = tree_count(contents.lines(), 5, 1);
    let d = tree_count(contents.lines(), 7, 1);
    let e = tree_count(contents.lines(), 1, 2);
    println!("a: {}, b: {}, c: {}, d: {}, e: {}", a,b,c,d,e);
    println!("new answer = {}", a*b*c*d*e);
    
    // let contents: Vec<Vec<char>> = contents.lines().map(|x| x.chars().collect()).collect();

    // println!("{}", tree_count(contents, 3, 1));

    
}

// fn tree_count(area: Vec<Vec<char>>, x_step: usize, y_step: usize) -> usize{
    
//     let mut column = 0;
//     let mut row = 0;
//     let mut trees = 0;

//     while row < area.len(){

//         if column >= area[row].len(){
//             column -= area[row].len();
//         }
        
//         if area[row][column] == '#'{
//             trees += 1;
//         }

//         row += x_step;
//         column += y_step;
        
//     }

//     return trees;
// }

fn tree_count(area: Lines, x_step: usize, y_step: usize) -> usize{

    let mut index = 0;
    let mut trees = 0;
    for (a,i) in area.enumerate(){

        if a % y_step == 0 {

            // println!("Row:{}, Index: {}, len: {}",a,  index, i.len());
            let square = i.chars().nth(index).unwrap();

            if square == '#' {
                trees +=1;
            }

            index += x_step;

            if index as usize >= i.len(){
                index = index - i.len();
            }

        }
    }

    return trees;

}