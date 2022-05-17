use std::collections::HashMap;

fn main() {
  // 初期値
  let mut vec: Vec<i32> = vec![3, 3, 0, 3, 3, 9, 9]; 
  let vec_num: usize = 7; // ベクタの数

  // 並び替え
  quick_sort(&mut vec, 0, 6);
  println!("ベクタ: {:?}", vec);

  // 平均値
  let mean = mean(&vec);
  println!("平均値: {}", mean);

  // 中央値
  let median: usize = &vec_num / 2;
  println!("中央値: {}", vec[median]);

  // 最大頻出値
  let max_num = mode(&vec);
  println!("最大頻出値: {:?}", max_num);
}

fn mean(vec: &Vec<i32>) -> i32 {
  let mut mean: i32 = 0;
  let mut cnt: i32 = 0;
  for num in vec {
    mean += num;
    cnt += 1;
  }
  mean / cnt
}

fn quick_sort(vec: &mut Vec<i32>, left: usize, right: usize) {
  let mut i = left;
  let mut j = right;
  let pivot = vec[(right + left) / 2];
  
  loop {
    while &vec[i] < &pivot {
      i += 1;
    }
    while &pivot < &vec[j] {
      j -= 1;
    }
    if i >= j {
      break;
    }
    swap(vec, &i, &j);
    i += 1;
    j -= 1;
  }

  if left < i - 1 {
    quick_sort(vec, left, i - 1);
  }
  if j + 1 <  right {
    quick_sort(vec, j + 1, right);
  }
}

fn swap(vec: &mut Vec<i32>, i: &usize, j: &usize) {
  let tmp = vec[*i as usize];
  vec[*i as usize] = vec[*j as usize];
  vec[*j as usize] = tmp;
}

fn mode(vec: &Vec<i32>) -> Vec<i32> {
  let mut map = HashMap::new();
  for num in vec {
    let cnt = map.entry(num).or_insert(0);
    *cnt += 1;
  }

  let max_num = max_num(&map);
  max_num
}

fn max_num(map: &HashMap<&i32, i32>) -> Vec<i32> {
  let mut vec: Vec<i32> = Vec::new();
  let mut max_num = 0;
  for num in map {
    if &max_num < num.1  {
      vec = vec![**num.0];
    }else if num.1 < &max_num {
      max_num = *num.1;
    }else {
      vec.push(**num.0);
    }
  }
  vec
}