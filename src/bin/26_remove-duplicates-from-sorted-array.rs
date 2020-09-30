fn main() {
    let mut nums = vec![1,1,2];
    let len = remove_duplicates(&mut nums);
    println!("len:{}", len);
    println!("nums:{:?}", nums);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0{
        return 0 as i32;
    }
    let mut j = 0;
    for i in 1..nums.len() {
        if nums[i] != nums[j] {
            j += 1;
            nums[j]=nums[i];
        }
    }
    j as i32 + 1
}

