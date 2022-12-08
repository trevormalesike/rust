struct Stack<T>{
    items:Vec<T>,
}
impl <T> Stack<T>{
    fn new()->Self{
        Stack{items:Vec::new()}
    }
    fn push(&mut self,item:T){
        self.items.push(item);
    }
    fn pop(&mut self)->Option<T>{
        self.items.pop()
    }
    fn is_empty(&self)->bool{
        self.items.is_empty()
    }
}
fn main(){
    let mut stack=Stack::new();
    stack.push(1);
    stack.push(2);
    while let Some(item)=stack.pop(){
        println!("{}",item);
    }
}