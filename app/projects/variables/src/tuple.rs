fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let w: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = w.0;
    let six_point_four = w.1;
    let one = w.2;

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    
    let months = [
      "January",
      "February",
      "March",
      "April",
      "May",
      "June",
      "July",
      "August",
      "September",
      "October",
      "November",
      "December"
    ];
}