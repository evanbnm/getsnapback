#!/usr/bin/env bash
#
# ============================================================================
#  snapchat_to_photos.sh
# ----------------------------------------------------------------------------
#  Prépare un export Snapchat "Memories" pour l'import dans Photos / iCloud,
#  en redonnant à chaque souvenir SA VRAIE DATE.
#
#  Ce que fait le script, étape par étape :
#    1. PHOTOS  : selon la route d'export, la date se récupère différemment.
#                 - export "Mes données" : EXIF effacé + date système = date de
#                   décompression (pourrie) -> on prend la date du NOM (AAAA-MM-JJ),
#                   heure fixée à midi (le nom ne contient pas l'heure).
#                 - téléchargement direct des souvenirs : la date système garde
#                   la vraie date+heure -> on récupère l'HEURE réelle.
#                 Le script détecte automatiquement la source fiable, fichier par
#                 fichier (si le jour de la date système = jour du nom -> fiable).
#    2. VIDÉOS  : Snapchat conserve déjà la vraie date+heure dans les tags
#                 QuickTime. On n'y touche pas — sauf si elle manque, auquel
#                 cas on la met depuis le nom du fichier.
#    3. OVERLAYS: les textes/dessins ("-overlay.png") sont incrustés sur le
#                 média correspondant ("-main"). Photos via ImageMagick,
#                 vidéos via ffmpeg (réencodage), puis on réinjecte la date.
#    4. DOUBLONS (contenu) : suppression des fichiers strictement identiques
#                 (même empreinte), en gardant toujours le plus ANCIEN.
#    5. DOUBLONS (identifiant) : Snapchat redate parfois un même souvenir
#                 (même UUID, date différente). On garde le plus ancien.
#
#  Rien n'est jamais SUPPRIMÉ définitivement : les doublons écartés sont
#  DÉPLACÉS dans un dossier "snapchat_dupes" (récupérables au besoin).
#
# ----------------------------------------------------------------------------
#  PRÉREQUIS
#    - macOS avec Homebrew (https://brew.sh)
#    - Outils : brew install exiftool ffmpeg imagemagick
#
#  COMMENT OBTENIR L'EXPORT
#    Snapchat -> Paramètres -> "Mes données" :
#      * activer "Exporter mes souvenirs"
#      * activer "Exporter les fichiers JSON"
#    Télécharger TOUS les ZIP reçus, les décompresser dans un même dossier.
#
#  USAGE
#    ./snapchat_to_photos.sh [options] <dossier_export> [dossier_sortie]
#
#    Options :
#      -n, --dry-run   Simulation : affiche ce qui serait fait, sans rien
#                      modifier ni déplacer. À lancer en premier pour vérifier.
#      -y, --yes       Ne pas demander de confirmation au démarrage.
#      -h, --help      Affiche cette aide.
#
#    Exemples :
#      ./snapchat_to_photos.sh --dry-run ~/Downloads/snap     (simulation)
#      ./snapchat_to_photos.sh ~/Downloads/snap               (pour de vrai)
#
#    Par défaut, la sortie est créée à côté de l'export :
#      <export>/../snapchat_ready    -> à importer dans Photos
#      <export>/../snapchat_dupes    -> doublons écartés (à vérifier/supprimer)
#
#  ATTENTION
#    Le script réorganise les fichiers de l'export (il les DÉPLACE vers la
#    sortie). Garde les ZIP d'origine en sauvegarde si tu veux pouvoir
#    recommencer. Les fichiers ne sont pas altérés en contenu (hors overlays).
# ============================================================================

set -uo pipefail

# --- petites fonctions d'affichage -----------------------------------------
log()  { printf '\n\033[1;36m== %s ==\033[0m\n' "$*"; }
info() { printf '   %s\n' "$*"; }
warn() { printf '\033[1;33m   ! %s\033[0m\n' "$*"; }
die()  { printf '\033[1;31m   x %s\033[0m\n' "$*" >&2; exit 1; }

show_help() { sed -n '2,60p' "$0" | sed 's/^#\{0,1\} \{0,1\}//'; }

# --- extensions / utilitaires ----------------------------------------------
is_image() { case "${1##*.}" in jpg|JPG|jpeg|JPEG|png|PNG|heic|HEIC) return 0;; *) return 1;; esac; }
is_video() { case "${1##*.}" in mp4|MP4|mov|MOV|m4v|M4V) return 0;; *) return 1;; esac; }

