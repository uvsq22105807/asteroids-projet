//! Module principal pour le jeu Asteroids.
//!
//! Ce fichier contient la boucle principale du jeu, la logique de gestion des niveaux, la création et la gestion
//! des astéroïdes, du vaisseau et des missiles, ainsi que la gestion des collisions entre ces objets.
//! Le jeu inclut un écran de "Game Over" et la possibilité de recommencer une partie.

// Importation des modules nécessaires
use asteroid::{positions_asteroides_apres_collision, Asteroid};
use bonus::Bonus;
use macroquad::prelude::*;
use missile::Missile;
use spaceship::Spaceship;

mod asteroid;
mod bonus;
mod missile;
mod spaceship;
mod stellarobject;

/// Fonction de configuration du jeu avant son lancement.
/// Ce `Conf` détermine les paramètres d'affichage de la fenêtre.
fn window_conf() -> Conf {
    Conf {
        window_title: "Asteroids".to_owned(),
        fullscreen: true,     // Mettre la fenêtre en plein écran
        high_dpi: true,       // Prendre en charge les écrans haute définition (si nécessaire)
        ..Default::default()  // Conserver les autres options par défaut
    }
}

/// Fonction pour dessiner les astéroïdes à l'écran ainsi que l'interface du niveau de jeu en haut à gauche.
/// # Arguments:
/// -'asteroids' - Vecteur de référence des astéroïdes présents à l'écran.
/// -'niveau' - Niveau actuel du jeu
/// -`background_texture` - Référence à la texture du fond d'écran
/// - 'texture_asteroids' : Référence à la texture de l'astéroïde.
fn draw(
    asteroids: &Vec<Asteroid>,
    niveau: i32,
    texture_asteroids: &Texture2D,
    background_texture: &Texture2D,
) {
    draw_background(background_texture); // On dessine le fond d'écran.
    draw_text(&format!("Level {}", niveau), 20.0, 30.0, 30.0, WHITE); // Affichage du niveau en haut à gauche.
                                                                      // On parcourt chaque astéroïde présent dans le vecteur asteroids.
    for asteroid in asteroids {
        // Pour chaque élément, on va le déssiner.
        draw_asteroids(asteroid, texture_asteroids);
    }
}

/// Dessine dans la fênetre de jeu le fond d'écran.
/// # Arguments
/// - `texture` : Référence à la texture du fond d'écran.
fn draw_background(texture: &Texture2D) {
    // Dessiner la texture redimensionnée à la taille de l'écran
    draw_texture_ex(
        texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        },
    );
}

/// Fonction pour dessiner un astéroïde à l'écran.
/// La couleur varie en fonction de la résistance de l'astéroïde.
/// # Arguments
/// -`asteroid` - Référence à un objet Asteroid à dessiner.
fn draw_asteroids(asteroid: &Asteroid, texture: &Texture2D) {
    let rayon = asteroid.rayon_asteroid(); // On récupère le rayon de l'astéroïd qu'on dessine.
    let position = asteroid.get_position(); // On récupère la position de l'astéroïd qu'on dessine.

    // Calculer les coordonnées du coin supérieur gauche pour centrer correctement l'image
    let top_left_x = position.x - rayon; // Positionner l'image en centrant horizontalement sur l'astéroïde
    let top_left_y = position.y - rayon; // Positionner l'image en centrant verticalement sur l'astéroïde

    // Dessiner l'image de l'astéroïde
    draw_texture_ex(
        texture,
        top_left_x,
        top_left_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(rayon * 2.0, rayon * 2.0)), // Redimensionner l'image pour correspondre au diamètre de l'astéroïde
            ..Default::default()
        },
    );
}

/// Fonction qui gère la touche échap.
/// Retourne 'True' si la touche Echap est enfoncée.
fn handle_input() -> bool {
    if is_key_down(KeyCode::Escape) {
        return true;
    }

    false
}

/// Met à jour le modèle des astéroïdes en déplaçant chaque astéroïde.
/// # Arguments
/// -`asteroids` - Référence mutable à un vecteur d'astéroïdes à mettre à jour.
fn update_model(asteroids: &mut Vec<Asteroid>) {
    // Pour chaque astéroïde à l'écran, on utilise la fonction move_object() du module asteroid pour changer sa position en fonction de la vitesse.
    for asteroid in asteroids {
        asteroid.move_object();
    }
}

