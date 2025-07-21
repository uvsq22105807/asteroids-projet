//! Module pour gérer les astéroïdes dans le jeu.
//! Ce module contient la structure 'Asteroid' ainsi que les méthodes pour les créer, déplacer et manipuler dans le jeu.

use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use std::f32::consts::PI;

/// Structure représentant un astéroïde dans le jeu.
/// Un 'Asteroid' est caractérisé par une position, une vitesse et une taille.
/// La taille de l'astéroïde détermine sa résistance (difficulté à le détruire) et sa taille à l'écran.
pub struct Asteroid {
    position: Vec2, // Vecteur 2 dimensions qui représente la position de l'astéroïde: (x,y)
    speed: Vec2, // Vecteur 2 dimensions qui représente sa vitesse: (1.0, 0.0) = il va vers la doite par ex.
    speed_min: Vec2,
    taille: u8, // Taille de l'astéroïde: 1 = petit, 2 = moyen, 3 = grand. Valeur u8 tirée au sort par la suite.
    resistance: u8, // Nombre de missiles necessaires pour détruire l'astéroïde.
}

impl Asteroid {
    /// Constante représentant la taille initiale des astéroïdes.
    pub const ASTEROID_INIT_SIZE: f32 = 60.0;

    /// Méthode qui permet de créer un nouvel astéroïde avec une position et une vitesse aléatoires.
    /// Lors de la création, la taille de l'astéroïde est donnée aléatoirement.
    /// Cette taille défini la résistance de l'astéroïde.
    /// # Retourne un objet 'Asteroid'
    pub fn new() -> Self {
        // Génère une taille entre 1 (petit), 2 (moyen), et 3 (grand) aléatoirement.
        let mut rng = thread_rng();
        let taille = rng.gen_range(1..=3);
        // La vitesse est choisie avec la méthode "new_alea_speed()", aléatoirement.
        let vitesse = Self::new_alea_speed();
        // La résistance de l'astéroïde dépend de sa taille.
        let resistance: u8 = match taille {
            1 => 1,
            2 => 3,
            3 => 5,
            _ => 0,
        };

        // Retourne un objet avec une position, une vitesse, une taille et une résistance.
        Self {
            position: Self::new_alea_pos(),
            speed: vitesse,
            speed_min: vitesse,
            taille,
            resistance,
        }
    }

    /// Crée un nouvel astéroïde de taille spécifique et à une position donnée.
    pub fn nouvel_asteroid(taille: u8, position: Vec2) -> Self {
        let vitesse = Self::new_alea_speed(); // Générer une nouvelle vitesse aléatoire

        Self {
            position,
            speed: vitesse,
            speed_min: vitesse,
            taille,
            resistance: match taille {
                1 => 1,
                2 => 3,
                3 => 5,
                _ => 1,
            },
        }
    }

    /// Retourne la position actuelle de l'astéroïde.
    /// # Retourne un vecteur 'Vec2' avec les positions x et y de l'astéroïde dans l'espace de jeu.
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Retourne la taille de l'astéroïde (1 = petit, 2 = moyen, 3 = grand)
    pub fn get_taille(&self) -> u8 {
        self.taille
    }

    /// Retourne la résistance actuelle de l'astéroïde. (Nombre de missiles qui peuvent encore taper l'astéroïde avant sa destruction)
    pub fn get_resistance(&self) -> u8 {
        self.resistance
    }

    /// Retourne la résistance de l'astéroïde de 1 (lors de l'impact avec un missile)
    pub fn diminuer_résistance(&mut self) {
        if self.resistance > 0 {
            self.resistance -= 1;
        }
    }

    /// Indique si l'astéroïde est détruit (si sa résistance est égale à 0)
    pub fn est_détruit(&self) -> bool {
        self.resistance == 0
    }

    /// Met à jour la position de l'astéroide en fonction de sa vitesse.
    pub fn move_object(&mut self) -> Vec2 {
        self.position += self.speed;
        self.position = Self::bound_pos(self.position);
        self.position
    }

    /// Applique une nouvelle vitesse à l'astéroïde (par exemple après une collision avec le vaisseau)
    pub fn nouvelle_vitesse(&mut self, nv_vitesse: Vec2) {
        self.speed = nv_vitesse;
    }

    /// Applique une résistance pour empêcher que la vitesse de l'astéroïde descende en dessous de sa vitesse minimale.
    /// Après une collision avec le vaisseau, l'astéroïde rebondi dessus en prenant de la vitesse,
    /// Cette méthode ajoute une friction pour que l'astéroïde reprenne sa vitesse initiale quelques frames après l'impact.
    pub fn appliquer_resistance(&mut self) {
        let effet_friction = 0.98;

        if self.speed.length() > self.speed_min.length() {
            self.speed *= effet_friction;

            if self.speed.length() < self.speed_min.length() {
                self.speed = self.speed;
            }
        }
    }

