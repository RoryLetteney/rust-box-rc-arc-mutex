use std::sync::{Arc, Weak, Mutex};
use std::thread;

struct Owner {
  name: String,
  tools: Mutex<Vec<Weak<Tool>>>
}

struct Tool {
  owner: Arc<Owner>
}

pub fn main() {
  println!("ARC BEGIN");
  let brad = Arc::from(Owner { name: "Brad".to_string(), tools: Mutex::new(vec![]) });
  let pliers = Arc::from(Tool { owner: Arc::clone(&brad)} );
  let wrench = Arc::from(Tool { owner: brad.clone()} );

  for i in 0..10 {
    let brad = Arc::clone(&brad);
    let pliers = Arc::clone(&pliers);
    let wrench = Arc::clone(&wrench);
    
    let child = thread::spawn(move || {

      let mut guard = brad.tools.lock().unwrap();
      guard.push(Arc::downgrade(&pliers));
      guard.push(Arc::downgrade(&wrench));

      println!("Brad's pliers owner: {}", guard[0].upgrade().unwrap().owner.name);
    
      println!("Pliers owner: {}", pliers.owner.name);

      println!("Thread {} END", i);
    });

    let _res = child.join();
  }

  println!("ARC END");
}
