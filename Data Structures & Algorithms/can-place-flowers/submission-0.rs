

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {

       let mut vec_flowerbed: Vec<i32> = Vec::new();
       vec_flowerbed.push(0);

       for i in 0..flowerbed.len(){
        vec_flowerbed.push(flowerbed[i]);
       }

        vec_flowerbed.push(0);

        let mut count = 0;


        for i in 1..vec_flowerbed.len()-1{
           if vec_flowerbed[i-1] == 0 && vec_flowerbed[i+1] == 0 && vec_flowerbed[i] == 0 {
                vec_flowerbed[i] = 1;
                count+=1;

           }
        }

        println!("{}", count);

        count >= n



    }
}
