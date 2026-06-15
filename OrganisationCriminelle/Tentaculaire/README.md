# Challenge : Tentaculaire

## Informations du challenge

| Catégorie | Difficulté | Points | Auteur |
|-----------|------------|--------|--------|
| Forensic | Moyen | 300 | RORO! |

**Preuve :** `Portugal Espagne Italie France`

## Résumé

L'objectif de la mission OSINT et Forensics était de récupérer les 4 occurrences de pays marquant l'évolution
chronologique du groupe criminel « Fantasmas-de-Redes » ou « FakeBrokers » et de les documenter.

---

## Étape 1 : Phase de reconnaissance (année 2022 — 1re conquête)

* **Découverte :** en atterrissant sur l'application web, on tombe sur la page d'accueil d'un forum underground s'appelant *Tentaculaire*.
* **Vulnérabilité / Vecteur :** inspection de code (Information Disclosure).
* **Exploitation :** lors de l'inspection du code source de la page publique non authentifiée (`CTRL+U` ou `F12`), une ligne de texte volontairement masquée à l'aide des classes CSS utilitaires `opacity-0 absolute select-none hidden` est présente et lisible dans le DOM :
  `<span class="opacity-0 absolute select-none hidden">1st: Portugal</span>`
* **Preuve obtenue :** `Portugal` *(lancement du business)*

---

## Étape 2 : Infiltration du réseau (année 2023 — 2e conquête)

* **Découverte :** pour aller plus loin, une authentification est nécessaire. Les mots de passe par défaut ou la possibilité de s'inscrire permettent d'accéder aux fonctionnalités privées.
* **Vulnérabilité / Vecteur :** fonctionnalité d'accès au niveau Autorisé / profil d'un utilisateur standard.
* **Exploitation :** après la création d'un compte (ou la découverte d'un compte valide), on a accès à l'espace de discussion (`/forum`). En lisant l'historique des publications (les anciens messages), on trouve un topic posté par l'entité *admin* dont le titre est : **« Recherche de nouveaux locaux »**. Dans le corps du message, on peut lire :
  > « Suite au succès de l'année dernière, nous recherchons activement de nouveaux locaux près de la Sagrada Família (Espagne) pour notre prochaine antenne. »
* **Preuve obtenue :** `Espagne` *(installation de l'antenne secondaire)*

---

## Étape 3 : Faille applicative et fuite (année 2024 — 3e conquête)

* **Découverte :** en naviguant dans l'espace authentifié sur la vue des utilisateurs membres (`/members`), on repère une fonctionnalité destinée à exporter ses propres données (`/api/user/export?id=X`).
* **Vulnérabilité / Vecteur :** Insecure Direct Object Reference (IDOR).
* **Exploitation :** la fonctionnalité d'export passe, par la Query String, l'ID de la cible à exporter sans vérifier si le compte en cours de session (token JWT de l'attaquant) détient la permission sur cette cible. L'identifiant demandé n'est pas validé.
  En interrogeant l'API avec un script ou en changeant simplement la valeur de l'`id` avec notre propre identifiant ou celui de l'administrateur, le fichier dump renvoyé par le serveur contient une trace technique du système d'export de données de l'organisation :
  `[INTERNAL_SYS_LOG] 2024: Rome (Italie) branch synchronized.`
* **Preuve obtenue :** `Italie` *(consolidation en pleine ampleur)*

---

## Étape 4 : Compromission critique du serveur (année 2025 — 4e conquête)

* **Découverte :** lors d'un scan basique ou d'une fouille de fichiers résiduels, on accède au fichier `package-lock.json` de l'application, exposé à la racine de la production. Ce fichier révèle l'utilisation de Next.js version *15.0.3* et de React Node *19.0.0-rc-66855b96*.
* **Vulnérabilité / Vecteur :** Server-Side Remote Code Execution (RCE) — alias CVE-2025-55182 / React2Shell.
* **Exploitation :** au moyen d'un scanner (comme Trivy), ces dépendances sont signalées comme vulnérables à React2Shell. La charge utile malveillante abuse de `react-server-dom-webpack` lors de l'appel aux flux d'une Server Action de React. Le simple fait d'envoyer un corps JSON / multipart forgé comprenant une directive nécessitant le module natif `child_process` force le backend à l'exécuter.

  Grâce à la commande :
  `var res = require('child_process').execSync('cat /flag.txt');` encapsulée dans le payload de la Server Action, la réponse 303 Redirect qui s'ensuit affiche dans le header `Location` le contenu du fichier `/flag.txt` hébergé sur le serveur distant :

  `Objectif ultime 2025 : France`
* **Preuve obtenue :** `France` *(cible finale)*

---

## Résultat

L'investigation progressive, de l'OSINT web classique à une compromission totale de l'infrastructure via la fameuse Zero-Day, a permis d'extraire la timeline complète de l'expansion du groupe criminel dans l'ordre de ses ouvertures géographiques.

La solution de notre challenge est la liste des 4 pays conquis dans l'ordre. Le fichier <a href="source/poc.sh">Proof of Concept</a> permet de résoudre le challenge.

✅ **Preuve :** `Portugal Espagne Italie France`
