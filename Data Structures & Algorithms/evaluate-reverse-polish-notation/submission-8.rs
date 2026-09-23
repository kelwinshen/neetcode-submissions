impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stacks: Vec<i32> = Vec::new();

        for token in tokens {
            match token.as_str() {
                "+" => {
                         let  right = stacks.pop().unwrap();
                    let  left = stacks.pop().unwrap();
                
                    stacks.push(left + right);
                }, 
                "-" =>{
                         let  right = stacks.pop().unwrap();
                       let  left = stacks.pop().unwrap();
                
                    stacks.push(left - right);
                },
                "*" =>{
                    let  right = stacks.pop().unwrap();
                       let  left = stacks.pop().unwrap();
                     
                    stacks.push(left * right);
                },
                "/" =>{
                      let  right = stacks.pop().unwrap();
                       let  left = stacks.pop().unwrap();
                   
                    stacks.push(left / right);
                },
                _=>{
                    stacks.push(token.parse::<i32>().unwrap());
                }
            }
        }


    stacks[0]

    }
}
