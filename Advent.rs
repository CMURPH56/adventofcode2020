use std::fs;

fn main() {
  let filename = "inputdayone.txt";
  println!("In File {}", filename);

  let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

  println!("With text:\n{}", contents);


  // Statements here are executed when the compiled binary is called
  let split = contents.lines();
  let mut num = Vec::new();
  for s in split {
    let my_int: i32 = s.trim().parse().unwrap();
    num.push(my_int)
  }
  let input_one = num.clone();
  let input_two = num.clone();
  day_one_part_one(input_one);
  day_one_part_two(input_two);
}


fn day_one_part_one(num: Vec<i32>){
  let num_len = num.len();
  for x in 1..num_len {
    for y in x..num_len { 
      if num[x] + num[y]  == 2020 {
        let answer = num[x] * num[y];
        println!("Day One Part One Answer: {}",answer);
      }
    }
  }
}

fn day_one_part_two(num: Vec<i32>){
  let num_len = num.len();
  for x in 1..num_len {
    for y in x..num_len { 
      for z in y..num_len {
        if num[z] + num[x] + num[y]  == 2020 {
          let answer = num[x] * num[y] * num[z];
          println!("Day One Part Two Answer: {}",answer);
        } 
      }
    }
  }
}
