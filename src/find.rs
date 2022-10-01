use std::io;
use std::io::Read;

pub fn run() {
    
println!("what word do you want to search for");
let mut userinput = String::new();
io::stdin().read_line( &mut userinput).expect("failed input");
let userinput:String = userinput.trim().parse().expect("failed parse");

let mut number2: String = String::new();

println!("Type the replaced char" );
let mut new_word: String = String::new();
io::stdin().read_line( &mut new_word).expect("failed input");
let new_word :String = new_word.to_owned().trim().parse().expect("failed parse");
let s_slice: &str = &new_word[..];

let mut file = std::fs::File::open("newstuff.txt").unwrap();
let  mut content = String::new();
file.read_to_string(&mut content).unwrap();

number2 = replace(content,userinput, s_slice );

println!("{}",number2);

}

pub fn replace(content: String, find: String, change: &str) -> String{
   let mut container: String = String::new();
    for i in content.split(&find){
    container.push_str(i);
    container.push_str(change);

        }
        for _w in 0..change.len(){
            container.pop();
}
container
    } 
   
   