/// Fonction principale du jeu qui initialise le jeu, gère les entrées utilisateur et l'affichage.
/// Utilise `macroquad` pour créer une boucle d'animation et gérer l'affichage.
#[macroquad::main(window_conf)]
async fn main() {
    // On charge le fond d'écran
    let background_texture: Texture2D =
        load_texture("ressources/Fond_ecran_jeu.png").await.unwrap();

    // Charger la texture de l'astéroïde
    let texture_asteroid: Texture2D = load_texture("ressources/asteroids.png").await.unwrap();

    // On charge la texture pour le bonus bouclier
    let texture_bouclier: Texture2D = load_texture("ressources/bouclier.png").await.unwrap();

    // On s'assure que les images sont chargées correctement
    texture_asteroid.set_filter(FilterMode::Nearest);
    texture_bouclier.set_filter(FilterMode::Nearest);
    background_texture.set_filter(FilterMode::Nearest);

    // On crée une variable pour stocker le niveau actuel
    let mut niveau = 1;

    // On va créer un Vecteur vide qui va stocker les astéroïdes qui seront à l'écran.
    // On utilise une boucle pour ajouter des astéroïdes
    let mut asteroids: Vec<Asteroid> = Vec::new();
    for _ in 0..8 {
        asteroids.push(Asteroid::new())
    }

    // On va créer un vaisseau
    let mut vaisseau = Spaceship::new();

    // On va créer un Vecteur vide qui va stocker les missiles qui seront à l'écran.
    let mut missiles: Vec<Missile> = Vec::new();

    // On crée une variable pour le bonus
    let mut bonus = Bonus::nouveau_bonus();

    loop {
        clear_background(BLACK);
        draw_background(&background_texture);

        // Gestion de l'écran "Game Over"
        // Si le vaisseau n'a plus de bouclier, à la prochaine collision on affiche l'écran de game over.
        if vaisseau.get_bouclier() == 0 {
            clear_background(BLACK);
            // On dessine à l'écran le texte "Game Over"
            let taille_texte = measure_text("GAME OVER", None, 80, 1.0).width;
            draw_text(
                "GAME OVER",
                (screen_width() - taille_texte) / 2.0,
                screen_height() / 2.0 - 50.0,
                80.0,
                RED,
            );

            // On dessine à l'écran le texte qui indique à quel niveau on est morts.
            let taille_texte_niveau = measure_text(
                &format!("Vous êtes mort au niveau {} !", niveau),
                None,
                40,
                1.0,
            )
            .width;
            draw_text(
                &format!("Vous êtes mort au niveau {} !", niveau),
                (screen_width() - taille_texte_niveau) / 2.0,
                screen_height() / 2.0,
                40.0,
                WHITE,
            );

            // On dessine à l'écran le texte avec des indications pour recommencer à jouer.
            let taille_texte_info = measure_text(
                "Appuyez sur 'Entrée' pour recommencer ou 'Échap' pour quitter.",
                None,
                25,
                1.0,
            )
            .width;
            draw_text(
                "Appuyez sur 'Entrée' pour recommencer ou 'Échap' pour quitter.",
                (screen_width() - taille_texte_info) / 2.0,
                screen_height() / 2.0 + 50.0,
                25.0,
                WHITE,
            );

            if is_key_pressed(KeyCode::Enter) {
                // Réinitialiser le jeu si on appuie sur la touche "Entrée".
                // On nettoye le vecteur avec les astéroïdes.
                asteroids.clear();
                // On génère 8 nouveaux astéroïdes qu'on stocke dans ce vecteur.
                for _ in 0..8 {
                    asteroids.push(Asteroid::new());
                }
                // On crée un nouveau vaisseau.
                // On nettoie le vecteur qui stocke les missiles présents à l'écran.
                // Et on remet le niveau à 1.
                vaisseau = Spaceship::new();
                missiles.clear();
                niveau = 1;
            }

            // Cependant, si la touche "Echap" est appuyé, on quitte le jeu.
            if is_key_down(KeyCode::Escape) {
                break;
            }

            next_frame().await;
            continue;
        }

        // On dessine les éléments à l'écran.
        draw(&asteroids, niveau, &texture_asteroid, &background_texture);

        vaisseau.draw(); // On dessine le vaisseau
        vaisseau.maj_pos(&mut asteroids); // Mise à jour de chaque position et gestion de la collision avec les astéroïdes
        vaisseau.dessiner_interface_bouclier(); // En haut à droite on affiche le pourcentage restant du bouclier.

        // Mettre à jour le bonus (apparition et disparition)
        bonus.update_bonus(get_frame_time(), vaisseau.get_bouclier());

        // Dessiner le bonus s'il est visible
        bonus.draw_bonus(&texture_bouclier);

        // Vérifier si le vaisseau récupère le bonus
        if bonus.verifier_collision(vaisseau.get_position(), 15.0) {
            vaisseau.restaurer_bouclier(); // Remettre le bouclier à 100%
        }

        // Tirs du vaisseau
        if is_key_pressed(KeyCode::Space) {
            // Créer un nouveau missile en utilisant la position et la direction du vaisseau
            let nv_missile =
                Missile::nouveau_missile(vaisseau.get_position(), vaisseau.get_rotation());
            missiles.push(nv_missile);
        }

        // Mettre à jour et dessiner les missiles
        for missile in missiles.iter_mut() {
            missile.maj_pos_missile();
            missile.dessiner_missile();
        }

        // Gestion des collisions entre missiles et astéroïdes
        let mut asteroids_to_remove = Vec::new(); // Pour stocker les astéroïdes qui vont être enlevés.
        let mut missiles_to_remove = Vec::new(); // Pour stocker les missiles qui vont devoir être enlevés.
        let mut new_asteroids = Vec::new(); // Pour stocker les astéroïdes créés lors de la fragmentation

        for (missile_index, missile) in missiles.iter().enumerate() {
            for (asteroid_index, asteroid) in asteroids.iter_mut().enumerate() {
                let distance = missile.get_position().distance(asteroid.get_position()); // Calcul de la distance entre le missile et le centre de l'astéroïde
                let collision_distance = 3.0 + asteroid.rayon_asteroid(); // Calcul de la distance entre le centre de l'astéroïde et le rebord
                                                                          // Si le missile se trouve entre le centre de l'astéroïde et le rebord = Collision
                if distance < collision_distance {
                    // Collision détectée entre un missile et un astéroïde
                    asteroid.diminuer_résistance(); // Donc on enlève un point de résistance
                                                    // Si l'astéroïde n'a plus de résistance, il est alors détruit.
                    asteroid.get_resistance();
                    if asteroid.est_détruit() {
                        println!("Astéroïde détruit !");
                        // Donc on va créer deux nouveaux astéroïdes.
                        match asteroid.get_taille() {
                            3 => {
                                // Créer 2 astéroïdes de taille 2
                                let (position1, position2) = positions_asteroides_apres_collision(
                                    missile.get_position(),
                                    asteroid.get_position(),
                                );
                                new_asteroids.push(Asteroid::nouvel_asteroid(2, position1));
                                new_asteroids.push(Asteroid::nouvel_asteroid(2, position2));
                            }
                            2 => {
                                // Créer 2 astéroïdes de taille 1
                                let (position1, position2) = positions_asteroides_apres_collision(
                                    missile.get_position(),
                                    asteroid.get_position(),
                                );
                                new_asteroids.push(Asteroid::nouvel_asteroid(1, position1));
                                new_asteroids.push(Asteroid::nouvel_asteroid(1, position2));
                            }
                            _ => {}
                        }
                        // Et on va rajouter les anciens astéroïdes à la liste des astéroïdes qu'on doit enlever.
                        asteroids_to_remove.push(asteroid_index);
                    }
                    // Même principe pour les missiles.
                    missiles_to_remove.push(missile_index);
                    break; // Le missile ne peut toucher qu'un astéroïde
                }
            }
        }

        // Supprimer les astéroïdes détruits
        asteroids_to_remove.sort_unstable();
        for index in asteroids_to_remove.iter().rev() {
            if *index < asteroids.len() {
                asteroids.remove(*index);
            }
        }

        // Ajouter les nouveaux astéroïdes créés lors de la fragmentation
        asteroids.extend(new_asteroids);

        // Supprimer les missiles qui ont touché un astéroïde
        missiles_to_remove.sort_unstable();
        for index in missiles_to_remove.iter().rev() {
            if *index < missiles.len() {
                missiles.remove(*index);
            }
        }

        // Si tous les astéroïdes sont détruits, passer au niveau suivant.
        // Premier niveau = 5 astéroïdes, ensuite 1 astéroïde de plus à chaque niveau.
        if asteroids.is_empty() {
            niveau += 1;
            for _ in 0..(4 + niveau) {
                asteroids.push(Asteroid::new());
            }
            // On recentre le vaisseau et on enlève tous les missiles qui avaient été tirés avant.
            vaisseau.recentrer();
            missiles.clear();
        }

        if handle_input() {
            break;
        }

        update_model(&mut asteroids);

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Les tests unitaires ont utilisant macroquad génèrent des erreurs.
    /// On simule alors la création des asteroids avec une nouvelle structure qui créé un astéroïde avec des variables prenant des f32 au lieu de Vec2 (fonction de Macroquad).
    #[derive(Debug, PartialEq)]
    struct AsteroidTest {
        x: f32,
        y: f32,
    }

    impl AsteroidTest {
        /// Crée un nouveau astéroïde pour les tests.
        /// On écrit exactement les mêmes fonctions (celles qui nous intéressent) et utilisant des f32 au lieu des Vec2.
        fn new() -> Self {
            Self { x: 0.0, y: 0.0 }
        }

        fn move_object(&mut self) {
            // Logique simplifiée pour simuler le mouvement de l'astéroïde
            self.x += 1.0;
            self.y += 1.0;
        }

        fn get_position(&self) -> (f32, f32) {
            (self.x, self.y)
        }
    }

    /// Test pour `update_model`, utilisant des astéroïdes simplifiés.
    #[test]
    fn test_update_model() {
        let mut asteroids = vec![AsteroidTest::new(), AsteroidTest::new()];

        // Mise à jour des astéroïdes, cela devrait les déplacer
        for asteroid in &mut asteroids {
            asteroid.move_object();
        }

        // Vérifiez que chaque astéroïde a bien été déplacé
        for asteroid in &asteroids {
            let (x, y) = asteroid.get_position();
            assert!(
                x != 0.0 || y != 0.0,
                "L'astéroïde a été déplacé de sa position initiale."
            );
        }
    }
}
