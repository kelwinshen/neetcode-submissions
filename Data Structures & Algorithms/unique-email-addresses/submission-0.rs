use std::collections::HashSet;
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {

        let mut hashset_unique_email: HashSet<String> = HashSet::new();
        
        for i in 0..emails.len(){
            let emails_components: Vec<&str> = emails[i].split('@').collect();

           let mut local_name = String::new();
           let unclean_local_name = emails_components[0];
           
           for ch in unclean_local_name.chars() {
                if ch == '+' {
                  break; 
                } else if ch != '.' {
                    local_name.push(ch)
                }
           }

           local_name.push_str(&emails_components[1]);

           hashset_unique_email.insert(local_name);

        }

        hashset_unique_email.len() as i32
    }
}