# extrait la date AAAA-MM-JJ du début d'un nom de fichier ; vide si absente
date_of() {
  local n; n="$(basename "$1")"; n="${n:0:10}"
  [[ "$n" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]] && printf '%s' "$n" || printf ''
}

# --- analyse des arguments ---------------------------------------------------
DRY=0; ASSUME_YES=0; SRC=""; OUT_ARG=""
while [ $# -gt 0 ]; do
  case "$1" in
    -n|--dry-run) DRY=1 ;;
    -y|--yes)     ASSUME_YES=1 ;;
    -h|--help)    show_help; exit 0 ;;
    -*)           die "Option inconnue : $1 (voir --help)" ;;
    *)            if [ -z "$SRC" ]; then SRC="$1"
                  elif [ -z "$OUT_ARG" ]; then OUT_ARG="$1"
                  else die "Trop d'arguments (voir --help)"; fi ;;
  esac
  shift
done

[ -z "$SRC" ] && die "Usage : $0 [--dry-run] <dossier_export> [dossier_sortie]"
[ -d "$SRC" ] || die "Dossier introuvable : $SRC"
SRC="$(cd "$SRC" && pwd)"

# --- vérification des outils -------------------------------------------------
for c in exiftool ffmpeg ffprobe magick shasum; do
  command -v "$c" >/dev/null 2>&1 || \
    die "Outil manquant : $c — installe-le avec :  brew install exiftool ffmpeg imagemagick"
done

# ============================================================================
#  MODE SIMULATION : on compte et on rapporte, sans rien toucher
# ============================================================================
if [ "$DRY" -eq 1 ]; then
  log "MODE SIMULATION — aucune modification, aucun déplacement"
  info "Export analysé : $SRC"

  nimg_real=0; nimg_noon=0; nvid=0; nvid_nodate=0
  while IFS= read -r -d '' f; do
    d="$(date_of "$f")"; [ -z "$d" ] && continue
    if is_image "$f"; then
      fday="$(exiftool -q -s3 -d '%Y-%m-%d' -FileModifyDate "$f" 2>/dev/null)"
      if [ "$fday" = "$d" ]; then nimg_real=$((nimg_real+1)); else nimg_noon=$((nimg_noon+1)); fi
    elif is_video "$f"; then
      nvid=$((nvid+1))
      exiftool -s3 -QuickTime:CreateDate "$f" 2>/dev/null | grep -q . || nvid_nodate=$((nvid_nodate+1))
    fi
  done < <(find "$SRC" -type f -name '*-main.*' -print0)

  nov_img=0; nov_vid=0; nov_orphan=0
  while IFS= read -r -d '' ov; do
    prefix="${ov%-overlay.*}"; main=""
    for cand in "${prefix}-main."*; do [ -f "$cand" ] && { main="$cand"; break; }; done
    if   [ -z "$main" ];        then nov_orphan=$((nov_orphan+1))
    elif is_image "$main";      then nov_img=$((nov_img+1))
    elif is_video "$main";      then nov_vid=$((nov_vid+1)); fi
  done < <(find "$SRC" -type f -name '*-overlay.*' -print0)

  ndup_uuid="$(find "$SRC" -type f -name '*-main.*' -exec basename {} \; \
    | sed -E 's/^[0-9]{4}-[0-9]{2}-[0-9]{2}_([A-Fa-f0-9-]+)-main\..*/\1/' \
    | sort | uniq -d | wc -l | tr -d ' ')"

  log "Ce qui serait fait"
  info "PHOTOS :"
  info "  - $nimg_real avec HEURE réelle (date système fiable)"
  info "  - $nimg_noon avec date du nom + midi (date système non fiable)"
  info "VIDÉOS :"
  info "  - $nvid au total ; $nvid_nodate sans date QuickTime (datées depuis le nom)"
  info "OVERLAYS (incrustations) :"
  info "  - $nov_img sur des photos, $nov_vid sur des vidéos"
  [ "$nov_orphan" -gt 0 ] && info "  - $nov_orphan overlay(s) sans -main correspondant (ignorés)"
  info "DOUBLONS :"
  info "  - $ndup_uuid identifiant(s) Snapchat en double (détectés sur les noms)"
  info "  - doublons au contenu identique : calculés seulement lors du vrai run"
  echo
  info "Pour lancer pour de vrai :  $0 \"$SRC\""
  echo
  exit 0
fi

