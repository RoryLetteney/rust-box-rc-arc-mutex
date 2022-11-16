use std::{rc::{Rc, Weak}, cell::RefCell};

struct Owner {
  name: String,
  tools: RefCell<Vec<Weak<Tool>>>
}

struct Tool {
  owner: Rc<Owner>
}

pub fn main() {
  let brad = Rc::from(Owner { name: "Brad".to_string(), tools: RefCell::new(vec![]) });
  let pliers = Rc::from(Tool { owner: Rc::clone(&brad) });
  let wrench = Rc::from(Tool { owner: brad.clone() });

  brad.tools.borrow_mut().push(Rc::downgrade(&pliers));
  brad.tools.borrow_mut().push(Rc::downgrade(&wrench));

  println!("Pliers owner: {}", pliers.owner.name);
  println!("Brad's pliers owner: {}", brad.tools.borrow()[0].upgrade().unwrap().owner.name);
}
