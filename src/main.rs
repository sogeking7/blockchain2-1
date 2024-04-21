use blockchain2_1::sorting;

fn main() {
  let mut numbers = vec![4, 2, 1, 3];
  println!("Original Numbers: {:?}", numbers);

  let quick_sorted = sorting::quick_sort(&mut numbers);
  println!("After Quick Sort: {:?}", quick_sorted);

  let selection_sorted = sorting::selection_sort(&mut numbers);
  println!("After Selection Sort: {:?}", selection_sorted);

  let insertion_sorted = sorting::insertion_sort(&mut numbers);
  println!("After Insertion Sort: {:?}", insertion_sorted);

  let merge_sorted = sorting::merge_sort(&mut numbers);
  println!("After Merge Sort: {:?}", merge_sorted);
}
