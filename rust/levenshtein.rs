extern mod extra;
use extra::treemap::TreeMap;

fn main() {
  let x = levenshtein_distance("zamanfoo", "mazoo");
  println(fmt!("%u", x));
}

#[deriving(TotalOrd, TotalEq)]
struct Point(uint, uint);

fn levenshtein_distance(word1: &str, word2: &str) -> uint {
  let word1_length = word1.len() + 1;
  let word2_length = word2.len() + 1;

  let mut matrix: TreeMap<Point, uint> = TreeMap::new();
  let matrix_find = |x, y| { *matrix.find(&Point(x, y)).unwrap() };

  matrix.insert(Point(0, 0), 0);
  for i in range(1, word1_length) { matrix.insert(Point(i, 0), i); }
  for j in range(1, word2_length) { matrix.insert(Point(0, j), j); }

  for j in range(1, word2_length) {
    for i in range(1, word1_length) {
      let x: uint = if word1[i - 1] == word2[j - 1] {
        matrix_find(i - 1, j - 1)
      }
      else {
        let min_distance = do [[i - 1, j], [i, j - 1], [i - 1, j - 1]].map |vec| {
          matrix_find(vec[0], vec[1])
        };
        *min_distance.iter().min().unwrap() + 1
      };

      matrix.insert(Point(i, j), x);
    }
  }

  matrix_find(word1_length - 1, word2_length - 1)
}
