use std::rc::Rc;
use std::cell::RefCell;

pub struct Person {
    pub name: String,
    pub height: f64,
    pub weight: f64,
    pub age: u8,
    pub sex: char,
    pub mother: Option<Rc<RefCell<Person>>>,
    pub father: Option<Rc<RefCell<Person>>>,
    pub children: Vec<Rc<RefCell<Person>>>,
}

impl Person {
    pub fn new(name: String, height: f64, weight: f64, age: u8, sex: char) -> Rc<RefCell<Person>> {
        Rc::new(RefCell::new(Person {
            name,
            height,
            weight,
            age,
            sex,
            mother: None,
            father: None,
            children: Vec::new(),
        }))
    }
}




