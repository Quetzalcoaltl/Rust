// desenvolver uma função de fibonaci

fn main() {
    let aux: i32 = 5;
 
    println!("Inicio do codigo");
    let str_Retorno = fib(aux);
    //fib(aux);
    println!(str_Retorno);
    println!("Final do codigo!");

}

fn fib(x: i32) {
    println!("O valor do argumento de entrada eh: {x}");
    if x>=1{
        return "O valor é maior que 1!!";
    }
    else {
        return "O valor é menor ou igual a 1!!";
    }
}