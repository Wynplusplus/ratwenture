use std::io;
use std::fs;


fn main() {

     let mut save = "test".to_string();
     let mut xp: u16 = 0;

     loop  {

    let story = String::from(save.clone()) + "/"  + "story.txt";
    let contents = read_file(story);
    if contents.trim() == "c".to_string() {
    break;
    }
    let mut iscd = false;
     if contents.trim() == "cd".to_string() || contents.trim() == "xp+".to_string() || contents.trim() == "xp-".to_string() || contents.trim() == "xp".to_string() {
    let xploc = String::from(save.clone()) + "/"  + "xp.txt";
     if contents.trim() == "xp" {
     let xploc = String::from(save.clone()) + "/"  + "xp.txt";
    
      if xp > read_file(xploc).parse().expect("an error occord"){
     iscd = true;
     }
     else{
    let tlxp = String::from(save.clone()) + "/"  + "tlxp.txt";
    println!("{}", read_file(tlxp));
    iscd = false;
    }
    }
    else {
     iscd = true;
    }
    if contents.trim() == "xp+".to_string() || contents.trim() == "xp-".to_string() {
    let xppm = read_file(xploc).parse::<u16>().expect("an error occord");
    if contents.trim() == "xp+".to_string() { 
    xp = xp + xppm
    } 
    else {
     xp = xp - xppm

     }
    }
    }
    else {
    iscd = false;
    }
    println!("{}", contents);

     if !iscd  {     
     let mut input = String::new();
     io::stdin().read_line(&mut input).expect("Failed to read line");
     input = input.trim().to_string();
     save = String::from(save) + "/" +  &input;
          
     if input == "c"{
     break;
     }
     }
   if iscd {
        let dir = String::from(save.clone()) + "/"  + "save.txt";
    let condir = read_file(dir);

    save = condir.trim().to_string();
   }

  println!();
}
 }
     

fn read_file(l: String) -> String {
   let fc = fs::read_to_string(l)
        .expect("Should have been able to read the file");

   return(fc.trim().to_string());
}
