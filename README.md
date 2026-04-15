# ReptileSim (Rust / RISC-V Linux)

Dashboard local type "ThermoOS" pour carte **Milk-V Jupiter NX** (SpacemiT K1/M1) sous Linux.

## Hypothèses (matériel)

Ce dépôt implémente une **UI de supervision locale** et une simulation capteurs/actionneurs.
Il **n'implémente pas** d'accès GPIO/I2C/UART réel faute de pinout + schéma exact de votre baseboard.

### Vérifications à faire côté carte

1. Vérifier l'architecture:
   ```bash
   uname -m
   ```
   Attendu: `riscv64`
2. Vérifier accélération graphique et backend:
   ```bash
   echo $XDG_SESSION_TYPE
   glxinfo | head
   ```
3. Vérifier présence des devices réels (si vous branchez capteurs):
   ```bash
   ls /dev/i2c-*
   ls /dev/ttyS* /dev/ttyUSB* 2>/dev/null
   ```

## Build (hôte Linux)

```bash
cargo check
cargo run
```

Avec image locale d'aperçu (JPEG/PNG) :

```bash
REPTILE_PREVIEW_PATH=/chemin/animal.jpg cargo run
```

## Cross-build cible RISC-V

```bash
rustup target add riscv64gc-unknown-linux-gnu
cargo build --release --target riscv64gc-unknown-linux-gnu
```

Binaire produit:
`target/riscv64gc-unknown-linux-gnu/release/reptile-sim`

## "Done" (critères)

- L'application démarre sans panic.
- En-tête `ThermoOS` visible.
- Le panneau `APERÇU ANIMAL` affiche l'image si `REPTILE_PREVIEW_PATH` est défini vers un fichier lisible, sinon affiche un message d'aide.
- 4 cartes de zones visibles (chaude/intermédiaire/humide/bassin).
- Chaque carte affiche une barre d'accent colorée à gauche, les mesures en grand, et les tags ÉCLAIRAGE/UVA/UVB/6500K/CHAUF/POMPE en pastilles ON/OFF.
- Les valeurs temp/hygro varient automatiquement toutes ~1.2 s.
- État ventilation passe ON automatiquement si CO₂ simulé dépasse le seuil.
- Panneau droit affiche les champs reptile étendus (naissance, poids, taille, etc.).

## Rollback

```bash
git reset --hard HEAD~1
```
