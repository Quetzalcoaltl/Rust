// desenvolver uma função de fibonaci
use std::collections::HashMap;

fn main() {
    let mut _valores_ja_iterados: HashMap<i32, i32> = HashMap::new();
   // _valores_ja_iterados.insert(45, 13);
    let aux: i32 = fib(5, _valores_ja_iterados) ;
    println!("o valor da var aux eh: {aux}");
}

fn fib(x: i32, mut _valores_conhecidos: HashMap<i32, i32>) -> i32{
    if x <= 2{ return 1 }
    if _valores_conhecidos.contains_key(&x){ return *_valores_conhecidos.get(&x).unwrap() as i32 }
    let mut _valores_1: HashMap<i32, i32> = HashMap::new();
    _valores_conhecidos.insert(x, fib(x-1, &_valores_conhecidos) + fib(x-2, &_valores_conhecidos));

    return  *_valores_conhecidos.get(&x).unwrap() as i32
}