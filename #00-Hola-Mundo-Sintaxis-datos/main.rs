/*
    Sintaxis Basica en Rust para 
    varias lineas
*/


//Comentario en una Linea

// let x = 5; // Esto es una variable
const PI: f64 = 3.14159; // Esto es una constante

/*

Rust tiene distintos tipos de datos que se pueden indicar como 
    Enteros con signo = i8, i16, i32, i64, i128
    Enteros sin signo = u8, u16, u32, u64, u128
    flotantes = f32, f64
    Booleanos = bool
    Caracteres = char
    tuplas = (5,"hola",3.14)
    arreglos = [1,2,3,4]

*/

//Como definir las variables
//Se puede usar "let" para variables inmutables y privadas
// let nombre_de_la_variable: tipo_de_dato = "valor";
//let numero: i32 = 10; // Variable de tipo entero con valor 10
//let mensaje: &str = "Hola, mundo!"; // Variable de tipo cadena de texto
//let es_verdadero: bool = true; // Variable de tipo booleano con valor true
//let caracter: char = 'a'; // Variable de tipo caracter con valor 'a'
//En algunos casos no es necesario definir el tipo de dato como let x = 5 
// "mut" variables mutables
/*
    let mut variable_mutable: TIPO_DE_DATO = VALOR;
variable_mutable = nuevo_valor; // Se puede cambiar el valor

*/

// "pub" estas variables pueden ser accedidas desde distintas partes del programa (globales)
// "static" para variables que no se destruyen en ejecucion

fn main(){
    let x = "Rust";
    println!("!Hola, {}! {}",x,PI)
}

//Nota siempre usar una funcion main
// Compilar con rustc en linea de comandos y 
// para guardar el archivo compilado en algun lugar especifico usar
// -o
// rustc main.rs -o <nombre_archivo_ejecutable>
// "let" no puede ser definida de manera global solo dentro de fn