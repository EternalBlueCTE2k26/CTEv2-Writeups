## Challenge : Porte dérobée

## Informations du challenge

| Catégorie | Difficulté | Points | Auteur |
|-----------|------------|--------|--------|
| Crypto | Moyen | 150 | B3cha |

**Preuve :** `C0rs41r@cybergold.agency`

## Résumé

Pour rebondir d'un serveur à l'autre, les administrateurs légitimes comme les attaquants sont susceptibles d'utiliser des clés SSH pour se latéraliser sur un réseau.

Lors du challenge `Configuration idéale`, en allant au-delà du flag recherché, vous avez trouvé le fichier `/home/<utilisateur>/.ssh/id_rsa`.

Ici, nous disposons seulement d'une clé SSH privée `id_rsa`, qui est un nom de fichier générique pour les clés SSH au format cryptographique RSA.

En l'ouvrant comme un fichier texte classique, on peut afficher son contenu :
```
-----BEGIN OPENSSH PRIVATE KEY-----                                                                     
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
-----END OPENSSH PRIVATE KEY-----
```

Par défaut, le nom d'utilisateur et le nom de la machine depuis laquelle le couple clé privée / clé publique a été généré sont mémorisés sous forme de commentaire.
Ces deux informations peuvent être précieuses pour identifier le pseudonyme et le nom du PC (voire du serveur) utilisés par le propriétaire de l'artefact.
Ce commentaire peut être facilement trouvé en clair à l'intérieur du fichier de clé publique `id_rsa.pub`, qui est automatiquement généré en complément de la clé privée. Cependant, dans notre cas, la clé publique `id_rsa.pub` n'est pas présente sur la machine.

Néanmoins, nous pouvons tout de même utiliser la commande `ssh-keygen` pour afficher ce précieux renseignement. La commande exacte est : `ssh-keygen -lf id_rsa`

Nous obtenons ainsi le pseudonyme du propriétaire de la clé ainsi que le nom du serveur depuis lequel il opère. Pas très fort en OpSec !

✅ Preuve : **C0rs41r@cybergold.agency**
