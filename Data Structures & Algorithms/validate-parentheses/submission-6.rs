impl Solution {
    pub fn is_valid(s: String) -> bool {

        let mut bytes = s.as_bytes();

        let mut curr_bracket: Vec<u8> = Vec::new();

        if bytes.len()%2 == 1 {
            return false;
        }

        for i in 0..bytes.len(){
            if bytes[i] == b'(' || bytes[i] == b'{' || bytes[i] == b'[' {
                curr_bracket.push(bytes[i]);
            } else if bytes[i] == b')' || bytes[i] == b'}' || bytes[i] == b']' {
                let mut reduce = 0;
                  if bytes[i] == b')' {
                    reduce = 1;
                  } else {
                    reduce = 2;
                  }
                if !curr_bracket.is_empty() && bytes[i] - reduce == *curr_bracket.last().unwrap() {
                    curr_bracket.pop();
                } else {
                      curr_bracket.push(bytes[i]);
                }
              
            } 
        }

        curr_bracket.len() == 0
    }
}
