//! Module représentant le bonus de bouclier dans le jeu Asteroids.
//! Ce fichier contient la structure `Bonus` et les méthodes associées permettant de gérer
//! l'apparition, l'affichage, la mise à jour, et la collision du bonus de bouclier avec le vaisseau.

use macroquad::prelude::*;

/// Structure représentant le Bonus qui s'affiche à l'écran et qui remet le bouclier à 100%.
/// Ce 'Bonus' a une position, un timer car il ne reste que quelques secondes à l'écran,
/// ainsi qu'un booléen qui permet de dire s'il est visible ou pas.
pub struct Bonus {
    position: Vec2,
    visible: bool,
    timer: f32, // Temps restant avant que le bonus disparaisse
}

impl Bonus {
    /// Méthode pour créer un nouveau bonus avec une position aléatoire.
    pub fn nouveau_bonus() -> Self {
        Self {
            position: vec2(
                rand::gen_range(50.0, screen_width() - 50.0), // Génération aléatoire de la position
                rand::gen_range(50.0, screen_height() - 50.0),
            ),
            visible: false,
            timer: 0.0,
        }
    }

    /// Dessine le bonus à l'écran uniquement s'il est visible.
    /// # Arguments:
    /// - 'texture': Utilise la texture (image bouclier) pour la dessiner.
    pub fn draw_bonus(&self, texture: &Texture2D) {
        if self.visible {
            draw_texture_ex(
                texture,
                self.position.x - 15.0, // -15.0 pour placer le début de l'image sur le périmètre du cercle et non pas au centre.
                self.position.y - 15.0, // -15.0 car le rayon est de 15.0px pour le bonus.
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(30.0, 30.0)), // Redimensionner l'image pour correspondre à un rayon de 15.0
                    ..Default::default()
                },
            );
        }
    }

    /// Met à jour le timer du bonus, gère son apparition aléatoire et sa disparition après un certain temps.
    ///
    /// # Arguments
    /// - `delta_time`: Le temps écoulé depuis la dernière mise à jour.
    /// - `bouclier`: Le niveau actuel du bouclier du vaisseau.
    pub fn update_bonus(&mut self, delta_time: f32, bouclier: u8) {
        // Si le bonus est visible la variable avec le timer va diminuer chaque seconde.
        // Dès qu'on atteint 0, le bonus devient invisible à nouveau
        if self.visible {
            self.timer -= delta_time;
            if self.timer <= 0.0 {
                self.visible = false; // Disparition après un certain temps
            }
        } else {
            // Générer aléatoirement l'apparition du bonus toutes les 60 secondes
            if rand::gen_range(0.0, 150.0) < delta_time * 5.0 {
                // Chance accrue si le bouclier est en dessous de 30%
                if bouclier < 30 {
                    if rand::gen_range(0.0, 1.0) < 0.2 {
                        // 20% de chances quand le bouclier est inférieur à 30%
                        self.position = vec2(
                            rand::gen_range(50.0, screen_width() - 50.0),
                            rand::gen_range(50.0, screen_height() - 50.0),
                        );
                        self.visible = true;
                        self.timer = rand::gen_range(10.0, 15.0); // Durée de 10-15 secondes
                    }
                } else {
                    if rand::gen_range(0, 10) == 0 {
                        // 10% de chance toutes les 60s.
                        self.position = vec2(
                            rand::gen_range(50.0, screen_width() - 50.0),
                            rand::gen_range(50.0, screen_height() - 50.0),
                        );
                        self.visible = true;
                        self.timer = rand::gen_range(5.0, 10.0); // Durée de 5-10 secondes
                    }
                }
            }
        }
    }

    /// Vérifie s'il y a collision entre le bonus et le vaisseau.
    ///
    /// # Arguments
    /// - `position_vaisseau`: La position actuelle du vaisseau.
    /// - `rayon_vaisseau`: Le rayon du vaisseau.
    ///
    /// # Retourne
    /// - `true` si une collision est détectée et que le bonus est collecté, `false` sinon.
    pub fn verifier_collision(&mut self, position_vaisseau: Vec2, rayon_vaisseau: f32) -> bool {
        if self.visible {
            // On calcule la distance entre le centre du vaisseau et le centre du bonus.
            let distance = position_vaisseau.distance(self.position);
            // Si la distance entre les deux centres est inférieur à la somme des deux rayons, il y a collision:
            // + 15.0 car c'est le rayon du bonus (ne change pas)
            if distance < rayon_vaisseau + 15.0 {
                self.visible = false; // Bonus collecté, donc il disparaît
                return true;
            }
        }
        false
    }
}
