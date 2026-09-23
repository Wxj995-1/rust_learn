pub trait Draw
{
    fn draw(&self);
}

/*
- 泛型 <T: Draw>：编译期就把 T 替换成具体类型，每种类型生成一份代码 → 性能好，但一个容器只能放同一种类型。
- trait 对象 dyn Draw：编译期不知道具体类型，运行期通过虚表（vtable）查"这个对象该调哪个 draw" → 能混合放不同类型，
  代价是一点点运行时开销（间接调用）。
*/
pub struct Screen
{
    pub components: Vec<Box<dyn Draw>>, // trait对象，使用dyn关键字
} 

impl Screen{
    pub fn run(&self)
    {
        for comp in self.components.iter()
        {
            comp.draw();
        }
    }

}

pub struct Button
{
    pub width: u32,
    pub height: u32,
    pub label:String,
}

impl Draw for Button
{
    fn draw(&self)
    {
        println!("draw Button! width = {}, height = {}, label = {}", self.width, self.height, self.label);
    }
}



pub struct SelectBox
{
    pub width: u32,
    pub height: u32,
    pub option: Vec<String>,
}

impl Draw for SelectBox
{
    fn draw(&self)
    {
        println!("draw SelectBox! width = {}, height = {}, option = {:?}", self.width, self.height, self.option);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
