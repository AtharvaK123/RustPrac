fn main() {
    let x: i32 = 5;
    let mut y: i32 = 10;
    let mut arr = [12, 43, 34, 46, 35, 23, 13];

    // String
    println!("Hello, world!");

    //Int
    let result = add(x, y);
    println!("The sum of {} and {} is: {}", x, y, result);

    //for loop
    for i in 0..11
    {
        if i % 2 == 0 
        {
            print!("{}", i);
        }
    }
    
    println!();

    for mut i in 1..arr.len()
    {
        if arr[i] < arr[i-1] 
        {
            let temp = arr[i];
            arr[i] = arr[i-1];
            arr[i-1] = temp;
            i = 1;
        }
        
    }
    print!("{:?}", arr);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
