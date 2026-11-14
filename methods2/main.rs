mod module;
use crate::module::Rectangle;

fn main() 
{
    let mut rect1 = Rectangle::new( 30, 50 );
    rect1.describe();
    println!( "La surface de ce rectangle est {}", rect1.aire() );
    println!( "Le périmètre de ce rectangle est {}", rect1.perimetre() );
    rect1.modif( 300, 500 );
    rect1.describe();
    println!( "La surface de ce rectangle est {}", rect1.aire() );
    println!( "Le périmètre de ce rectangle est {}", rect1.perimetre() )
}
