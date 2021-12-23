use regex::Regex;
use std::collections::VecDeque;


struct DeskCalc {
  code: String,
  stack: VecDeque<String>,
}

impl DeskCalc {
  fn run(&mut self) {
    let num_regex = r#"(?P<num>\d*.?\d+|\d+.?\d*)"#;
    let comment_regex = r#"(?P<comment>#.*)"#;
    // let string_regex = r#"\[(?P<str>.*)\]"#;
    let space_regex = r#"\s+"#;
    let token_regex = Regex::new(
      &vec![
        num_regex,
        // string_regex,
        comment_regex,
        space_regex,
      ]
      .join("|"),
    )
    .unwrap();
    let mut stack_temp: Vec<String> = vec![];

    let mut string_bracket: isize = 0;
    let mut string_parse_curr = false;

    while self.code.len() != 0 {
      match token_regex.find(&self.code) {
        // regular case
        Some(mat) => {
          self.code = (&self.code[mat.end()..]).to_string();
        }

        // edge case for string and commands
        None => {
          let next_char = self.code.chars().next().unwrap();

          match next_char {
            '[' => {
              string_parse_curr = true;
              string_bracket += 1;
            }

            ch if ch == ']' && string_bracket != 0 => {
              string_bracket -= 1;
            }

            ch @ 'a'..='z' | ch @ 'A'..='Z' => {
              if !string_parse_curr {
                match ch {
                  'p' => {
                    // println!("{:#?}", self.stack.front())
                    match self.stack.front() {
                      Some(item) => {
                        println!("{}", item)
                      }

                      None => {
                        eprintln!("stack empty nob")
                      }
                    }
                  }
                  _ => todo!("all the commands go here"),
                }
              }
            }

            _ => {
              unreachable!()
            }
          }

          // string logic
          if string_parse_curr {
            if string_bracket == 0 {
              self
                .stack
                .push_back((&stack_temp.join("")[1..]).to_string());
              string_parse_curr = false;
              stack_temp = vec![];
            } else {
              stack_temp.push(next_char.to_string());
            }
          }

          self.code = (&self.code[1..]).to_string();
        }
      }
    }

    println!("{:#?}", self.stack);
  }
}

fn main() {
  // let code = "[chick[wow]en[]][chicken finger[]]p".to_string();
  let code = "[chick[cie]ew]p".to_string();
  let stack = VecDeque::new();
  let mut desk_calc = DeskCalc { code, stack };

  desk_calc.run();
}
