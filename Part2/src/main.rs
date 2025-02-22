use rand::Rng;
fn deleteALetter(input: String) -> String {
    let mut vec_u8:Vec<u8> = Vec::from(input);
    let mut vec_len: usize = vec_u8.len();
    let mut random_index: usize;
    let mut curr_char: u8;
    let mut output_string = "".to_string();
    for i in 1..=5 {
        random_index = rand::thread_rng().gen_range(0..vec_len);
        curr_char = vec_u8[random_index];
        while (curr_char <= 65 && curr_char >= 90) || (curr_char <= 97 && curr_char >= 122) {
            random_index = rand::thread_rng().gen_range(0..vec_len);
            curr_char = vec_u8[random_index];
        }
        vec_u8.remove(random_index);
        output_string = String::from_utf8(vec_u8.clone()).unwrap();
        println!("Iteration {}: {}",i,output_string);
        vec_len -= 1;
    }
    return output_string;
}

fn replaceALetter(input: String) -> String {
    let mut vec_u8:Vec<u8> = Vec::from(input);
    let vec_len: usize = vec_u8.len();
    let mut random_index: usize;
    let mut curr_char: u8;
    let mut rand_char_u8: u8;
    let mut output_string = "".to_string();
    for i in 1..=5 {
        random_index = rand::thread_rng().gen_range(0..vec_len);
        curr_char = vec_u8[random_index];
        rand_char_u8 = rand::thread_rng().gen_range(32..127);
        while (curr_char <= 65 && curr_char >= 90) || (curr_char <= 97 && curr_char >= 122) {
            random_index = rand::thread_rng().gen_range(0..vec_len);
            curr_char = vec_u8[random_index];
        }
        vec_u8[random_index] = rand_char_u8;
        output_string = String::from_utf8(vec_u8.clone()).unwrap();
        println!("Iteration {}: {}",i,output_string);
    }
    return output_string;
}

fn insertALetter (input: String) -> String {
    let mut vec_u8:Vec<u8> = Vec::from(input);
    let vec_len: usize = vec_u8.len();
    let mut random_index: usize;
    let mut curr_char: u8;
    let mut rand_char_u8: u8;
    let mut output_string = "".to_string();
    for i in 1..=5 {
        random_index = rand::thread_rng().gen_range(0..vec_len);
        curr_char = vec_u8[random_index];
        rand_char_u8 = rand::thread_rng().gen_range(32..127);
        while (rand_char_u8 <= 65 && rand_char_u8 >= 90) || (rand_char_u8 <= 97 && rand_char_u8 >= 122) {
            rand_char_u8 = rand::thread_rng().gen_range(32..127);
        }
        if vec_len-1 == random_index {
            vec_u8.push(rand_char_u8);
        } else {
            let mut left: Vec<u8> = vec_u8[..random_index].to_vec();
            let right: Vec<u8> = vec_u8[random_index..].to_vec();
            left.push(rand_char_u8);
            vec_u8 = vec![left,right].concat();
        }
        output_string = String::from_utf8(vec_u8.clone()).unwrap();
        println!("Iteration {}: {}",i,output_string);
    }
    return output_string;
}

#[cfg(test)]
mod tests {
    use super::*;
    static test_inputs: [&str; 4] = [
        "GET /index.html HTTP/1.1",
        "https://www.umkc.edu/current-students/",
        "https://en.wikipedia.org/wiki/American_Fuzzy_Lop_(software)",
        "http://www.google.com"
        ];
    #[test]
    fn test_deleteALetter(){
        for i in 0..test_inputs.len(){
            println!("\ndeleteALetter: {}",test_inputs[i]);
            deleteALetter(test_inputs[i].to_string());
        }
    }
    #[test]
    fn test_replaceALetter(){
        for i in 0..test_inputs.len(){
            println!("\nreplaceALetter: {}",test_inputs[i]);
            replaceALetter(test_inputs[i].to_string());
        }
    }
    #[test]
    fn test_insertALetter(){
        for i in 0..test_inputs.len(){
            println!("\ninsertALetter: {}",test_inputs[i]);
            insertALetter(test_inputs[i].to_string());
        }
    }
}

fn main() {
    deleteALetter("This is a long sentence.".to_string());
    replaceALetter("thereAreNoSpaces".to_string());
    insertALetter("Insert Here".to_string());
}