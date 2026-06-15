## Challenge : Le Vault

## Informations du challenge

| Catégorie | Difficulté | Points | Auteur |
|-----------|------------|--------|--------|
| Reverse | Facile | 200 | RORO! |

**Preuve :** `TALEB`

## Résumé

Le but de ce challenge est de retrouver le nom de famille de l'auteur d'un script Python sécurisé nommé `vault.py`.

**Nota :** il faut bien utiliser la version `Python version 3.13`.

## 1. Analyse initiale

Le fichier `vault.py`, téléchargé depuis le GitHub de Samir [https://github.com/samir-taleb/The-Vault](https://github.com/samir-taleb/The-Vault), est protégé par **PyArmor**, un outil d'obfuscation qui rend le code source illisible et empêche l'analyse statique simple. Le code ressemble à ceci :

```python
from pyarmor_runtime_000000 import __pyarmor__
__pyarmor__(__name__, __file__, b'PY000000...')
```

## 2. Étapes d'extraction du code

L'extraction du code déchiffré a suivi une méthodologie précise d'analyse dynamique :

1. **Préparation de l'environnement** : utilisation d'un script d'interception (`bypass.py`) reposant sur la fonction `sys.addaudithook` de Python.
2. **Configuration du Hook** : mise en place d'un "audit hook" pour surveiller l'événement `code.__new__`. Cet événement est déclenché chaque fois qu'un nouvel objet de code est créé en mémoire (ce que fait PyArmor après avoir déchiffré le bytecode).
3. **Lancement et blocage** : exécution du script `vault.py` via l'intercepteur. Le script s'exécute normalement jusqu'à ce qu'il demande une saisie utilisateur (`input` ou `getpass`). À ce stade précis, le code est entièrement déchiffré et présent dans la mémoire vive.
4. **Inspection de la pile (Stack)** : pendant que le script attend la saisie, nous utilisons le module `inspect` pour parcourir la pile d'exécution (`inspect.stack()`). Cela nous permet de localiser l'objet de code correspondant au module `vault`.
5. **Extraction des constantes** : une fois l'objet de code identifié, nous extrayons ses constantes (`co_consts`). C'est ici que nous avons récupéré la clé de chiffrement, le sel, et les données chiffrées (mot de passe et bannière).

## 3. Exemple de script de bypass

Voici un script simplifié (`bypass_example.py`) qui illustre cette technique d'interception et d'extraction :

```python
import sys, os, inspect, getpass

# 1. Hook pour intercepter la création d'objets de code
def audit_hook(event, args):
    if event == "code.__new__": pass
sys.addaudithook(audit_hook)

# 2. Interception au moment du prompt de mot de passe
original_getpass = getpass.getpass
def hooked_getpass(prompt='Password: ', stream=None):
    print(f"\n[!] Interception au moment du prompt : {prompt}")
    for frame_info in inspect.stack():
        code = frame_info.frame.f_code
        if "vault" in code.co_filename:
            print(f"[*] Objet de code trouvé : {code.co_filename} ({code.co_name})")
            for i, const in enumerate(code.co_consts):
                if isinstance(const, (bytes, tuple)):
                    print(f"  [{i}] {const}")
    sys.exit(0)
getpass.getpass = hooked_getpass

# 3. Exécution
if __name__ == "__main__":
    target = "dist/vault.py"
    sys.path.append(os.path.dirname(os.path.abspath(target)))
    with open(target, "rb") as f: code = f.read()
    exec(code, {"__name__": "__main__", "__file__": os.path.abspath(target)})
```

### Résultat de l'exécution

```text
[*] Lancement de dist/vault.py...
Welcome to LeVault Secure Storage.
[!] Interception au moment du prompt : Enter password: 
[*] Objet de code trouvé : <frozen vault> (<module>)
[*] Extraction des constantes (co_consts) :
  [16] b'Sup3rS3cr3tK3y'
  [17] b'S4ltY_V4lu3'
  [18] (48, 117, 44, 112, 25, 60, 85, 111)
  [19] (10, 32, 105, 51, 67, 99, 23, 119, 36, 102, 20, 121, 106, 124, 85, 12, 123, 36, ...)
```

## 4. Identification de la logique XOR + Salt

L'identification du double XOR avec un sel a été basée sur plusieurs observations. Tout d'abord, les variables `KEY`, `SALT` et `ENC_PASSWORD` ont été trouvées regroupées dans les constantes extraites (`co_consts`). Ensuite, le `ENC_PASSWORD` était stocké sous forme de tuple d'entiers, ce qui est un schéma classique pour représenter des octets chiffrés. L'intuition cryptographique nous a également orientés vers le XOR, qui est l'algorithme de prédilection pour l'obfuscation simple, et la présence de deux chaînes d'octets distinctes suggérait que les deux étaient appliquées. Enfin, le désassemblage de la fonction `decrypt` a confirmé l'utilisation de l'opcode `BINARY_XOR` à deux reprises dans une boucle, avec `BINARY_SUBSCR` et `BINARY_MODULO` pour parcourir la clé et le sel.

## 5. Analyse des données extraites

En inspectant la mémoire, nous avons trouvé :

* **Clé (KEY)** : `Sup3rS3cr3tK3y`
* **Sel (SALT)** : `S4ltY_V4lu3`
* **Mot de passe chiffré** : `(48, 117, 44, 112, 25, 60, 85, 111)`

## 6. Déchiffrement du mot de passe

Le script utilise une opération XOR simple avec la clé et le sel. En appliquant l'opération inverse, nous avons trouvé le mot de passe :

**Mot de passe : `04072004`**

## 7. Résultat final

En entrant le mot de passe `04072004`, le coffre-fort affiche :

```text
author : Samir TALEB
email contact : Samir.taleb@proton.me
```

## Résultat

Le nom de famille recherché est : **TALEB**.

✅ **Preuve :** `TALEB`
