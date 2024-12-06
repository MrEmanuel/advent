use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> io::Result<()> {

    let mut products:Vec<i32> = vec![];
    // Open the file
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    // Read the file into a buffer
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    // Iterate over the buffer
    let mut iter = buffer.iter().enumerate();
    let mut enable = true;
    while let Some((i, &byte)) = iter.next() {
        if byte == b'd' {
            // Match against "do()"
            let temp_do_chars = String::from_utf8_lossy(&buffer[i..i + 4]);
            let do_chars = temp_do_chars.chars().as_str();
            println!("do_chars: {:?}", do_chars);
            if do_chars == "do()" {
                println!("Found 'do()' at {i}");
                enable = true;
            } else {
                // Match against "don't()"
            let temp_dont_chars = String::from_utf8_lossy(&buffer[i..i + 7]);
            let dont_chars = temp_dont_chars.chars().as_str();
            println!("dont_chars: {:?}", dont_chars);
            if dont_chars == "don't()" {
                println!("Found 'don't()' at {i}");
                enable = false;
            }

                
            }

        }

        if enable == false {
            println!("Skipping index {i}");
            continue;
        }
        println!("checking index {i}");


        if byte == b'm' {
            // println!("Found m at pos: {i}");
            // Check the next 3 characters if available
            
            if i + 3 < buffer.len() { // TODO: Maybe need to make this +3 longer..
                let temp_next_chars = String::from_utf8_lossy(&buffer[i + 1..i + 4]);
                let next_chars = temp_next_chars.chars().as_str();
                // println!("next 3 chars: {next_chars}");
                //  1234567901011
                //  ul(abc,abc)
                if next_chars == "ul("{
                    // Check buffer length. 
                    let mut max_length = 12;
                    if i + max_length > buffer.len() {
                        max_length = buffer.len() - i;
                        if max_length < 4 {
                            break
                        }
                    }

                    // Iterate over i+4 to i+12 to find the "," and ")"
                    let temp_arg_chars = String::from_utf8_lossy(&buffer[i + 4..i + max_length]);
                    let arg_chars = temp_arg_chars.chars().as_str();

                    println!("Arg chars: {:?}", arg_chars);
                    let mut comma_index = None;
                    let mut parenthesis_index = None;

                    for (index, val) in arg_chars.char_indices() {
                        let is_comma = val == ',';
                        let is_parenthesis = val == ')';
                        if is_comma && comma_index.is_none() {
                            println!("Found comma at {index}");
                            comma_index = Some(index);
                            continue;
                        }
                        if is_parenthesis && parenthesis_index.is_none() {
                            println!("Found parenthesis at {index}");
                            parenthesis_index = Some(index);
                            continue;
                        }

                    }

                    if comma_index.is_some() && parenthesis_index.is_some() {
                        let c_index = comma_index.unwrap();
                        let p_index = parenthesis_index.unwrap();
                        if c_index > p_index {
                            continue
                        }

                        let val1 = &arg_chars[..c_index].parse::<i32>().unwrap_or_default();
                        let val2 = &arg_chars[c_index + 1..p_index].parse::<i32>().unwrap();
                        let product = val1*val2;
                        println!("Pushing product: {product} from {val1}x{val2}");
                        products.push(product);

                    

                    }

                 }
                    
                
                
            } else {
                println!("Found 'm' at position {}, but not enough chars after it", i);
            }}
        }
    
    println!("Found products: {:?}", products);
    let sum = products.iter_mut().reduce(|acc, e| {
        *acc += *e;
        acc
    });
    println!("Final sum is: {}", sum.unwrap());

    Ok(())
}
