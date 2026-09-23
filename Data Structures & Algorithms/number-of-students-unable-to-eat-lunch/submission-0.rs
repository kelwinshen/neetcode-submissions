impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut count_0 = 0;
        let mut count_1 = 0;

        for &student in &students {
            if student == 0 {
                count_0 += 1;
            } else {
                count_1 += 1;
            }
        }

        for sandwich in sandwiches {
            if sandwich == 0 {
                if count_0 > 0 {
                    count_0 -= 1;
                } else {
                    break; // No students left who want a 0 sandwich
                }
            } else {
                if count_1 > 0 {
                    count_1 -= 1;
                } else {
                    break; // No students left who want a 1 sandwich
                }
            }
        }

        count_0 + count_1
    }
}