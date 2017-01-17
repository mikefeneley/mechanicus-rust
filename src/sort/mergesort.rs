fn mergesort(data: &mut std::vec::Vec<i32>) -> bool
{
    if data.len() > 1 {

        let mid:usize = data.len() / 2;
        let upper:usize = data.len() - mid;
     
        let mut left = vec![0; mid];
        let mut right = vec![0; upper];
        
        for x in 0..mid
        {       
            left[x] = data[x];
        } 
    
        for x in mid..data.len()
        {
            right[x-mid] = data[x]; 
        }
        
        mergesort(&mut left);
        mergesort(&mut right); 
        
        let mut i:usize = 0;
        let mut j:usize = 0;
        let mut k:usize = 0;
        
        while i < left.len() && j < right.len() {
            if left[i] < right[j] {
                println!("1");
                data[k] = left[i];
                i += 1;
            } else {
                println!("2");
                data[k] = right[j];
                j += 1;
            };
            k += 1;
        }
        while i < left.len() {
            data[k] = left[i];
            i += 1;
            k += 1;
        }
        while j < right.len() {
            data[k] = right[j];
            j += 1;
            k += 1;
        }
    }
    return true;
}

/*
 * Practice data for mergesort algorithim.
 */
fn main() 
{
    let mut data:std::vec::Vec<i32> = vec![54, 26, 93, 17, 77, 31, 44, 55, 20]; 
    mergesort(&mut data);
}

