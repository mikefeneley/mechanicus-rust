/*
 * Sort the elements in array data in range [low, high].
 */
fn quicksort(data: &mut [i32], low: usize, high: usize) -> bool
{
    if low < high {
        let mid: usize = partition(data, low, high);
        quicksort(data, low, mid.saturating_sub(1));
        quicksort(data, mid.wrapping_add(1), high);
    }
    return true;
}

/*
 * Move elements to correct size of parition.
 */
fn partition(mut data: &mut [i32], low: usize, high: usize) -> usize
{
    let pivot = data[low];
    let mut left:usize = low + 1;
    let mut right:usize = high;
    let mut done = false; 
    while !done {

        while left <= right && data[left] <= pivot {
            left += 1;
        }
        while right >= left && data[right] >= pivot {
            right -= 1;
        }
        if left > right { 
            done = true;
        } else {
            swap(&mut data, left, right);
        }    
    }
    swap(&mut data, low, right);
    return right;
}
/*
 * Swap elements data[left] and data[right]
 */
fn swap(data: &mut [i32], left: usize, right: usize) -> bool 
{
   let tmp = data[left];
   data[left] = data[right];
   data[right] = tmp;
   return true;
}

/*
 * Practice data for quicksort algorithim.
 */
fn main() 
{
    let mut data: [i32; 9] = [54, 26, 93, 17, 77, 31, 44, 55, 20]; 
    let len = data.len() - 1;
    quicksort(&mut data, 0, len);
}

