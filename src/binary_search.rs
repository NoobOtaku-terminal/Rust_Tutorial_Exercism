pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // todo!(
    //     "Using the binary search algorithm, find the element '{key}' in the array '{array:?}' and return its index."
        
    // );
    let mut start = 0;
    let mut end = array.len();
    if(end == 0){
        return None;
    }
    end -=1;
    while start <= end {
        let mid= start + (end - start)/2;
        if array[mid] == key{
            return Some(mid as usize);
        }else if array[mid] > key{
            if mid == 0{break;}
            end = mid-1;
        }else{
            start = mid+1;
        }
    }
    return None;
}

