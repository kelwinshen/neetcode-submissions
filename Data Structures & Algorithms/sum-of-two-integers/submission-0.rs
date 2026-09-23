impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
          while b != 0 {
        let carry = a & b; // Find positions where both bits are 1
        a = a ^ b;         // Sum bits without carry
        b = carry << 1;    // Shift carry to add it in the next iteration
    }
    a
    }
}
