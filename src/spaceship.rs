//! Module pour gérer le vaisseau spatial dans le jeu.
//! Ce module contient la structure `Spaceship` ainsi que les méthodes associées pour créer, déplacer,
//! et manipuler le vaisseau dans le jeu. Le vaisseau est capable de se déplacer dans toutes les directions,
//! de tourner, de ralentir, et de gérer les collisions avec les astéroïdes.
//! Il possède également un bouclier qui peut être restauré avec des bonus.

use crate::asteroid::Asteroid;
use macroquad::prelude::*;

/// Structure qui représente le vaisseau spatial.
/// Un vaisseau a une position (x,y) une vitesse de déplacement, l'angle dans lequel il est orienté (vers où il se dirige) et il a un bouclier.
pub struct Spaceship {
    position: Vec2, // Vecteur 2 dimensions qui représente la position du vaisseau: (x,y)
    vitesse: Vec2, // Vecteur 2 dimensions qui représente sa vitesse: (1.0, 0.0) = il va vers la doite par ex.
    rotation: f32, // Angle de rotation
    bouclier: u8,  // Pourcentage bouclier
    cooldown: f64, // Cooldown pour empêcher les collisions multiples
}

impl Spaceship {
    /// Crée un nouveau vaisseau immobile initialisé au centre de l'écran, orienté vers le haut et avec son bouclier à 100%.
    pub fn new() -> Self {
        Self {
            position: vec2(screen_width() / 2.0, screen_height() / 2.0), // Au départ il est centré sur l'écran
            rotation: 0.0,           // Au départ il est orienté vers le haut
            vitesse: vec2(0.0, 0.0), // Au départ le vaisseau est immobile
            bouclier: 100,           // Bouclier au maximum (100%)
            cooldown: 0.0,
        }
    }

    /// Méthode pour obtenir la position actuelle du vaisseau.
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Méthode pour obtenir le pourcentage de bouclier du vaisseau.
    pub fn get_bouclier(&self) -> u8 {
        self.bouclier
    }

    /// Retourne l'angle de rotation actuel du vaisseau.
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    /// Recentre le vaisseau au milieu de l'écran et réinitialise sa vitesse.
    pub fn recentrer(&mut self) {
        self.position = vec2(screen_width() / 2.0, screen_height() / 2.0);
        self.vitesse = vec2(0.0, 0.0); // Réinitialise aussi la vitesse du vaisseau
    }

    /// Fonction pour réstaurer le bouclier à 100% (Bonus).
    pub fn restaurer_bouclier(&mut self) {
        self.bouclier = 100
    }

    /// Méthode pour dessiner le vaisseau à l'écran avec un triangle, représentant le vaisseau, entouré par un cercle qui représente son bouclier.
    pub fn draw(&self) {
        // Dessine un cercle, son point central c'est les cordonnées x et y du vaisseau.
        // Son rayon est de 15px, son épaisseur est de 3px et il est vert
        draw_circle_lines(self.position.x, self.position.y, 15.0, 3.0, GREEN);

        // Maintenant pour le triangle qui représente le vaisseau.
        // Dessine un triangle à l'intérieur du cercle pour représenter un vaisseau entouré de son bouclier.

        // Sommet du triangle
        let point1 = vec2(
            self.position.x + self.rotation.cos() * 15.0,
            self.position.y + self.rotation.sin() * 15.0,
        );

        // Points pour dessiner la base du triangle
        // Cercle trigonométrique, on place les points sur 4PI/5 pour la base du triangle.
        let decale_angle = 3.1415 * 4.0 / 5.0;
        let point2 = vec2(
            self.position.x + (self.rotation + decale_angle).cos() * 15.0,
            self.position.y + (self.rotation + decale_angle).sin() * 15.0,
        );

        let point3 = vec2(
            self.position.x + (self.rotation - decale_angle).cos() * 15.0,
            self.position.y + (self.rotation - decale_angle).sin() * 15.0,
        );

        // Dessiner le triangle en connectant les points qu'on vient de créer.
        draw_line(point1.x, point1.y, point2.x, point2.y, 3.0, GRAY);
        draw_line(point2.x, point2.y, point3.x, point3.y, 3.0, GRAY);
        draw_line(point3.x, point3.y, point1.x, point1.y, 3.0, GRAY);
    }