# ============================================================================
#  PRÉPARATION DU VRAI RUN
# ============================================================================
OUT="${OUT_ARG:-$SRC/../snapchat_ready}"
mkdir -p "$OUT"; OUT="$(cd "$OUT" && pwd)"
DUPES="$OUT/../snapchat_dupes"
mkdir -p "$DUPES"; DUPES="$(cd "$DUPES" && pwd)"

info "Export  : $SRC"
info "Sortie  : $OUT"
info "Doublons: $DUPES"

# --- garde-fou : sauvegarde + confirmation ----------------------------------
if [ "$ASSUME_YES" -ne 1 ]; then
  echo
  warn "Ce script va RÉORGANISER l'export : les fichiers seront DÉPLACÉS"
  warn "vers le dossier de sortie. L'export d'origine sera donc consommé."
  warn "Garde tes ZIP Snapchat d'origine en sauvegarde avant de continuer."
  warn "Astuce : lance d'abord avec --dry-run pour prévisualiser."
  printf '   >> Appuie sur Entrée pour continuer, ou Ctrl-C pour annuler... '
  read -r _ || true
fi

# ============================================================================
log "Étape 1/5 — Inscription des dates (heure réelle si dispo, sinon midi)"
# ============================================================================
n=0
while IFS= read -r -d '' f; do
  d="$(date_of "$f")"; [ -z "$d" ] && continue
  dc="${d//-/:}"                       # AAAA-MM-JJ -> AAAA:MM:JJ (format exiftool)
  if is_image "$f"; then
    # On choisit la source fiable fichier par fichier : si le JOUR de la date
    # système = JOUR du nom -> date système fiable (heure réelle préservée) ;
    # sinon -> date du nom + midi.
    fday="$(exiftool -q -s3 -d '%Y-%m-%d' -FileModifyDate "$f" 2>/dev/null)"
    if [ "$fday" = "$d" ]; then
      exiftool -q -overwrite_original \
        '-EXIF:DateTimeOriginal<FileModifyDate' \
        '-EXIF:CreateDate<FileModifyDate' "$f" >/dev/null 2>&1 && n=$((n+1))
    else
      exiftool -q -overwrite_original \
        "-EXIF:DateTimeOriginal=${dc} 12:00:00" \
        "-EXIF:CreateDate=${dc} 12:00:00" "$f" >/dev/null 2>&1 && n=$((n+1))
    fi
  elif is_video "$f"; then
    # on ne touche la vidéo QUE si elle n'a pas déjà une date QuickTime
    if ! exiftool -s3 -QuickTime:CreateDate "$f" 2>/dev/null | grep -q .; then
      exiftool -q -overwrite_original -api QuickTimeUTC \
        "-QuickTime:CreateDate=${dc} 12:00:00" \
        "-QuickTime:ModifyDate=${dc} 12:00:00" \
        "-QuickTime:TrackCreateDate=${dc} 12:00:00" \
        "-QuickTime:MediaCreateDate=${dc} 12:00:00" "$f" >/dev/null 2>&1 && n=$((n+1))
    fi
  fi
done < <(find "$SRC" -type f -name '*-main.*' -print0)
info "Fichiers datés : $n"

# ============================================================================
log "Étape 2/5 — Incrustation des overlays (textes/dessins) sur les snaps"
# ============================================================================
info "Les vidéos sont réencodées : cette étape peut être longue. Patiente."
while IFS= read -r -d '' ov; do
  prefix="${ov%-overlay.*}"            # chemin sans le suffixe -overlay.ext
  main=""
  for cand in "${prefix}-main."*; do [ -f "$cand" ] && { main="$cand"; break; }; done
  [ -z "$main" ] && { warn "aucun -main pour $(basename "$ov")"; continue; }

  out="$OUT/$(basename "$main")"
  [ -e "$out" ] && continue            # déjà fusionné lors d'un run précédent

  if is_image "$main"; then
    if magick "$main" "$ov" \
         -resize "$(magick identify -format '%wx%h' "$main")^" \
         -gravity center -composite "$out" 2>/dev/null; then
      exiftool -q -overwrite_original -TagsFromFile "$main" \
        -DateTimeOriginal -CreateDate "$out" >/dev/null 2>&1
      info "photo fusionnée : $(basename "$main")"
    else
      warn "overlay illisible, snap conservé sans incrustation : $(basename "$main")"
      cp "$main" "$out"
    fi
  elif is_video "$main"; then
    if ffmpeg -y -loglevel error -i "$main" -i "$ov" \
         -filter_complex "[1:v]scale=rw:rh[o];[0:v][o]overlay=0:0[v]" \
         -map "[v]" -map "0:a?" -c:v libx264 -crf 18 -preset medium -c:a copy \
         "$out" 2>/dev/null; then
      exiftool -q -overwrite_original -api QuickTimeUTC -TagsFromFile "$main" \
        -QuickTime:CreateDate -QuickTime:ModifyDate \
        -QuickTime:TrackCreateDate -QuickTime:MediaCreateDate "$out" >/dev/null 2>&1
      info "vidéo fusionnée  : $(basename "$main")"
    else
      warn "incrustation vidéo échouée, snap conservé tel quel : $(basename "$main")"
      cp "$main" "$out"
    fi
  fi
