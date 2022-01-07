use rand;

fn main() {
    print!("Generating random array... \n\n");

    let arr = generate_random_array();

    print!("Generated array: \n{:?}\n\n", arr);

    let (bubble_sorted_arr, number_of_swaps_bubble) = bubble_sort_with_opp_count(arr);

    println!(
        "Array sorted with bubble sort: \n{:?}\n\nOperations during bubble sorting: {}\n\n",
        bubble_sorted_arr, number_of_swaps_bubble
    );

    let (insertion_sorted_arr, number_of_swaps_insertion) = insertion_sort_with_opp_count(arr);

    println!(
        "Array sorted with insertion sort: \n{:?}\n\nOperations during insertion sorting: {}\n\n",
        insertion_sorted_arr, number_of_swaps_insertion
    );
}

fn generate_random_array() -> [i8; 20] {
    rand::random()
}

fn bubble_sort_with_opp_count(arr: [i8; 20]) -> ([i8; 20], i32) {
    let mut temp_arr = arr;
    let mut swap_no = 0;

    for i in 0..temp_arr.len() {
        for j in 0..temp_arr.len() - 1 - i {
            if temp_arr[j] > temp_arr[j + 1] {
                temp_arr.swap(j, j + 1);
                swap_no += 1;
            }
        }
    }

    (temp_arr, swap_no)
}

fn insertion_sort_with_opp_count(arr: [i8; 20]) -> ([i8; 20], i32) {
    let mut temp_arr = arr;
    let mut swap_no = 0;

    for i in 1..temp_arr.len() {
        let mut j = i;
        while j > 0 && temp_arr[j - 1] > temp_arr[j] {
            temp_arr.swap(j - 1, j);
            j -= 1;
            swap_no += 1
        }
    }

    (temp_arr, swap_no)
}
