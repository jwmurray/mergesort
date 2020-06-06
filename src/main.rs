use  rand::Rng;

fn merge(a_array:&[i32], b_array:&[i32], output:&mut [i32]){
    let mut a_index = 0;
    let mut b_index = 0;
    let mut o_index = 0;
    let a_len = a_array.len();
    let b_len = b_array.len();

    while a_index < a_len && b_index < b_len {
        if a_array[a_index] < b_array[b_index]  {
            output[o_index] = a_array[a_index];
            a_index += 1;
        } else {
            output[o_index] = b_array[b_index];
            b_index += 1;
        }
        o_index += 1;
    }
    if a_index < a_len {
        output[o_index..].copy_from_slice(&a_array[a_index..]);
    } else if b_index < b_len {
        output[o_index..].copy_from_slice(&b_array[b_index..]);
    }
}

fn merge_sort( array: &mut [i32]) {
    let n = array.len();
    let m = n/2;

    if n <= 1 {
        return;
    }

    merge_sort(&mut array[0..m]);
    merge_sort(&mut array[m..n]);

    let mut y: Vec<i32> = array.to_vec(); // creates copy of array
    merge(&array[0..m], &array[m..n], &mut y[..]);
    array.copy_from_slice(&y[..]);
}

fn main() {

    let mut array: Vec<i32> = vec![4,3,2,1];
    println!("before: {:?}", array);

    merge_sort(&mut array[..]);
    println!("after: {:?}", array);


    let mut array: Vec<i32> = vec![4,3,2,1];
    let mut rng = rand::thread_rng();

    for _i in 0..1_000_000u64 {
        array.push(rng.gen());
    }
    println!("after: {:?}", &array[0..10]);

    merge_sort(&mut array[..]);
    for i in 0..10 {
        println!("after: {} {}", i, array[i]);

    }

}