done < <(find "$SRC" -type f -name '*-overlay.*' -print0)

# ============================================================================
log "Étape 3/5 — Regroupement de tous les snaps dans le dossier de sortie"
# ============================================================================
# On déplace les -main restants. Ceux qui ont déjà une version fusionnée
# (même nom déjà présent dans la sortie) sont ignorés : pas de doublon.
n=0
while IFS= read -r -d '' f; do
  out="$OUT/$(basename "$f")"
  [ -e "$out" ] && continue
  mv -f "$f" "$out" && n=$((n+1))
done < <(find "$SRC" -type f -name '*-main.*' -print0)
info "Snaps déplacés vers la sortie : $n"

# ============================================================================
log "Étape 4/5 — Doublons au contenu identique (on garde le plus ancien)"
# ============================================================================
info "Calcul des empreintes (lecture de tous les fichiers : peut être long)..."
TMPH="$(mktemp)"; TMPD="$(mktemp)"
find "$OUT" -type f -name '*-main.*' -exec shasum {} \; | sort > "$TMPH"

# Pour chaque empreinte identique : garder le fichier dont la DATE est la plus
# ancienne (les noms commencent par AAAA-MM-JJ), lister les autres.
awk '{
  h=$1; $1=""; sub(/^  */,""); p=$0
  n=p; sub(/.*\//,"",n); d=substr(n,1,10)
  if (!(h in kd) || d < kd[h]) { if (h in keep) print keep[h]; keep[h]=p; kd[h]=d }
  else { print p }
}' "$TMPH" > "$TMPD"

n=0
while IFS= read -r f; do [ -f "$f" ] && mv -f "$f" "$DUPES/" && n=$((n+1)); done < "$TMPD"
info "Doublons (contenu) écartés : $n"

# ============================================================================
log "Étape 5/5 — Doublons de même identifiant Snapchat (on garde le plus ancien)"
# ============================================================================
TMPU="$(mktemp)"
find "$OUT" -type f -name '*-main.*' -exec basename {} \; \
  | sed -E 's/^[0-9]{4}-[0-9]{2}-[0-9]{2}_([A-Fa-f0-9-]+)-main\..*/\1/' \
  | sort | uniq -d > "$TMPU"

n=0
while IFS= read -r u; do
  [ -z "$u" ] && continue
  first=1
  while IFS= read -r p; do
    if [ "$first" -eq 1 ]; then first=0; continue; fi   # garde le 1er (plus ancien)
    [ -f "$p" ] && mv -f "$p" "$DUPES/" && n=$((n+1))
  done < <(find "$OUT" -type f -name "*${u}-main.*" | sort)
done < "$TMPU"
info "Doublons (identifiant) écartés : $n"

rm -f "$TMPH" "$TMPD" "$TMPU"

# ============================================================================
log "Terminé"
# ============================================================================
total="$(find "$OUT" -type f -name '*-main.*' | wc -l | tr -d ' ')"
ndup="$(find "$DUPES" -type f | wc -l | tr -d ' ')"
info "Snaps prêts à importer : $total   (dans $OUT)"
info "Doublons écartés       : $ndup   (dans $DUPES)"
echo
info "ÉTAPES SUIVANTES :"
info "  1. Ouvre l'app Photos -> Fichier -> Importer -> choisis : $OUT"
info "  2. Vérifie dans la vue par année que les dates sont correctes."
info "  3. Photos -> Réglages -> iCloud : active 'Photos iCloud' pour synchroniser."
info "  4. Une fois TOUT confirmé dans Photos + iCloud, tu peux supprimer"
info "     les dossiers de travail pour libérer de l'espace :"
info "       rm -rf \"$SRC\" \"$DUPES\""
echo
