# Rathia 🎵
> **Rhythia** pour le terminal avec **Ratatui** et rendu 3D en **Braille**.

Rathia est une réimplémentation terminale haute performance du jeu de rythme **Rhythia** (Sound Space). Conçu pour le terminal moderne avec rendu pseudo-3D fil-de-fer en caractères Braille haute résolution, gestion de la souris en mode hover-to-hit (auto-hit) et audio haute fidélité synchronisé par échantillon.

---

## ✨ Fonctionnalités

- 🎮 **Contrôles à la souris naturels** : Capture directe des mouvements de souris dans le terminal (`crossterm`) pour viser les notes en "hover-to-hit" sans clic, exactement comme dans Rhythia.
- 🌌 **Moteur de rendu 3D Braille** : Projection en perspective du tunnel, des lignes de fuite, des notes animées et du réticule avec le widget `Canvas` de Ratatui (`Marker::Braille`).
- 🔊 **Audio haute fidélité (Kira)** : Précision au sample audio sans dérive temporelle pour le playback des morceaux (MP3/OGG) et les effets sonores de hit/fail.
- 📦 **Support multi-formats de maps** :
  - `.sspm` (v1 et v2)
  - `.phxm` (archives Zip Rhythia)
  - `.txt` (format CSV legacy SSMapV1)
- 📊 **Système de score et de rang complet** : Calcul dynamique de vie (escalade sur miss), combo, accuracy %, rangs (SS, S, A, B, C, D, F).
- 🎛️ **Menus complets** : Menu principal cyberpunk, sélection des morceaux par difficulté/catégorie avec aperçu, menu pause et écran des scores.

---

## 🕹️ Commandes & Contrôles

### En Jeu
| Action | Touche / Contrôle |
|---|---|
| **Viser les notes** | Déplacer la souris dans la zone 3D |
| **Viser (clavier)** | `Flèches Directionnelles` |
| **Pause / Reprendre** | `P` ou `Échap` |
| **Recommencer** | `R` |
| **Quitter vers la sélection** | `Q` |

### Dans les Menus
| Action | Touche |
|---|---|
| **Naviguer** | `↑` / `↓` ou molette de la souris |
| **Sélectionner / Valider** | `Entrée` |
| **Retour** | `Échap` ou `Q` |

---

## 🚀 Lancement

Pour lancer Rathia :
```bash
cargo run --release
```
Ou directement avec le binaire compilé :
```bash
./target/release/rathia
```

Les maps sont automatiquement scannées et chargées depuis le dossier `./maps/`.
