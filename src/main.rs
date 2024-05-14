use std::{error::Error, io::{self, BufRead}};
use regex::Regex;

static REGEXES: &str=r#"(?:"|'|=)(((?:[a-zA-Z]{1,10}://|//)[^"'/]{1,}\.[a-zA-Z]{2,}[^"']{0,})|((?:/|\.\./|\./)[^"'><,;| *()(%%$^/\\\[\]][^"'><,;|()]{1,})|([a-zA-Z0-9_\-/]{1,}/[a-zA-Z0-9_\-/.]{1,}\.(?:[a-zA-Z]{1,4}|action)(?:[\?|#][^"|']{0,}|))|([a-zA-Z0-9_\-/]{1,}/[a-zA-Z0-9_\-/]{3,}(?:[\?|#][^"|']{0,}|))|([a-zA-Z0-9_\-]{1,}\.(?:php|asp|aspx|jsp|json|action|html|js|txt|xml)(?:[\?|#][^"|']{0,}|)))(?:"|'| )"#;
// Linkfinder regexes

fn get_links(body: &String, printed: &mut Vec<String>) {

    let re = Regex::new(&REGEXES).unwrap();
    let result = re.captures_iter(&body);

    for m in result{
        
        let lin = m.get(1).map_or("", |v| v.as_str()).trim().to_string();

        if !printed.contains(&lin){
            println!("{}", lin);
            printed.push(lin);
        }

    }
}


fn main() -> Result<(), Box<dyn Error>>{
    let mut stdin = io::stdin().lock();
    let mut buffer = String::new();
    let mut count = 0;
    let mut body = String::new();
    let mut printed: Vec<String> = vec![];

    // read stdin
    loop {

        let red_byte = stdin.read_line(&mut buffer)?;

        if red_byte == 0 {
            get_links(&body, &mut printed);
            break;
        }

        count +=1;
        body += &buffer;

        if count == 20000 {
            get_links(&body, &mut printed);
            count = 1;
            body.clear()
        }

        buffer.clear();
    }

    Ok(())
}
