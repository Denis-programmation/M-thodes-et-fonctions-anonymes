pub struct Rectangle 
{
    largeur: u32,
    hauteur: u32
}

impl Rectangle 
{
    // construit une nouvelle instance à partir des valeurs fournies.
    pub fn new(largeur: u32, hauteur: u32) -> Self 
    {
        Self { largeur, hauteur }
    }

    // Remplace les valeurs de l'instance existante.
    pub fn modif(&mut self, largeur: u32, hauteur: u32) 
    {
        self.largeur = largeur;
        self.hauteur = hauteur;
    }

    // calcul de la surface
    pub fn aire(&self) -> u32 
    {
        self.largeur * self.hauteur
    }

    pub fn perimetre(&self) -> u32
    {
        (self.largeur + self.hauteur) * 2
    }

    // Affiche les valeurs.
    pub fn describe(&self) 
    {
        println!("Le rectangle défini a une largeur de {} et une hauteur de {}", self.largeur, self.hauteur);
    }
}
