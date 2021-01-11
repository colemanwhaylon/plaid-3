extern crate phrases;
use phrases::greetings::spanish;

extern crate rand;
use rand::Rng;

fn print_values(x: u32)
{
    println!("(printValues) values = {}", x);
}

fn add_to_expression(x: &mut String)
{
    *x += " Today ";
    println!("(addToExpression) {}", *x);
}

fn products(x: String, y: String) -> String
{
    "(products) ".to_string() + &x + &y
}

fn is_even(x: u32) -> bool
{
    x % 2 == 0
}

fn closures()
{
    let plus_one = |x:i32| -> i32 {x + 1};
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let plus_two = |x|
    {
        let mut z = x;
        z += 2;
        z
    };
    println!("{} + 2 = {}", 3, plus_two(3));
}

fn higher_order()
{
    let limit = 1500;

    let sum2 = (0..).map(|x| x*x)
         .take_while(|&x| x < limit)
         .filter(|x| is_even(*x))
         .fold(0, |sum,x| sum + x);
    println!("hof sum = {}", sum2);
}

trait Animal
{
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) 
    {
        println!("{} cannot talk", self.name());
    }
}

struct Human
{
    name: &'static str
}

struct Cat
{
    name: &'static str
}

impl Animal for Human
{
    fn create(name: &'static str) -> Human
    {
        Human{name: name}
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat
{
    fn create(name: &'static str) -> Cat
    {
        Cat{name: name}
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says meow", self.name());
    }
}

trait Summable<T>
{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut result:i32 = 0;
        for x in self {result += *x; }
        return result;
    }
}

fn sum_and_product(x:i32, y:i32) -> (i32, i32)
{
    (x+y, x*y)
}

fn tuples()
{
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);

    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

}

fn traits()
{
    let h:Human = Animal::create("John");
    let c = Cat{name: "Pussy"};
    h.talk();
    c.talk();

    let a = vec![1,2,3];
    println!("sum = {}", a.sum());
}


struct Point
{
    x: f64,
    y: f64
}

struct Line
{
    start: Point,
    end: Point
}

impl Line
{
    fn len(&self) -> f64
    {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}

fn main()
{
    let mut rng = rand::thread_rng();
    let b:u32 = rng.gen();
    print_values(b);
    add_to_expression(&mut b.to_string());
    let z = products("x ".to_string(), "u ".to_string());
    println!("{}", z);
    
    println!("");


    let p = Point{x: 3.0, y:5.0};
    let p2 = Point{x: 5.0, y:7.0};
    let myline = Line{start: p, end: p2};
    println!("length = {}", myline.len());
/*
    println!("B's value: {}", b);
    println!("");

    println!("English: {} {} ", 
        phrases::greetings::english::hello(),
        phrases::greetings::english::goodbye()
    );

*/    
    println!("Spanish: {} {} ", 
        spanish::hello(),
        spanish::goodbye()
    );

    closures();
    higher_order();
    traits();
    tuples();

}