use std::fs;

fn main() {
    // let file_path = "characters_sample.txt";
    let file_path = "characters.txt";
    let contents = fs::read_to_string(file_path).expect("could not open file");
    let chars: Vec<char> = contents.chars().collect();
    let num_to_scan = 14;
    let mut marker = 0;
    'global: for i in 0..chars.len() {
       'local: for j in 1..num_to_scan {
            for k in j..num_to_scan {
                if chars[i+j-1] == chars[i+k] {
                    break 'local;
                }
            }
           if j == num_to_scan - 1 {
               marker = i + num_to_scan;
               break 'global;
           }
        }
    }

    println!("marker = {}", marker);
}
