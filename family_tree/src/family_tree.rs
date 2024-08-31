use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::person::Person;

pub struct FamilyTree {
    pub mother: Option<Rc<RefCell<Person>>>,
    pub father: Option<Rc<RefCell<Person>>>,
    pub name_map: HashMap<String, Rc<RefCell<Person>>>,
}

impl FamilyTree {
    pub fn new(mother: Option<Rc<RefCell<Person>>>, father: Option<Rc<RefCell<Person>>>) -> Self {
        let mut name_map = HashMap::new();
        if let Some(mother) = &mother {
            name_map.insert(mother.borrow().name.clone(), Rc::clone(mother));
        }
        if let Some(father) = &father {
            name_map.insert(father.borrow().name.clone(), Rc::clone(father));
        }
        FamilyTree { mother, father, name_map }
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Person>>, mother: &Rc<RefCell<Person>>, father: &Rc<RefCell<Person>>) {
        child.borrow_mut().mother = Some(Rc::clone(mother));
        child.borrow_mut().father = Some(Rc::clone(father));

        if !self.contains_child(&mother.borrow().children, &child) {
            mother.borrow_mut().children.push(Rc::clone(&child));
        }

        if !self.contains_child(&father.borrow().children, &child) {
            father.borrow_mut().children.push(Rc::clone(&child));
        }

        // Add the child to the name map
        self.name_map.insert(child.borrow().name.clone(), Rc::clone(&child));

        // add the parents to the hashmap 
        // if they exist the hashmap will not be updated
        self.name_map.insert(mother.borrow().name.clone(), Rc::clone(mother));
        self.name_map.insert(father.borrow().name.clone(), Rc::clone(father));
    }

    pub fn contains_child(&self, children: &Vec<Rc<RefCell<Person>>>, child: &Rc<RefCell<Person>>) -> bool {
        self.name_map.contains_key(child.borrow().name.as_str()) || children.iter().any(|c| Rc::ptr_eq(c, child))
    }

    pub fn get_siblings_vec(&self, person: &Rc<RefCell<Person>>, sibling_type: char) -> Vec<Rc<RefCell<Person>>> {
        let mut siblings = Vec::new();
    
        // If looking for maternal or all siblings
        if sibling_type == 'm' || sibling_type == 'a' {
            if let Some(mother) = &person.borrow().mother {
                for child in &mother.borrow().children {
                    // Add the child if it's not the person itself and not already in the siblings list
                    if !Rc::ptr_eq(child, person) && !siblings.iter().any(|sibling| Rc::ptr_eq(sibling, child)) {
                        siblings.push(Rc::clone(child));
                    }
                }
            }
        }
    
        // If looking for paternal or all siblings
        if sibling_type == 'f' || sibling_type == 'a' {
            if let Some(father) = &person.borrow().father {
                for child in &father.borrow().children {
                    // Add the child if it's not the person itself and not already in the siblings list
                    if !Rc::ptr_eq(child, person) && !siblings.iter().any(|sibling| Rc::ptr_eq(sibling, child)) {
                        siblings.push(Rc::clone(child));
                    }
                }
            }
        }
    
        siblings
    }
    
    pub fn get_spouses_vec(&self, person: &Rc<RefCell<Person>>) -> Vec<Rc<RefCell<Person>>> {
        let mut spouses = Vec::new();
    
        for child in &person.borrow().children {
            if let Some(mother) = &child.borrow().mother {
                if !Rc::ptr_eq(mother, person) && !spouses.iter().any(|spouse| Rc::ptr_eq(spouse, mother)) {
                    spouses.push(Rc::clone(mother));
                }
            }
            if let Some(father) = &child.borrow().father {
                if !Rc::ptr_eq(father, person) && !spouses.iter().any(|spouse| Rc::ptr_eq(spouse, father)) {
                    spouses.push(Rc::clone(father));
                }
            }
        }
    
        spouses
    }
    

    pub fn get_siblings(&self, person: &Rc<RefCell<Person>>, sibling_type: char) -> Option<Vec<Rc<RefCell<Person>>>> {
        let siblings = self.get_siblings_vec(person, sibling_type);
        if siblings.is_empty() {
            None
        } else {
            Some(siblings)
        }
    }

    pub fn get_spouses(&self, person: &Rc<RefCell<Person>>) -> Option<Vec<Rc<RefCell<Person>>>> {
        let spouses = self.get_spouses_vec(person);
        if spouses.is_empty() {
            None
        } else {
            Some(spouses)
        }
    }   

    pub fn get_person_by_name(&self, name: &str) -> Option<Rc<RefCell<Person>>> {
        self.name_map.get(name).map(Rc::clone)
    }

    fn merge_maps_add_prim(&mut self, other: &FamilyTree){
        for (name, person) in &other.name_map{
            self.name_map.insert(name.clone(), Rc::clone(person));
        }
    }
    pub fn merge_maps_add(&mut self, other: &FamilyTree) -> bool{
        // first check for any relationship to see if merge is valid by checking the hashmaps for relationships 
        // if the relationship exists then merge is valid and we do there merge while returning true
        // else we return false

        for (name, person) in &other.name_map{
            if self.name_map.contains_key(name){
                self.merge_maps_add_prim(other);
                return true; 
            }
        }
        false
    }
    
}


