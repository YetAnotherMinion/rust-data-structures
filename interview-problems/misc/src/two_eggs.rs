use std::cmp;

fn two_eggs_inner(k: u64, arr: &mut Vec<u64>, count: &mut u64) -> u64 {
    let mut ii = k as usize;
    ii -= 1;
    *count += 1;

    if k == 1 {
        arr[ii] = 1;
    } 
    
    if arr[ii] == 0 {
        let mut tmp = k;
        for x in 1..(k/2 + 1) {
            tmp = cmp::min(tmp, cmp::max(x, 1+two_eggs_inner(k-x, arr, count)))
        }
        arr[ii] = tmp;
    }
    arr[ii]
}

fn two_eggs(k: u64) -> (u64, u64) {
    let mut arr = vec![0; k as usize];
    let mut count: u64 = 0;
    (two_eggs_inner(k, &mut arr, &mut count), count)
}
        
fn main() {
    for k in 1..100 {
        let (x, y) = two_eggs(k);
        println!("For {} floors, minimum number of moves is {} [{} steps]", k, x, y);
    }
}
