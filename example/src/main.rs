fn main() {
    println!("{}", fibonacci(10));
    println!("{}", fib_recursion(10));
}

fn fibonacci(n: usize) -> i32 {
    let ans: i32 = if n==0 || n==1 {
        n as i32
    } else {
        let mut array: [i32; 500] = [0; 500];
        array[0] = 0;
        array[1] = 1;

        for i in 2..n+1 {
            array[i] = array[i-1] + array[i-2];
        }
        
        array[n]
    };

    ans
}

fn fib_recursion(n: i32) -> i32 {
    if n==1 || n==0 {
        n
    } else {
        fib_recursion(n-1) + fib_recursion(n-2)
    }
}

