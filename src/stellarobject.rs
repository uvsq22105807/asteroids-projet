//! Module `stellarobject`
//!
//! Ce module définit le trait `StellarObject`, qui est utilisé pour regrouper les comportements
//! communs à tous les objets du jeu capables de se déplacer dans l'espace, tels que les astéroïdes,
//! le vaisseau, et les missiles. Il permet d'éviter la redondance de code en définissant un ensemble
//! de méthodes que les objets stellaires doivent implémenter.
//!
//!
//! ## Méthodes
//!
//! - `get_position`: Renvoie la position actuelle de l'objet.
//! - `set_position`: Définit une nouvelle position pour l'objet.
//! - `get_vitesse`: Renvoie la vitesse actuelle de l'objet.
//! - `set_vitesse`: Définit une nouvelle vitesse pour l'objet.
//! - `maj_position`: Met à jour la position de l'objet en fonction de sa vitesse actuelle.
//! - `gestion_bords`: Gère la situation où l'objet dépasse les bords de l'écran pour le repositionner.

use macroquad::prelude::*;

/// Trait `StellarObject`
///
/// Ce trait définit les caractéristiques communes à tous les objets du jeu qui se déplacent,
/// comme les astéroïdes, le vaisseau et les missiles.
/// Les objets stellaires doivent avoir une position, une vitesse, et pouvoir se déplacer

pub trait StellarObject {
    /// Retourne la position actuelle de l'objet stellaire
    fn get_position(&self) -> Vec2;

    /// Définit une nouvelle position pour l'objet stellaire.
    fn set_position(&mut self, new_position: Vec2);

    /// Retourne la vitesse actuelle de l'objet stellaire.
    fn get_vitesse(&self) -> Vec2;

    /// Définit une nouvelle vitesse pour l'objet stellaire.
    fn set_vitesse(&mut self, new_vitesse: Vec2);

    /// Met à jour la position de l'objet stellaire en fonction de sa vitesse actuelle.
    /// Cette méthode permet de déplacer l'objet en fonction de sa vitesse.
    /// Elle additionne la position actuelle avec la vitesse pour calculer la nouvelle position.
    fn maj_position(&mut self) {
        let position_actuelle = self.get_position();
        let vitesse = self.get_vitesse();
        self.set_position(position_actuelle + vitesse);
    }

    /// Gère les bords de l'écran pour assurer que l'objet reste visible en le repositionnant.
    /// Si l'objet dépasse les limites de l'écran, cette méthode le replace de l'autre côté de l'écran.
    fn gestion_bords(&mut self) {
        let mut position = self.get_position();
        if position.x > screen_width() {
            position.x = 0.0;
        } else if position.x < 0.0 {
            position.x = screen_width();
        }
        if position.y > screen_height() {
            position.y = 0.0;
        } else if position.y < 0.0 {
            position.y = screen_height();
        }
        self.set_position(position);
    }
}
