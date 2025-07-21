//! Module pour gérer les missiles dans le jeu.
//! Ce module contient la structure `Missile` et les méthodes associées pour créer et manipuler les missiles.

use macroquad::prelude::*;

/// Structure représentant un missile tiré par un vaisseau.
/// Un 'Missile' a une position, une direction et une vitesse.
pub struct Missile {
    position: Vec2, // Position actuelle du missile
    vitesse: Vec2,  // Vecteur vitesse du missile
}

impl Missile {
    /// Méthode pour créer un nouveau missile en utilisant la position du vaisseau et sa direction.
    /// # Paramètres:
    ///     - position: position initiale du missile quand il est tiré.
    ///     - rotation: angle de rotation du vaisseau lors du tir
    /// # Retourne un nouvel objet 'Missile'.
    pub fn nouveau_missile(position: Vec2, direction: f32) -> Self {
        let vitesse = vec2(direction.cos(), direction.sin()) * 5.0; // Vitesse de base d'un missile
        Self { position, vitesse } // Renvoie un element missile avec une positon et une vitesse (en fonction de la direction du vaisseau)
    }

    /// Méthode pour mettre à jour la position du missile en foction de sa vitesse.
    /// Cette méthode est appelée à chaque image pour déplacer le missile, qui avance en ligne droite.
    pub fn maj_pos_missile(&mut self) {
        self.position += self.vitesse;
    }

    /// Méthode pour obtenir la position actuelle du missile
    /// # Retourne un vecteur avec les positions x,y du missile
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Dessine le missile à l'écran.
    /// Utilise la fonction draw_circle de 'macroquad' pour dessiner un cercle rouge représentant le missile.
    /// Cette méthode est appellée à chaque frame pour afichier le missile à sa nouvelle position.
    pub fn dessiner_missile(&self) {
        draw_circle(self.position.x, self.position.y, 2.0, RED);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_missile() {
        let missile = Missile::nouveau_missile(vec2(10.0, 20.0), 0.0);
        assert_eq!(missile.get_position(), vec2(10.0, 20.0));
    }

    #[test]
    fn test_mouvement_missile() {
        let mut missile = Missile::nouveau_missile(vec2(0.0, 0.0), 0.0);
        missile.maj_pos_missile();
        assert_eq!(missile.get_position(), vec2(5.0, 0.0));
    }

    #[test]
    fn test_direction_missile() {
        let missile = Missile::nouveau_missile(vec2(0.0, 0.0), 0.0); // Angle de rotation 0 (vers la droite)
        assert_eq!(missile.get_position(), vec2(0.0, 0.0));
        let mut missile_moving = missile;
        missile_moving.maj_pos_missile();
        assert_eq!(missile_moving.get_position(), vec2(5.0, 0.0)); // Vérifie qu'il avance dans la bonne direction
    }

    #[test]
    fn test_mouvement_apres_plusieurs_frames() {
        let mut missile = Missile::nouveau_missile(vec2(0.0, 0.0), 0.0);
        missile.maj_pos_missile(); // 1ère mise à jour
        missile.maj_pos_missile(); // 2ème mise à jour
        assert_eq!(missile.get_position(), vec2(10.0, 0.0)); // Vérifie que le missile a bien avancé de 10 unités (5.0 par mise à jour)
    }
}
