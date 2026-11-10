# Rust particles

Demo 2D SDL2 du mecanisme decrit dans
[Advanced Character Physics, Thomas Jakobsen](https://www.cs.cmu.edu/afs/cs/academic/class/15462-s13/www/lec_slides/Jakobsen.pdf).

## Lancement

Installer Rust et la bibliotheque de developpement SDL2 (`libsdl2-dev` sur
Debian/Ubuntu), puis executer `cargo run`.

Une corde suspendue, un drapeau en tissu et une particule libre apparaissent
au lancement. Les segments bleus representent les contraintes de distance
et les points rouges sont les ancrages immobiles. `R` ajoute une corde,
`F` ajoute un drapeau, `N` ajoute une particule libre, `Echap` ferme la fenetre.
`E` declenche une explosion a une position aleatoire dans le bas de l'ecran,
avec un rayon de 300 a 450 pixels et une impulsion de 400 a 700,
pour deplacer aussi les particules les plus lourdes.
Un cercle orange indique temporairement l'expansion de l'explosion.
L'impulsion radiale est appliquee immediatement aux centres des particules
dans ce rayon, decroit lineairement avec la distance et est divisee par la masse.
Les noeuds du tissu et de la corde sont affectes, mais les ancrages restent fixes.
Chaque particule libre recoit une couleur aleatoire a sa creation et la conserve.
Les ancrages restent rouges et les noeuds de corde et de tissu restent noirs.

Le drapeau est un maillage de 13 colonnes et 9 lignes, fixe uniquement aux
coins inferieurs gauche et droit. Sa partie superieure retombe sous la gravite.
`World::create_flag` construit une grille avec des contraintes horizontales
et verticales uniquement. Chaque lien limite la distance maximale sans imposer
de longueur minimale : les noeuds peuvent se rapprocher et le tissu retomber
sous les attaches. Les collisions des disques empechent leur superposition.
Sans diagonales, les cellules peuvent aussi se deformer en cisaillement.
La corde conserve ses contraintes de longueur constante.
Le rendu montre le maillage, sans surface pleine. Ses noeuds participent aux
collisions comme ceux de la corde ; les faces et les segments ne collisionnent pas.

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
Les particules sont des disques pleins : rayon de 3 pixels par defaut,
personnalisable avec `Particle::new(...).with_radius(rayon)`. Les particules
libres de la demo ont un rayon aleatoire entre 5 et 60 pixels, au lancement
comme avec `N`, pour obtenir des disques plus grands que les mailles du tissu.
Leur masse est proportionnelle a leur surface (densite constante en 2D) :
`masse = 10 * (rayon / 12)^2`. Les grosses particules deplacent donc davantage
les autres particules et le tissu lors des contacts. Leur vitesse initiale
et leur acceleration gravitationnelle restent identiques.
Les noeuds de corde ont un rayon
de 3 pixels au maximum, reduit si necessaire a la moitie de la longueur d'un
segment pour ne pas contredire les contraintes.

A chaque iteration, les paires de disques en chevauchement sont separees
jusqu'a la somme de leurs rayons, selon leurs masses inverses, puis les bords
sont appliques en tenant compte du rayon. Les ancrages ne bougent pas.
Les contraintes comparent d'abord les distances au carre : une distance deja
valide ne demande pas de racine carree, sauf aux echelles numeriques extremes.
Cette projection de contact utilise la vitesse implicite de Verlet ; les
coefficients de restitution et de frottement s'appliquent seulement aux bords.
Les particules libres ont une restitution de 0.85 pour des rebonds marques.
La detection examine toutes les paires (cout quadratique).
Comme les contraintes de corde, les contacts sont resolus iterativement :
un petit chevauchement numerique peut subsister dans les configurations denses.
Deux ancrages superposes ne peuvent pas etre separes.
Les collisions sont discretes : des particules tres rapides peuvent se traverser
entre deux pas. Les noeuds de corde se heurtent, mais pas les segments eux-memes.

`cargo test` execute les tests de l'integration, des contraintes et des collisions,
des disques et de leurs chevauchements, ainsi qu'une simulation prolongee de la corde.
