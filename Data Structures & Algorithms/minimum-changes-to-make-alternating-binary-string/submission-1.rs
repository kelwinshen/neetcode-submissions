impl Solution {
    pub fn min_operations(s: String) -> i32 {
      let mut expect = '2';
      let mut count = 0;

      for ch in s.chars(){
        if ch != expect && expect != '2' {
            count +=1;
            expect = ch;
        } else {
            if ch == '1' {
                expect = '0';
            } else {
                 expect = '1';
            }
        }
      }

      count.min(s.len() as i32 - count)


     

    }
}