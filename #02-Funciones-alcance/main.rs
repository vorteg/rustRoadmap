/* 
    Tipos de funciones

*/

fn main(){
    // funcion sin retorno
    // fn saludar(){
    //     println!("Hola Rust!")
    // }
    // funcion con parametros
    fn saludar2(nombre:&str)-> String{
        format!("Hola,{}!",nombre)
    } 

    let saludo =  saludar2("Pedro");
    println!("{}",saludo);

    // closuers funciones anonimas
    let suma = | x:i32, y:i32| x + y;
    let resultado = suma(2,3);
    println!("la suma es: {}", resultado);

    // funciones de orden superior permiter pasar funciones y regresar funciones
    fn aplicar_funcion<F>(funcion:F, valor:i32) -> i32
    where F: Fn(i32) -> i32 {
        funcion(valor)
    }

    let duplicar = |x:i32| x*2;
    let resultado = aplicar_funcion(duplicar, 10);
    println!("El doble es: {}", resultado);

    // Funciones generadoras
    //son funciones que devuelven un objeto generador, lo que le permite iterar sobre una secuencia de valores uno a la vez
    // fn fibonacci() -> impl Iterator<Item = u32> {
    //     let mut a = 0;
    //     let mut b = 1;
      
    //     move || {
    //       let next = a;
    //       a = b;
    //       b = a + next;
    //       next
    //     }
    //   }
      
    //   for numero in fibonacci().take(10) {
    //     println!("{}", numero);
    //   }

      /*
      El scope en Rust ayuda a organizar y aislar variables, evitando conflictos de nombres y mejorando la seguridad del código.
Las variables solo son accesibles dentro del bloque o función donde se declaran, a menos que se declare como static.
Consejos:

Declara las variables lo más cerca posible de su uso para mejorar la legibilidad del código y evitar confusiones.
Ten cuidado con el sombreado, ya que puede introducir errores si no se usa con atención.
Utiliza static solo cuando sea absolutamente necesario
      
      */
      fn fizzbuzz (text_1:&str,text_2:&str) -> i32{
        let my_numbers = 1..101;
        let mut count  = 0;
        for i in my_numbers{
            if i % 3 == 0 && i % 5 == 0 {
                println!("{}{}",text_1,text_2);
            }
            if i % 3 == 0  {
                println!("{}",text_1);
            }
            if i % 5 == 0 {
                println!("{}",text_2);
            }
            else{
                println!("{}",i);
                count += 1;
            }
        } 
        return count;
      }

    //fizzbuzz("Fizz","Buzz")
    let result = fizzbuzz("Fizz","Buzz");
    println!("{}",result);

}