// desenvolver uma função de fibonaci

fn main() {
    let aux: i32 = 5;
    let aux2: i32 = fib(15);
    println!("o valor da var aux eh: {aux}");
    //aux= 6;
    println!("o novo valor da var aux2 eh: {aux2}");
}

fn fib(x: i32) -> i32{
    if x <= 2{
    //println!("o valor é menor do que 1!!");
    return 1
    }
    //println!("o valor é maior do que 1!!");
    return fib(x-1) + fib(x-2)
}

x = 52; 
