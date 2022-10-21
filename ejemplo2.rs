//example struct Employee
//variables let (cuando defines una variable se le dará un tipo.) y puede definirlo de manera explicita o puede ser de manera implicita definido por el compilador
//u32 definiria de manera explicita el tipo. tipo de entero o entero sin signo
//

struct Employee {
  name: String,
  company: String,
  employee_id: u32,
  profile: String
 
}

fn main() { 
  let value = Employee {
    name: String::from("Geek"),
    company: String::from("Geeksforgeeks.org"),
    employee_id: 007,
    profile:String::from("Manager"),
  };
  
  println!("Employee: {} of {} Company bearing EmployeeID {} is of {} level.",
    value.name,
    value.company,
    value.employee_id,
    value.profile);
}