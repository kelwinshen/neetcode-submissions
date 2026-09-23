struct MinStack {
    pub stacks: Vec<(i32, i32)>
}

impl MinStack {
    pub fn new() -> Self {
        MinStack{
            stacks: Vec::new()
        }
    }

    pub fn push(&mut self, val: i32) {
        if self.stacks.is_empty(){
             self.stacks.push((val,val));
        } else {
            let latest_min_value = self.stacks.last().unwrap().1;
            if latest_min_value > val {
                  self.stacks.push((val,val));
            } else  {
                 self.stacks.push((val,latest_min_value));
            }
        }
       
    }

    pub fn pop(&mut self) {
          self.stacks.pop();
    }

    pub fn top(&self) -> i32 {
      self.stacks.last().unwrap().0
    }

    pub fn get_min(&self) -> i32 {
         self.stacks.last().unwrap().1
    }
}
