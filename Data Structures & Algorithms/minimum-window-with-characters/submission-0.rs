impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let  s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let mut t_freq: [i32; 128] = [0; 128];


        for i in 0..t_bytes.len(){
            t_freq[(t_bytes[i]) as usize] +=1;
        }

        let mut start_length: usize = t_bytes.len();


        while start_length <= s_bytes.len(){
            let mut start = 0;
           


            while start+start_length <= s_bytes.len() {
                 let mut s_freq: [i32; 128] = [0; 128];
                for i in start..start+start_length{
                s_freq[(s_bytes[i]) as usize] +=1;
            }
            

            let mut count: usize = 0;
            for i in 0..t_freq.len(){
                if t_freq[i] <= s_freq[i]{
                    count+=t_freq[i] as usize;
                } 
            } 

            if count == t_bytes.len() {
               
                 let mut sub_string: Vec<u8> = Vec::new();
                for i in start..start+start_length{
                    sub_string.push(s_bytes[i]);
                }
                 println!("{:?}", sub_string);
               return String::from_utf8(sub_string).unwrap();
            }            
        
          
                 start+=1;
            }

        start_length+=1;

        }

       
        String::new()


    }
}