    /// Méthode qui retourne le rayon de l'astéroïde en fonction de sa taille.
    pub fn rayon_asteroid(&self) -> f32 {
        match self.taille {
            1 => Self::ASTEROID_INIT_SIZE / 2.0, //Petit
            2 => Self::ASTEROID_INIT_SIZE,       //Moyen
            3 => Self::ASTEROID_INIT_SIZE * 1.5, //Grand
            _ => Self::ASTEROID_INIT_SIZE,       //Par défaut : moyen
        }
    }

    /// Fait rebondir l'astéroïde lorqu'il entre en collision avec un autre objet.
    pub fn rebondir(&mut self, collision_direction: Vec2) {
        // Variable qui stocke la normale par rapport à l'endroit de la collision
        let normale = collision_direction.normalize();
        // L'astéroïde rebondi alors en prenant de la vitesse dans le sens inverse à la collision.
        self.speed = self.speed - 2.0 * self.speed.dot(normale) * normale;
    }

    /// Génère une position aléatoire près de l'un des bords.
    fn new_alea_pos() -> Vec2 {
        let mut rng = thread_rng();

        let nearpos: f32 = rng.gen_range(Self::ASTEROID_INIT_SIZE / 2.0..=Self::ASTEROID_INIT_SIZE);
        let nearside = rng.gen_range(1..=4); // 1 = top, 2 = right, 3 = down, 4 = left
        let xpos: f32 = match nearside {
            2 => screen_width() - nearpos,
            4 => nearpos,
            _ => rng.gen_range(0.0..=screen_width()),
        };
        let ypos: f32 = match nearside {
            1 => nearpos,
            3 => screen_height() - nearpos,
            _ => rng.gen_range(0.0..=screen_height()),
        };
        vec2(xpos, ypos)
    }

    /// Génère une vitesse aléatoire pour l'astéroïde.
    fn new_alea_speed() -> Vec2 {
        let mut rng = thread_rng();

        let angle: f32 = rng.gen_range(0.0..=(2.0 * PI));
        Vec2::from_angle(angle)
    }

    /// Assure que l'astéroïde reste à l'écran en rebouclant sa position.
    fn bound_pos(mut pos: Vec2) -> Vec2 {
        pos.x = Self::bound_to(pos.x, screen_width());
        pos.y = Self::bound_to(pos.y, screen_height());
        pos
    }

    /// Assure que les cordonnées restent dans les limites de l'écran.
    fn bound_to(coord: f32, max: f32) -> f32 {
        if coord < 0.0 {
            max - coord
        } else if coord > max {
            coord - max
        } else {
            coord
        }
    }
}

/// Fonction qui permet de créer 2 nouveaux astéroïdes de taille inférieure après la destruction d'un astéroIde de taille 2 ou 3.
/// Pos1 sera la position d'un astéroïde.
/// Pos2 sera la position du deuxième astéroïde.
pub fn positions_asteroides_apres_collision(missile_pos: Vec2, asteroid_pos: Vec2) -> (Vec2, Vec2) {
    // Calculer la direction du missile par rapport à l'astéroïde
    let direction_missile = (asteroid_pos - missile_pos).normalize();

    // Générer les nouvelles positions en se décalant légèrement de part et d'autre de l'axe de la collision
    let offset_distance = 50.0; // Distance pour écarter les fragments
    let perpendiculaire = vec2(-direction_missile.y, direction_missile.x); // Vecteur perpendiculaire pour écarter les fragments

    let pos1 = asteroid_pos + perpendiculaire * offset_distance;
    let pos2 = asteroid_pos - perpendiculaire * offset_distance;

    (pos1, pos2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_asteroid() {
        let asteroid = Asteroid {
            position: Vec2::new(0.0, 0.0), // Assure-toi de ne pas appeler de fonctions de macroquad ici.
            speed: Vec2::new(1.0, 1.0),
            speed_min: Vec2::new(1.0, 1.0),
            taille: 2,
            resistance: 3,
        };
        assert!(asteroid.get_taille() >= 1 && asteroid.get_taille() <= 3);
        assert!(asteroid.get_resistance() > 0);
    }

    #[test]
    fn test_diminuer_resistance() {
        let mut asteroid = Asteroid {
            position: Vec2::new(0.0, 0.0),
            speed: Vec2::new(1.0, 1.0),
            speed_min: Vec2::new(1.0, 1.0),
            taille: 2,
            resistance: 3,
        };
        let initial_resistance = asteroid.get_resistance();
        asteroid.diminuer_résistance();
        assert_eq!(asteroid.get_resistance(), initial_resistance - 1);
    }

    #[test]
    fn test_est_detruit() {
        let mut asteroid = Asteroid {
            position: Vec2::new(0.0, 0.0),
            speed: Vec2::new(0.0, 0.0),
            speed_min: Vec2::new(0.0, 0.0),
            taille: 1,
            resistance: 1,
        };
        asteroid.diminuer_résistance();
        assert!(asteroid.est_détruit());
    }
}
