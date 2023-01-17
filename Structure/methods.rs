struct Rectangle{
    height:i32,
    width:i32
}
impl Rectangle{
    fn area(&self)->i32{
        self.height*self.width
    }
    fn width(&self)->bool{
        self.width>0
    }
    fn height(&self)->bool{
        self.height>0
    }
    fn can_fit(&self,other:Rectangle)->bool{
        self.width>other.width && self.height>other.height
    }
    
}
fn main() {
    let rect1=Rectangle{
        height:10,
        width:10
    };
    let rect2=Rectangle{
        height:5,
        width:5
    };
    let rect3=Rectangle{
        height:-2,
        width:2
    };
    println!("{}",rect1.area());
    println!("{}",rect1.width());
    println!("{}",rect1.height());
    println!("{}",rect3.height());
    println!("{}",rect1.can_fit(rect2));
}
