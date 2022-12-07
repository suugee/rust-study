fn main() {
    let mut arr:[i32;5] = [4, 65,2, 83, 782];
    
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }

    println!("{:?}", arr);

}

