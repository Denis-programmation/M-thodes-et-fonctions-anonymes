struct Rectangle 
{
    largeur: u32,
    hauteur: u32
}

impl Rectangle 
{
    // construit une nouvelle instance à partir des valeurs fournies.
    fn new(largeur: u32, hauteur: u32) -> Self 
    {
        Self { largeur, hauteur }
    }

    // Remplace les valeurs de l'instance existante.
    fn modif(&mut self, largeur: u32, hauteur: u32) 
    {
        self.largeur = largeur;
        self.hauteur = hauteur;
    }

    // calcul de la surface
    fn aire(&self) -> u32 
    {
        self.largeur * self.hauteur
    }

    fn perimetre(&self) -> u32
    {
        (self.largeur + self.hauteur) * 2
    }

    // Affiche les valeurs.
    fn describe(&self) 
    {
        println!("Le rectangle défini a une largeur de {} et une hauteur de {}", self.largeur, self.hauteur);
    }
}

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
