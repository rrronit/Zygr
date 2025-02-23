use zygr::tokenizer;





fn main() {
    let args=std::env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        println!("Usage: {} (-t || -p) <filename>", args[0]);
        return;
    }

    let method = &args[1];
    let filename = &args[2];

    match method.as_str() {
        "-t" => {
            let text = std::fs::read_to_string(filename).unwrap();
            let mut tokenizer = tokenizer::Tokenizer::new(text);
            let tokens = tokenizer.tokenize();
            for token in tokens {
                println!("{:?}", token);
            }
        }
        "-p" => {
            let text = std::fs::read_to_string(filename).unwrap();
            let mut tokenizer = tokenizer::Tokenizer::new(text);
            let tokens = tokenizer.tokenize();
            let mut parser = zygr::parser::Parser::new(tokens);
            let tokens = parser.parse();
            for token in tokens {
                println!("{:?}", token);
            }
          
        }
        _ => {
            println!("Usage: {} (-t || -p) <filename>", args[0]);
        }
        
    }


    
}
