# Rust particles

Demo 2D SDL2 du mecanisme decrit dans
[Advanced Character Physics, Thomas Jakobsen](https://www.cs.cmu.edu/afs/cs/academic/class/15462-s13/www/lec_slides/Jakobsen.pdf).

## Lancement

Installer Rust et la bibliotheque de developpement SDL2 (`libsdl2-dev` sur
Debian/Ubuntu), puis executer `cargo run`.

Une corde suspendue et une particule libre apparaissent au lancement.
Les segments bleus representent les contraintes de distance et le point rouge
est l'ancrage immobile. `R` ajoute une corde, `N` ajoute une particule libre,
`Echap` ferme la fenetre.

## Simulation

L'integration de Verlet conserve les positions courante et precedente :
`position += position - precedente + acceleration * dt^2`.
La physique avance a pas fixe de 1/60 s, independamment du rendu.
Les vitesses initiales sont exprimees en pixels par pas ; `World::gravity`
contient deja le deplacement `acceleration * dt^2` (0.1 pixel par pas au carre).

Chaque contrainte reference deux indices dans les particules du monde.
Le solveur projette leur distance dans l'intervalle autorise en repartissant
la correction selon les masses inverses, puis reprojette les particules dans
les limites de la fenetre. Une masse nulle ou negative indique un ancrage.
La corde utilise des contraintes de longueur constante entre voisins,
relaxees 80 fois par pas ; elle reste donc legerement extensible numeriquement.
Les collisions aux bords appliquent restitution et frottement.
Il n'y a pas de collision entre particules ni d'auto-collision de la corde.

`cargo test` execute les tests de l'integration, des contraintes et des collisions,
ainsi qu'une simulation prolongee de la corde.
