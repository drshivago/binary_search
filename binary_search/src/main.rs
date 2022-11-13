fn search(nums: Vec<i32>, target: i32) -> i32 {
    return nums[3] + target
}

fn main() {
    let nums: Vec<i32> = Vec::from([-1,0,3,5,9,12]);
    let target: i32 = 9;
    println!("{}", search(nums, target));

}
