// desenvolver uma função de fibonaci

fn main() {
    let aux: i32 = 5;
    let aux2: i32 = 5*aux;
    let _aux3: i32 = fun(6);
    println!("o valor da var aux eh: {aux}");
    //aux= 6;
    println!("o novo valor da var aux2 eh: {aux2}");
}

fn fun(x: i32) -> i32{
    return x + 1
}
// não funcionou, apesar disso preciso de uma configuração correta a partir do sistema de dados de rust
