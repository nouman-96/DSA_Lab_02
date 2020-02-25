
#[derive(Debug)]
struct Item {
    name : String,
    price : f32,
    quantity : i32
}

#[derive(Debug)]
struct Cart {
    list : Vec<Item>,
    curr_pos : i32,
    capacity : i32
}

impl Cart{
    fn insert(&mut self, item:Item){
        self.list.push(item);
    }
    
    fn showItems(&self){
        println!("The item in the cart are");
        for i in &self.list{
            println!("{:?}",i);
        }
    }
}

fn main() {

    let I1 = Item{name:"Muneeb".to_string(), price:100.5, quantity:4};
    let I2 = Item{name:"Usama".to_string(), price:200.6, quantity:6};
    let mut v: Vec<Item> = Vec::new();
    let mut C1 = Cart{list:v,curr_pos:0,capacity:20};
    C1.insert(I1);
    C1.insert(I2);
    C1.showItems();
}
