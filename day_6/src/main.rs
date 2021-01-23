use std::{collections::HashMap, fs};
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");

    // println!("Tester: {:?}", contents.split("\n\n").count());
    
    let count: usize = contents.split("\n\n").map(|x| process_group(x)).sum() ;

    println!("{}", count);
}

fn count_letters(questions: &str) -> HashSet<char>{
    questions.chars()
        .fold( HashSet::new(), |mut set, question| if question != '\n' {set.insert(question); set} else {set})
}

fn process_group(questions: &str) -> usize {

    // let mut char_set: HashSet<char> = HashSet::new();
    // for group in questions.lines(){
    //     if char_set.is_empty() {
    //         char_set = count_letters(group);
    //     } else {
    //         char_set = char_set.union(&count_letters(group));
    //     }
    // }

    // let list = questions.lines().map(|line| count_letters(line));
    
    // let mut set : HashSet<char> = HashSet::new();

    // for i in list {
    //     if set.is_empty() {
    //         set = i;
    //     } else {
    //         let x = set.union(&i).collect::<Vec<char>>();
    //         println!("{:?}", x);
    //         set = HashSet::from_iter(x);
    //     }
    // }
    // println!("new entry");

    // println!("questions: {}", questions);

    let max =  questions.lines().count();

    let mut count_map = HashMap::new();

    for c in questions.chars().filter(|x| x != &'\n'){
        *count_map.entry(c).or_insert(0) += 1;
    }

    // println!("map: {:?}", count_map);

    let count = count_map.iter().filter(|k| *k.1 == max).count();
    
    // println!("count: {}", count);

    // println!();

    return count;
}