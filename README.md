# Snapback

Application de bureau qui redonne à vos souvenirs Snapchat **leur vraie date**
avant de les importer dans Photos (Apple) ou Google Photos.

Quand on exporte ses *Memories* depuis Snapchat, les photos perdent leur date :
une fois importées dans une galerie, elles apparaissent toutes à la date du
téléchargement au lieu de la date réelle. Cette app corrige ça automatiquement,
réincruste les textes/dessins (overlays) sur les snaps, supprime les doublons,
et vous rend un dossier propre, prêt à importer.

> **Tout se passe en local, sur votre ordinateur.** Aucun fichier n'est envoyé
> sur Internet. Vos souvenirs ne quittent jamais votre machine.

---

## Ce que fait l'app

- **Corrige la date** de chaque photo et vidéo.
  - Photos : récupère l'heure réelle quand elle est disponible, sinon utilise la
    date du nom de fichier (midi par défaut).
  - Vidéos : conserve la date + heure d'origine présente dans les métadonnées.
- **Réincruste les overlays** (textes, dessins, légendes) sur le snap
  correspondant — pour les photos, et en option pour les vidéos.
- **Supprime les doublons** : fichiers identiques, et souvenirs redatés par
  Snapchat (on garde toujours la version la plus ancienne).
- **Produit un dossier (ou un ZIP)** prêt à glisser dans Photos / Google Photos.

---

## Confidentialité

L'app ne contient aucun serveur, aucun envoi réseau, aucun traçage. Le
traitement utilise uniquement la puissance de votre ordinateur. Les fichiers
restent dans les dossiers que vous choisissez.

---

## Installation

Téléchargez la dernière version pour votre système dans la page
[Releases](../../releases) :

| Système  | Fichier            |
|----------|--------------------|
| macOS    | `.dmg`             |
| Windows  | `.exe` / `.msi`    |
| Linux    | `.AppImage`        |

### Premier lancement — avertissements de sécurité

L'app n'est pas (encore) signée par un certificat Apple Developer / Microsoft
payant, donc votre système affiche un avertissement au premier lancement.
C'est normal et sans danger ; voici comment l'ouvrir :

- **macOS** : sur les versions récentes (Sonoma, Sequoia…) macOS affiche
  « Snapback est endommagé » et empêche l'ouverture. Pour débloquer :
  1. Glissez l'app du DMG dans **Applications**.
  2. Ouvrez **Terminal** (Spotlight → « Terminal »).
  3. Copiez-collez cette commande puis appuyez sur **Entrée** :
     ```
     xattr -cr "/Applications/Snapback.app"
     ```
  4. Lancez l'app normalement (double-clic sur l'icône).

  La commande retire l'attribut *quarantine* que macOS pose sur les fichiers
  téléchargés ; elle ne touche pas au contenu de l'app. À ne faire qu'une
  seule fois, à l'installation.

- **Windows** : sur l'écran *Windows a protégé votre ordinateur*, cliquez sur
  **Informations complémentaires** → **Exécuter quand même**.

Une fois ouverte la première fois, l'app se lance ensuite normalement par
simple double-clic sur son icône.

---

## Comment l'utiliser

1. Exportez vos souvenirs depuis Snapchat :
   **Snapchat → Paramètres → Mes données**, en activant **Exporter mes
   souvenirs** et **Exporter les fichiers JSON**, puis téléchargez tous les ZIP.
2. Ouvrez Snapback et **déposez** votre ZIP (ou le dossier décompressé).
3. Choisissez les options (incrustation des overlays photo / vidéo).
4. Lancez le traitement et patientez (l'incrustation vidéo peut être longue).
5. Récupérez le dossier de sortie et **importez-le dans Photos / Google Photos**.

---

## Développement

### Prérequis

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Node.js](https://nodejs.org/) (LTS)
- Dépendances système Tauri : voir la
  [doc officielle](https://v2.tauri.app/start/prerequisites/)

### Lancer en local

```bash
npm install
npm run tauri dev
```

### Construire les exécutables

```bash
npm run tauri build
```

Les artefacts sont générés dans `src-tauri/target/release/bundle/`.

---

## Structure du dépôt

```
.
├── src/                 # interface (web)
├── src-tauri/           # cœur Rust (traitement, commandes Tauri)
├── reference/
│   └── snapchat_to_photos.sh   # script de référence : spécification du traitement
└── README.md
```

> Le fichier `reference/snapchat_to_photos.sh` décrit la logique exacte de
> traitement (datation, overlays, dédoublonnage). Il sert de référence ; toute
> évolution du traitement doit rester cohérente avec lui.

---

## Limites connues

- L'incrustation des **overlays vidéo** nécessite un réencodage : c'est lent sur
  un export contenant beaucoup de vidéos. Désactivable dans les options.
- Pour les **photos sans heure récupérable**, l'heure est fixée à midi — le jour
  reste correct, donc le classement chronologique est respecté.
- App non signée pour l'instant : voir la section *Premier lancement*.

---

## Licence

À définir.