    /// Met à jour la position du vaisseau en fonction des entrées utilisateur.
    /// Cette fonction gère également les collisions avec les astéroïdes.
    /// # Arguments:
    /// - Référence mutable au vecteur qui contient les astéroïdes présents à l'écran.
    /// - L'objet vaisseau mutable car on va changer son positionnement en fonction des touches, etc...
    pub fn maj_pos(&mut self, asteroids: &mut Vec<Asteroid>) {
        // Rotation avec les touches droite et gauche:
        if is_key_down(KeyCode::Left) {
            self.rotation -= 0.05; // Tourne à gauche
        }

        if is_key_down(KeyCode::Right) {
            self.rotation += 0.05; // Tourne à droite
        }

        // Accélération avec la touche "Haut"
        if is_key_down(KeyCode::Up) {
            let accel = vec2(self.rotation.cos(), self.rotation.sin()) * 0.2;
            self.vitesse += accel
        }

        // Rétro-poussée avec la touche "Bas"
        if is_key_down(KeyCode::Down) {
            let accel = vec2(self.rotation.cos(), self.rotation.sin()) * 0.2;
            self.vitesse -= accel
        }

        // Pour eviter qu'on puisse prendre une vitesse infinie, on va rajouter un effet de friction pour que le vaisseau ralentisse.
        self.vitesse *= 0.97;

        // Mise à jour de la position, on ajoute la vitesse actuelle à la position
        self.position += self.vitesse;

        // Rebouclage si on sort de l'écran
        self.position = Self::bound_pos(self.position);

        // Detection des collisions avec les astéroïdes
        for asteroid in asteroids.iter_mut() {
            let distance = self.position.distance(asteroid.get_position());
            // Variable distance qui permet de calculer la distance entre un astéroïde et le vaisseau
            // self.position = position x, y du vaisseau
            // asteroid.get_position() = position x, y de l'astéroïde avec la méthode get_position qu'on a créé.
            // .distance() méthode de Vec2 qui calcule la distance entre deux points (doc)

            let distance_collision = 15.0 + asteroid.rayon_asteroid();
            // Tout d'abord on calcule la distance entre le centre des deux objets avec la variable distance.
            // La vrai distance avant la collision n'est pas la distance jusqu'au centre de l'objet mais là où se trouve son périmètre
            // Le vaisseau a un rayon de 15, asteroid.rayon_asteroid prends le rayon de l'astéroïde.

            if distance < distance_collision {
                // Calculer le vecteur directionnel de collision
                let collision_direction = asteroid.get_position() - self.position;

                // Empêcher que le vaisseau rentre dans l'astéroïde
                let correction_vector =
                    collision_direction.normalize() * (distance_collision - distance);
                self.position -= correction_vector;

                // L'asteroid rebondi sur le vaisseau si vaisseau immobile.
                asteroid.rebondir(collision_direction);

                if get_time() - self.cooldown > 0.5 {
                    self.cooldown = get_time();
                    //println!("Collision détectée !");
                    self.bouclier = match asteroid.get_taille() {
                        1 => self.bouclier.saturating_sub(10),
                        2 => self.bouclier.saturating_sub(15),
                        3 => self.bouclier.saturating_sub(25),
                        _ => self.bouclier, // On ne change pas la valeur du bouclier.
                    };
                };

                // Ajouter une impulsion à l'astéroïde si le vaisseau a une vitesse suffisante
                if self.vitesse.length() > 0.1 {
                    let impulse_strength = 1.2; // Facteur d'impulsion
                    let impulse =
                        collision_direction.normalize() * self.vitesse.length() * impulse_strength;
                    asteroid.nouvelle_vitesse(impulse); // Appliquer l'impulsion temporaire
                }

                // Réduire la vitesse du vaisseau après la collision (optionnel)
                self.vitesse *= 0.5;
            }

            // Assurez-vous que l'astéroïde ne descend pas en dessous de sa vitesse minimale
            asteroid.appliquer_resistance();
        }
    }

    /// Limite la position du vaisseau pour qu'il reste sur l'écran. (Même principe que pour les astéroïdes)
    fn bound_pos(mut pos: Vec2) -> Vec2 {
        pos.x = Self::bound_to(pos.x, screen_width());
        pos.y = Self::bound_to(pos.y, screen_height());
        pos
    }

    /// Limite une coordonnée à une valeur maximale pour assurer que les cordonnées restent dans les limites de l'écran.
    fn bound_to(coord: f32, max: f32) -> f32 {
        if coord < 0.0 {
            max - coord
        } else if coord > max {
            coord - max
        } else {
            coord
        }
    }

