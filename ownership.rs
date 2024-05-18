fn main () {
  let _name = "Subho Yadav"; // immutable string literal
  let mut profession = String::from("SE"); // mutable string

  profession.push_str("-> Software Engineer");
  let job = profession; // profession is now no longer valid

  // println!("{}", profession);
  let living = job.clone();
  println!("{}", living);

  let random_string = String::from("Gide.ai");
  let (len, mut random_string_2) = get_length(random_string); // ownership of random_string is moved to get_length
  println!("The length of {} is {}", random_string_2, len);

  let size = get_size(&random_string_2);
  println!("The size of {} is {}", random_string_2, size);

  let word: String = String::from("hello world");
  let fw = first_word(&word);
}

fn get_length(some_string: String) -> (usize, String) {
  let length = some_string.len();
  (length, some_string) // returning a tuple
}

// get_size works same as get_length but with references
fn get_size(some_string: &String) -> usize {

  let length = some_string.len();
  length // returning a tuple
}

fn first_word (wrd: &String) -> &