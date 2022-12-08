fn main() {
    println!("排序结果{:?}", bubble_sort(&mut vec![22, 33, 44, 2, 55, 66, 11, 33, 96]));
}

// 冒泡排序
fn bubble_sort(vec: &mut Vec<u32>) -> Vec<u32>{

    let mut v1 = Vec::new();
    let length = vec.len();
    let mut j = 0;
    
    while j < length - 1 {
    
        let mut i = 0;
        while i < (length - 1 - j) {
        
            if vec[i] > vec[i+1]{
                let temp = vec[i+1];
                vec[i+1] = vec[i];
                vec[i] = temp;
            }
            
            i += 1;
        }
        
        j += 1;
        
        v1.push(vec[length-j]);
    }
    
    v1
}