// desenvolver uma função de fibonaci
use std::collections::HashMap;

fn main() {
    let mut valores_ja_iterados: HashMap<i32, i32> = HashMap::new();
    valores_ja_iterados.insert(1, 1);
    println!("Funcionou? {}", valores_ja_iterados.get(&1).unwrap());
}



----
// desenvolver uma função de fibonaci
use std::collections::HashMap;

fn main() {
    let mut valores_ja_iterados: HashMap<i32, i32> = HashMap::new();
    valores_ja_iterados.insert(1, 1);
    if valores_ja_iterados.contains_key(&1) {println!("Funcionou");}
    //println!("Funcionou? {}", valores_ja_iterados.get(&1).unwrap());
}