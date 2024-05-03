/*
    Operadores basicos

*/


fn main(){
    //destructuracion
    let tupla = (10, "Hola", 3.14);

    match tupla {
    (valor1, valor2, valor3) => {
        println!("Valor 1: {}", valor1);
        println!("Valor 2: {}", valor2);
        println!("Valor 3: {}", valor3);
    }
    }
    // bucle for
    let vector = [1, 2, 3, 4, 5];

    for numero in vector {
    println!("Numero: {}", numero);
    }
    // bucle while

    let mut contador = 0;

    while contador < 10 {
    println!("Contador: {}", contador);
    contador += 1;
    }

    let edad = 18;

    if edad >= 18 {
    println!("Eres mayor de edad");
    } else {
    println!("Eres menor de edad");
    }

    let suma = 10 + 5;
    let resta = 20 - 3;
    let multiplicacion = 4 * 6;
    let division = 15 / 3;

    println!("Suma: {}", suma);
    println!("Resta: {}", resta);
    println!("Multiplicación: {}", multiplicacion);
    println!("División: {}", division);

    let base = 2;
    let exponente = 3;

    let potencia = base ^ exponente;

    println!("{} elevado a la {} potencia es: {}", base, exponente, potencia);

    let numero1 = 10;
    let numero2 = 15;

    let es_igual = numero1 == numero2;
    let es_mayor = numero1 > numero2;
    let es_menor = numero1 < numero2;

    println!("¿{} es igual a {}?: {}", numero1, numero2, es_igual);
    println!("¿{} es mayor que {}?: {}", numero1, numero2, es_mayor);
    println!("¿{} es menor que {}?: {}", numero1, numero2, es_menor);

    let numero1 = 10;
    let numero2 = 15;
    let numero3 = 5;

    let es_mayor_que_suma = numero1 > numero2 + numero3;
    let es_par = numero1 % 2 == 0;

    println!("¿{} es mayor que la suma de {} y {}?: {}", numero1, numero2, numero3, es_mayor_que_suma);
    println!("¿{} es un número par?: {}", numero1, es_par);

    let my_numbers = 10..56;

    for num in my_numbers {
        if num % 2 == 0 && num % 3 == 1 &&  num != 16{
            println!("{}",num);

        }
    }

}