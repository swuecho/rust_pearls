
  struct Thing{
    id: u32,
    extra: u32
  }

  impl Thing{
    pub fn new() -> Thing { 
      Thing { id: 3, extra: 2} 
      }
    pub fn get_total(&self) -> u32 {
      self.id + self.extra
    }
  }

fn main(){

  let my_thing = Thing::new();
  println!("the thing's total is {}", my_thing.get_total());
}
