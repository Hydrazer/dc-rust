use regex::Regex;
use std::collections::VecDeque;

struct DeskCalc {
  code: String,
  stack: VecDeque<String>,
  output: String,
}

impl DeskCalc {
  fn run(&mut self) -> String {
    let num_regex = r#"(?P<num>\d*\.?\d+|\d+\.?\d*)"#;
    let comment_regex = r#"(?P<comment>#.*)"#;
    // let string_regex = r#"\[(?P<str>.*)\]"#;
    let space_regex = r#"(?P<space>\s+)"#;
    let token_regex = Regex::new(
      &vec![
        num_regex,
        // string_regex,
        comment_regex,
        space_regex,
      ]
      .into_iter()
      .map(|re| "^".to_string() + re)
      .collect::<Vec<_>>()
      .join("|"),
    )
    .unwrap();
    let mut stack_temp: Vec<String> = vec![];

    let mut string_bracket: isize = 0;
    let mut string_parse_curr = false;

    while self.code.len() != 0 {
      // regular case
      let mat = token_regex.captures(&self.code);

      if mat.is_some() && !string_parse_curr {
        let mat = mat.unwrap();
        if let Some(num) = mat.name("num") {
          self.stack.push_front(num.as_str().to_string());
        }

        self.code = (&self.code[mat.get(0).unwrap().end()..]).to_string();

      // edge case for string and commands
      } else {
        let next_char = self.code.chars().next().unwrap();

        match next_char {
          '[' => {
            string_parse_curr = true;
            string_bracket += 1;
          }

          ch if ch == ']' && string_bracket != 0 => {
            string_bracket -= 1;
          }

          ch => {
            if !string_parse_curr {
              match ch {
                'P' => {
                  match self.stack.pop_back() {
                    Some(item) => {
                      self.output += &item;
                    }

                    None => {
                      eprintln!("stack empty nob")
                    }
                  }
                },

                'p' => {
                  match self.stack.back() {
                    Some(item) => {
                      self.output += &format!("{}\n", item);
                    }

                    None => {
                      eprintln!("stack empty nob")
                    }
                  }
                }

                'f' => {
                  if self.stack.len() != 0 {
                    self.output += &format!("{}\n", self.stack.clone().into_iter().collect::<Vec<_>>().join("\n"))
                  }
                }

                _ => {
                  todo!("all the commands go here")
                }
              }
            }
          }
        }

        // string logic
        if string_parse_curr {
          if string_bracket == 0 {
            self
              .stack
              .push_front((&stack_temp.join("")[1..]).to_string());
            string_parse_curr = false;
            stack_temp = vec![];
          } else {
            stack_temp.push(next_char.to_string());
          }
        }

        self.code = (&self.code[1..]).to_string();
      }
    }

    println!("{:#?}", self.stack);

    self.output.clone()
  }
}

fn main() {
}


fn run_test(code: &str, expected: &str) {
  let stack = VecDeque::new();
  let output = "".to_string();
  let mut desk_calc = DeskCalc { code: code.to_string(), stack, output };

  assert_eq!(desk_calc.run().as_str(), expected)
}

#[cfg(test)]
mod test_print {
  use super::*;
  #[test]
  fn print_a() {
    run_test("32p", "32\n")
  }

  #[test]
  fn print_b() {
    run_test("6969", "")
  }

  #[test]
  fn print_c() {
    run_test("[aw]PP", "aw")
  }

  #[test]
  fn print_d() {
    run_test("[32]pP", "32\n32")
  }

  #[test]
  fn print_e() {
    run_test("[32][6032chcike[][[tew][rew]]ew[]n]f", "6032chcike[][[tew][rew]]ew[]n\n32\n")
  }

  #[test]
  fn print_f() {
    run_test("ffff", "")
  }

  #[test]
  fn print_g() {
    run_test("fppPppfpfpPPfpPf", "")
  }
}
