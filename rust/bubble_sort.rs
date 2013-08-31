fn main()  {
  let a: ~[int] = ~[2, 1, 3, 9, 5, 20, 13, 40, 4];
  assert!(bubble_sort(a) == ~[1,2,3,4,5,9,13,20,40]);
}

fn bubble_sort(vector: &[int]) -> ~[int] {
  let mut sorted: ~[int] = vector.to_owned();
  let length: uint = sorted.len() - 1;
  if length == 0 { return sorted; }

  let mut swapped = true;

  while swapped {
    swapped = false;
    for i in range(0, length) {
      if sorted[i] > sorted[i+1] {
        sorted.swap(i, i+1);
        swapped = true;
      }
    }
  }

  sorted
}
