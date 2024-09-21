use std::fs::File;
use std::io;
use std::fs;


fn main() {

     let mut save = "test".to_string();


     while true {

    let story = String::from(save.clone()) + "/"  + "story.txt";
    let contents = fs::read_to_string(story)
        .expect("Should have been able to read the file");
    if contents.trim() == "p".to_string() {
    panic!();
    }
    let mut iscd = 0;
     if contents.trim() == "cd".to_string() {
    iscd = 1;
    }
    else {
    iscd = 0;
    }
    println!("{}", contents);

     if iscd == 0 {     
     let mut input = String::new();
     io::stdin().read_line(&mut input).expect("Failed to read line");
     input = input.trim().to_string();
     save = String::from(save) + "/" +  &input;
          
     if input == "p"{
     panic!();
     }
     }
   if iscd == 1 {
        let dir = String::from(save.clone()) + "/"  + "save.txt";
    let condir = fs::read_to_string(dir)
        .expect("Should have been able to read the file");

    save = condir.trim().to_string();
    
   println!("test");
   }

  println!();
}
 }
     

