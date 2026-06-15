# Challenge : FALL IN LOVE

## Informations du challenge

| Catégorie | Difficulté | Points | Auteur |
|-----------|------------|--------|--------|
| Web | Moyen | 100 | Rooting |

**Preuve :** `Sam1r_&&_M3la13_L0ve_f0R_Ev3R`

---

## Résumé

Ce challenge web enchaîne 3 vulnérabilités pour accéder à un message secret contenant le flag :

1. **Information disclosure** – L'API expose les utilisateurs et leurs IDs.
2. **IDOR sur l'inscription** – Le formulaire d'inscription accepte un paramètre `user_id` (présent dans la requête par défaut) permettant de prendre le contrôle du compte admin.
3. **SQL Injection** – Le déverrouillage d'un message protégé par mot de passe est vulnérable à une injection SQL.

---

## Étape 1 : Reconnaissance et information disclosure

### Découverte

Après création d'un compte et connexion, le tableau de bord propose un « API Explorer » avec un bouton **Lister tous les utilisateurs**. Cet endpoint expose les comptes et leurs IDs.

### Exploitation

```bash
# Une fois connecté (cookie de session valide)
curl -b "session=..." http://target:5000/api/users
```

Ou via l'interface : cliquer sur « Lister tous les utilisateurs (API) ».

### Résultat

On obtient la liste des utilisateurs avec leurs **id** (1 = samir, 2 = melanie, 3 = admin). L'admin (id 3) est la cible pour accéder au panel admin et aux messages.

### Indice pour la suite

Lors de l'inscription, en inspectant la requête POST (onglet Réseau), on voit que le formulaire envoie un paramètre **user_id** (vide par défaut). Le backend peut donc accepter ce paramètre lors de la création de compte.

---

## Étape 2 : IDOR sur le formulaire d'inscription

### Contexte

L'inscription se fait uniquement via le formulaire `/register`. Aucun champ `user_id` n'est affiché, mais la requête contient déjà `user_id=` (champ caché). Si on envoie `user_id=3`, le backend peut mettre à jour le compte admin au lieu de créer un nouveau compte.

### Analyse

- Les IDs ont été découverts à l'étape 1 (admin = 3).
- Seul le compte avec `id = 3` (admin) peut être modifié via ce mécanisme ; les autres renvoient une erreur « Ce compte ne peut pas être modifié ».

### Exploitation

**Méthode 1 – DevTools :**
Sur la page d'inscription, ouvrir les DevTools (F12) → onglet **Elements**, trouver le champ caché :

```html
<input type="hidden" name="user_id" value="">
```

Changer `value=""` en `value="3"`, puis remplir **tous les champs du formulaire** (nom d'utilisateur, email, mot de passe) et soumettre. L'email peut rester vide, mais le champ doit être présent comme dans le formulaire original.

**Méthode 2 – Requête directe :**

Envoyer les mêmes paramètres que le formulaire (username, password, email, user_id). L'email peut être vide mais doit être inclus dans la requête :

```bash
curl -X POST http://target:5000/register \
  -d "username=hacker&password=pass123&email=&user_id=3" \
  -c cookies.txt \
  -L
```

Puis se connecter avec **les identifiants que vous venez de définir** (ex. `hacker` / `pass123`). Le compte admin (id 3) a été remplacé par ces identifiants.

### Résultat

Le compte admin (id 3) est remplacé par les identifiants choisis. Après connexion, le lien **Panel Admin** apparaît dans le tableau de bord.

### Ce qui ne fonctionne pas

- `user_id=1` ou `user_id=2` → « Ce compte ne peut pas être modifié »
- Créer un compte sans envoyer `user_id` → compte normal, pas admin
- Utiliser une ancienne API d'inscription (supprimée) → 404

---

## Étape 3 : Accès au message secret et SQL Injection

### Contexte

En tant qu'admin, on accède au **Panel Admin**, qui permet de consulter un message par ID (`/admin/messages?id=X`). Certains messages sont « protégés » et demandent un mot de passe pour afficher le contenu. Le déverrouillage se fait via `POST /admin/messages/<id>/unlock` avec un JSON `{"password": "..."}`. La vérification du mot de passe est faite avec une requête SQL construite par concaténation → **SQL Injection**.

### Découverte du message secret

Énumérer les IDs de messages (par exemple 1 à 20) jusqu'à trouver un message dont le sujet contient « secret » (ex. « Mon message secret pour toi ❤️ »). Exemple avec l'interface : saisir l'ID dans « Consulter un message spécifique par ID » et cliquer sur « Voir ». Le contenu affiché est « *** Contenu protégé - Mot de passe requis *** » et un champ mot de passe apparaît. **Notez cet ID** (par ex. 11) : c'est le seul message pour lequel le déverrouillage par SQLi fonctionnera (voir ci-dessous).

### Analyse de la vulnérabilité

La requête côté serveur est du type :

```sql
SELECT message_id FROM message_passwords WHERE message_id = <id> AND password = '<password>'
```

En envoyant un mot de passe comme `' OR '1'='1' --`, la condition devient toujours vraie et la requête renvoie une ligne. **Important :** le backend vérifie que le `message_id` renvoyé correspond à celui demandé dans l'URL. La table `message_passwords` ne contient qu'une seule entrée (celle du message secret). Donc si vous appelez `/unlock` avec un autre ID (ex. 5), la requête renvoie quand même l'ID du message secret (ex. 11) ; le serveur compare 11 ≠ 5 et renvoie « Incorrect password ». **Il faut donc utiliser l'ID du message secret dans l'URL** (celui dont le sujet contient « secret »), sinon la SQLi ne « déverrouille » pas.

### Exploitation

Une fois l'ID du message secret trouvé (ex. 11) :

**Via l'interface :**
Dans le champ « Mot de passe pour déverrouiller », saisir :

```
' OR '1'='1' --
```

Puis cliquer sur « Déverrouiller ».

**Via curl :**

```bash
curl -X POST http://target:5000/admin/messages/11/unlock \
  -H "Content-Type: application/json" \
  -b "session=..." \
  -d "{\"password\": \"' OR '1'='1' --\"}"
```

(Adapter l'ID si nécessaire et le cookie de session admin.)

### Résultat

La réponse contient le contenu complet du message, avec le flag :

```
Le flag est : Sam1r_&&_M3la13_L0ve_f0R_Ev3R
```

**Preuve :** `Sam1r_&&_M3la13_L0ve_f0R_Ev3R`

### Ce qui ne fonctionne pas

- Mot de passe vide → « Password required »
- Mot de passe incorrect sans injection → « Incorrect password »
- **Mauvais ID dans l'URL** (un message qui n'est pas le message secret) → « Incorrect password » : la SQLi ne contourne que la vérification du mot de passe, mais le serveur exige que l'ID du message demandé soit celui renvoyé par la requête (le message secret).
- ID de message inexistant → « Message not found »

---

## Récapitulatif de la chaîne

1. S'inscrire, se connecter.
2. Utiliser l'API Explorer pour lister les utilisateurs et noter l'ID admin (3).
3. Voir dans la requête d'inscription le paramètre `user_id`, puis se réinscrire en forçant `user_id=3` pour prendre le compte admin.
4. Se connecter en admin, aller au Panel Admin, trouver l'ID du message secret.
5. Déverrouiller ce message avec une SQL Injection (`' OR '1'='1' --`) pour lire le flag.
