use std::rc::Rc;
use std::cell::RefCell;
use std::io::{self, Write};
use std::process::Command;

// Definición del tipo de lista enlazada
type Link = Option<Rc<RefCell<Node>>>;

// Definición de la estructura de un nodo de la lista enlazada
#[derive(Clone)]
struct Node {
    value: String,
    next: Link,
}

// Definición de la lista enlazada
pub struct LinkedList {
    head: Link,
}

impl LinkedList {
    // Método para crear una nueva lista enlazada
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Método para insertar un nuevo valor en la lista
    pub fn insert(&mut self, value: String) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.clone(),
        }));
        self.head = Some(new_node);
    }

    // Método para eliminar un valor de la lista
    pub fn remove(&mut self, value: &str) -> bool {
        let mut current = self.head.clone();
        let mut prev: Link = None;

        while let Some(node) = current.clone() {
            if node.borrow().value == value {
                if let Some(prev_node) = prev {
                    prev_node.borrow_mut().next = node.borrow().next.clone();
                } else {
                    self.head = node.borrow().next.clone();
                }
                return true;
            }
            prev = current.clone();
            current = node.borrow().next.clone();
        }
        false
    }

    // Método para buscar un valor en la lista
    pub fn search(&self, value: &str) -> bool {
        let mut current = self.head.clone();

        while let Some(node) = current {
            if node.borrow().value == value {
                return true;
            }
            current = node.borrow().next.clone();
        }
        false
    }

    // Método para imprimir la lista en orden inverso
    pub fn print_list_reverse(&self) {
        fn print_reverse(node: &Link) {
            if let Some(ref n) = node {
                print_reverse(&n.borrow().next);
                println!("{}", n.borrow().value);
            }
        }

        print_reverse(&self.head);
    }
}

// Función para limpiar la pantalla
fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Fallo al limpiar la pantalla");
    } else {
        Command::new("clear")
            .status()
            .expect("Fallo al limpiar la pantalla");
    }
}

// Función para pausar y esperar a que el usuario presione Enter
fn pause() {
    println!("\nPresione Enter para continuar...");
    io::stdin().read_line(&mut String::new()).unwrap();
}

// Función principal
fn main() {
    let mut list = LinkedList::new();
    loop {
        clear_screen();
        println!("\nMenu:");
        println!("1. Insertar texto");
        println!("2. Eliminar texto");
        println!("3. Buscar texto");
        println!("4. Imprimir texto");
        println!("5. Salir");

        print!("Seleccione una opción: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => {
                print!("Ingrese el texto a insertar: ");
                io::stdout().flush().unwrap();
                let mut text = String::new();
                io::stdin().read_line(&mut text).unwrap();
                let text = text.trim().to_string();
                list.insert(text);
                println!("Texto insertado.");
                pause();
            },
            2 => {
                print!("Ingrese el texto a eliminar: ");
                io::stdout().flush().unwrap();
                let mut text = String::new();
                io::stdin().read_line(&mut text).unwrap();
                let text = text.trim();
                if list.remove(text) {
                    println!("Texto eliminado.");
                } else {
                    println!("Texto no encontrado.");
                }
                pause();
            },
            3 => {
                print!("Ingrese el texto a buscar: ");
                io::stdout().flush().unwrap();
                let mut text = String::new();
                io::stdin().read_line(&mut text).unwrap();
                let text = text.trim();
                if list.search(text) {
                    println!("Texto encontrado.");
                } else {
                    println!("Texto no encontrado.");
                }
                pause();
            },
            4 => {
                println!("Texto:");
                list.print_list_reverse();
                pause();
            },
            5 => {
                println!("Saliendo...");
                break;
            },
            _ => println!("Opción no válida. Intente de nuevo."),
        }
    }
}