    /// Affichage de l'interface du boulier en haut à droite.
    pub fn dessiner_interface_bouclier(&self) {
        let largeur_barre_bouclier = 199.0 * (self.bouclier as f32 / 100.0);
        // On dessine un premier rectangle blanc = Fond de la barre
        draw_rectangle(screen_width() - 220.0, 20.0, 200.0, 10.0, WHITE);

        // On dessine maintenant un rectangle vert qui represente le pourcentage:
        draw_rectangle(
            screen_width() - 219.0,
            21.0,
            largeur_barre_bouclier,
            8.0,
            GREEN,
        );

        // Finalement on écrit un petit texte avec les informations
        draw_text(
            &format!("Bouclier: {}%", self.bouclier),
            screen_width() - 220.0,
            52.0,
            20.0,
            WHITE,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Les tests unitaires ont utilisant macroquad génèrent des erreurs.
    /// On simule alors la création du vaisseau  avec une nouvelle structure qui créé un vaisseau avec des variables prenant des f32 au lieu de Vec2 (fonction de Macroquad).
    #[derive(Debug, PartialEq)]
    struct SpaceshipTest {
        x: f32,
        y: f32,
        vitesse_x: f32,
        vitesse_y: f32,
        bouclier: u8,
    }

    impl SpaceshipTest {
        /// Crée un nouveau vaisseau pour les tests.
        /// On écrit exactement les mêmes fonctions (celles qui nous intéressent) et utilisant des f32 au lieu des Vec2.
        fn new() -> Self {
            Self {
                x: 400.0, // Centre fictif de l'écran (largeur)
                y: 300.0, // Centre fictif de l'écran (hauteur)
                vitesse_x: 0.0,
                vitesse_y: 0.0,
                bouclier: 100,
            }
        }

        /// Restaure le bouclier à 100%.
        fn restaurer_bouclier(&mut self) {
            self.bouclier = 100;
        }

        /// Recentre le vaisseau au milieu de l'écran et réinitialise sa vitesse.
        fn recentrer(&mut self) {
            self.x = 400.0; // Centre fictif de l'écran (largeur)
            self.y = 300.0; // Centre fictif de l'écran (hauteur)
            self.vitesse_x = 0.0; // Le vaisseau est à l'arrêt.
            self.vitesse_y = 0.0;
        }
    }

    #[test]
    fn test_creation_vaisseau() {
        let vaisseau = SpaceshipTest::new(); // Nouveau vaisseau.
        assert_eq!(vaisseau.bouclier, 100); // On vérifie que le bouclier est bien à 100%.
        assert_eq!(vaisseau.x, 400.0); // Et qu'il se trouve bien au centre de l'écran en x.
        assert_eq!(vaisseau.y, 300.0); // Et qu'il se trouve bien au centre de l'écran en y.
    }

    #[test]
    fn test_restaurer_bouclier() {
        let mut vaisseau = SpaceshipTest::new(); // Nouveau Vaisseau.
        vaisseau.bouclier = 50; // Son bouclier est à 50%.
        vaisseau.restaurer_bouclier(); // On utilise notre fonction pour réstaurer le bouclier.
        assert_eq!(vaisseau.bouclier, 100); // On vérifie que le bouclier est bien à 100%.
    }

    #[test]
    fn test_recentrer_vaisseau() {
        let mut vaisseau = SpaceshipTest::new();
        vaisseau.x = 100.0; // On met le vaisseau à un autre endroit que le centre en x.
        vaisseau.y = 100.0; // On met le vaisseau à un autre endroit que le centre en y.
        vaisseau.vitesse_x = 1.0; // On lui donne une vitesse pour montrer qu'il n'est pas à l'arrêt.
        vaisseau.vitesse_y = 1.0; // On lui donne une vitesse pour montrer qu'il n'est pas à l'arrêt.
        vaisseau.recentrer(); // On utilise notre méthode pour recenter le vaisseau.
        assert_eq!(vaisseau.x, 400.0); // On vérifie que les cordonnées x sont bien au centre que l'on a défini.
        assert_eq!(vaisseau.y, 300.0); // On vérifie que les cordonnées y sont bien au centre que l'on a défini.
        assert_eq!(vaisseau.vitesse_x, 0.0); // On vérifie également que la vitesse est nulle en x.
        assert_eq!(vaisseau.vitesse_y, 0.0); // On vérifie également que la vitesse est nulle en y.
    }
}
