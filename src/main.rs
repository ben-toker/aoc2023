use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("message.txt");

    let mut file = match File::open(&path) {
            Err(e) => panic!("could not open: {}", e),
            Ok(file) => file,
    };
    
    let mut s = String::new();
    let _ = file.read_to_string(&mut s);

    let mut nums: Vec<u32> = Vec::new();

    for line in s.lines() {
        //println!("{}",line);
        let mut snum: String = "".to_string();

         
        let mut subnum: String = "".to_string();
        for ch in line.chars() {            
            subnum += &ch.to_string();

            if subnum.contains("one") {
                snum+= &"1".to_string();
                break;
            }
            else if subnum.contains("two") {
                snum+= &"2".to_string();
                break;
            }
            else if subnum.contains("three") {
                snum+= &"3".to_string();
                break;
            }
            else if subnum.contains("four") {
                snum+= &"4".to_string();
                break;
            }
            else if subnum.contains("five") {
                snum+= &"5".to_string();
                break;
            }
            else if subnum.contains("six") {
                snum+= &"6".to_string();
                break;
            }
            else if subnum.contains("seven") {
                snum+= &"7".to_string();
                break;
            }
            else if subnum.contains("eight") {
                snum+= &"8".to_string();
                break;
            }
            else if subnum.contains("nine") {
                snum += &"9".to_string();
                break;
            }

            if ch.is_numeric() {
                snum += &ch.to_string();
                break;
           }


        }
        

        let mut subnum: String = "".to_string();
        for ch in line.chars().rev() {
            subnum += &ch.to_string();

            if subnum.contains("eno") {
                snum+= &"1".to_string();
                break;
            }
            else if subnum.contains("owt") {
                snum+= &"2".to_string();
                break;
            }
            else if subnum.contains("eerht") {
                snum+= &"3".to_string();
                break;
            }
            else if subnum.contains("ruof") {
                snum+= &"4".to_string();
                break;
            }
            else if subnum.contains("evif") {
                snum+= &"5".to_string();
                break;
            }
            else if subnum.contains("xis") {
                snum+= &"6".to_string();
                break;
            }
            else if subnum.contains("neves") {
                snum+= &"7".to_string();
                break;
            }
            else if subnum.contains("thgie") {
                snum+= &"8".to_string();
                break;
            }
            else if subnum.contains("enin") {
                snum += &"9".to_string();
                break;
            }

        if ch.is_numeric() {
                snum += &ch.to_string();
                break;
           }


        }


        let num = match snum.parse::<u32>() {
            Err(e) => panic!("Error: {}", e),
            Ok(num) => num,
        };
        
        nums.push(num);

    }

    let sum: u32 = nums.iter().sum();
    println!("Total: {}",sum);
}
