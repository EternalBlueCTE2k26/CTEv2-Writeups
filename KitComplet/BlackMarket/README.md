# Challenge : Black market

## Informations du challenge

| Catégorie | Difficulté | Points | Auteur |
|-----------|------------|--------|--------|
| Crypto    | Moyen      | 200    | Miaou  |

**Preuve :** `[admin]xXCarv4lho78Xx` (sensible à la casse)

---

## Résumé

Ce challenge présente une conversation chiffrée par une substitution monoalphabétique sur les lettres.
Le but est de déduire la substitution choisie pour retrouver la conversation et déterminer qui est le chef du groupe.

1. **Reconnaissance** — Analyse du chiffrement
2. **Déductions** — Indices permettant le déchiffrement
3. **Déchiffrement** — Déchiffrer le message pour comprendre qui est le chef

---

## Étape 1 : Reconnaissance

### Découverte

Les premières lignes du fichier `encrypted_leak` sont :

```
{Dtpqqsm: #aspr-ysdgqendngqpegysf | fsciqep 09:15}

{FnwasyMj bgnqse ats dtpqqsm}

[pernq]jJDpyo4mtg78Jj: Gmp xsffgpm!
E4a4Rnqsy_XA: Gmp.
FnwasyMj: Gmp!
[pernq]jJDpyo4mtg78Jj: Asrgf ir qgog tgbs.
E4a4Rnqsy_XA: Usr-onqeg FnwasyMj!
```


### Analyse

Nous pouvons déduire quelques éléments :

- le chiffrement n'est effectué que sur les lettres
  (les symboles, espaces et retours à la ligne sont toujours visibles),
- le mot `Gmp` apparaît souvent,
- le fichier ressemble à une conversation, au format `pseudo: Phrase.`,
- il n'y a pas d'accents.

En poursuivant l'analyse, on retrouve un schéma similaire :

```
{Dtpqqsm: #aspr-ysdgqendngqpegysf | fsciqep 09:15}

pseudo: Gmp.
```

On peut supposer que la partie `fsciqep 09:15` correspond à un horodatage.


### Résultat

Nous sommes face à une conversation chiffrée, probablement en portugais puisque l'enquête se passe au Portugal.

Grâce à la répétition de certains mots, on peut déduire qu'il s'agit d'une substitution monoalphabétique,
c'est-à-dire qu'à une lettre chiffrée correspond une lettre déchiffrée.

Il faut maintenant trouver plus d'indices pour déduire cette substitution,
puis déchiffrer le message.


## Étape 2 : Déductions

Cette étape consiste à collecter les différents indices permettant de déduire la substitution.


### Participants

Il y a 4 personnes dans cette conversation :

- `[pernq]jJDpyo4mtg78Jj` parle le plus et `[pernq]` pourrait indiquer *un rôle particulier*,
- `FnwasyMj` parle un peu moins mais est mentionné par les autres dans la conversation,
- `E4a4Rnqsy_XA`,
- `MspzWgycs` est d'abord mentionné puis commence à parler plus tard.

Ce dernier élément donne un indice sur la ligne :

```
{MspzWgycs bgnqse ats dtpqqsm}
```

qui indiquerait par exemple que `MspzWgycs` a rejoint la conversation.
Si c'est le cas, on pourrait également déduire de :

```
{FnwasyMj bgnqse ats dtpqqsm}
```

que `FnwasyMj` est arrivé dans la conversation au début du fichier.
On peut supposer que c'est lui la victime du leak et qu'il s'agit de son historique de conversation.
Il est donc logique de trouver une ligne indiquant qu'il a rejoint la conversation en début de fichier.


### Horodatage

Les lignes :

```
{Dtpqqsm: #aspr-ysdgqendngqpegysf | fsciqep 09:15}
{Dtpqqsm: #aspr-ysdgqendngqpegysf | fsciqep 11:42}
{Dtpqqsm: #aspr-ysdgqendngqpegysf | fsciqep 14:20}
{Dtpqqsm: #aspr-ysdgqendngqpegysf | asydp 09:47}
{Dtpqqsm: #aspr-ysdgqendngqpegysf | asydp 18:30}
{Dtpqqsm: #aspr-ysdgqendngqpegysf | hipyap 08:59}
```

semblent donner un horaire et éventuellement un changement de jour pour la première conversation du matin.

Le texte du challenge indique que le fichier a été récupéré mercredi.
On peut donc supposer que `hipyap`, le dernier jour mentionné dans le fichier, correspond à mercredi en portugais (`quarta`).

C'est une hypothèse pour le moment, car il se peut qu'il n'y ait pas eu de conversation pendant plusieurs jours avant la fuite.


### Conversation

À chaque début de conversation, les protagonistes disent `Gmp` (40 fois), et en fin de conversation, on a parfois `Pas mgcg` ou `Pas prpqtp`.
En cherchant une liste des mots de conversation de base en portugais, on trouve :

- `Olá` (salut ou bonjour) correspondrait à `Gmp`,
- `Até logo` (à plus tard) et `Até amanhã` (à demain) correspondraient à `Pas mgcg` et `Pas prpqtp`.

L'hypothèse que le `p` chiffré correspond au `a` déchiffré (déduite de la correspondance `Gmp` <-> `Olá`)
est cohérente avec `Pas` déchiffré en `Até` et `prpqtp` déchiffré en `amanhã`.

De même, on ne trouve pas d'autre mot de 4 lettres utilisé couramment avec `Até` et dont la deuxième lettre est la même que la dernière.
On en déduit que `mgcg` correspond à `logo`, ce qui est appuyé également par le `O` de `Olá` qui serait chiffré par un `g`.

```
  chiffré: gmp pas prpqtp mgcg
déchiffré: ola ate amanha logo
```

Ces indices sont suffisamment solides et cohérents pour démarrer le déchiffrement.
On déduit les premières lettres du chiffrement monoalphabétique :

```
  chiffré: abcdefghijklmnopqrstuvwxyz
déchiffré: t-g---o-----l--anmeh------
```


### Analyse statistique

La [page Wikipédia sur la fréquence d'apparition des lettres](https://fr.wikipedia.org/wiki/Fr%C3%A9quence_d%27apparition_des_lettres)
nous donne la fréquence des lettres en portugais.
Les lettres les plus courantes sont `a`, `e`, `o`, puis `s`, `r`, `i`, `d`, ...
Nous avons donc déjà les plus courantes, mais pas les suivantes.

Avec le petit script suivant, on peut compter la fréquence d'apparition des lettres dans le texte des échanges :

```python
#!/usr/bin/env python3

from collections import Counter
import sys

if __name__ == '__main__':
    freqs = Counter()
    for line in sys.stdin:
        # chat will be the right hand side of the ':' on each line (or empty ottherwise)
        #  and contain the chat text which we want to use to count letters
        *_, chat = line.strip().partition(':')
        for c in chat:
            freqs[c] += 1
    # Results
    for c,f in freqs.most_common(10):
        print(f'{c} appeared {f} times')
```

On peut l'utiliser avec `python3 freq.py < encrypted_leak` par exemple.

Les résultats sont :

```
  appeared 572 times
p appeared 385 times
s appeared 318 times
g appeared 299 times
y appeared 198 times
f appeared 191 times
n appeared 158 times
a appeared 154 times
r appeared 133 times
q appeared 132 times
```

Nous connaissons déjà `p`, `s`, `g`, `a`, `r` et `q`, mais `y`, `f` et `n` sont de bons candidats
pour les lettres déchiffrées `s`, `r` et `i`.
Attention cependant, on ne peut pas conclure pour l'instant avec certitude que
notre texte *contient ces lettres avec la même fréquence* que celle donnée par Wikipédia.

```
    chiffré: abcdefghijklmnopqrstuvwxyz
  déchiffré: t-g---o-----l--anmeh------
à confirmer: -----r-------i----------s-
```

Il s'agit cependant d'un indice supplémentaire indiquant que les premières déductions pour `p`, `s` et `g` sont bonnes.


### Début de déchiffrement

Réalisons le script de déchiffrement, que nous améliorerons avec les nouvelles connaissances.
Le but est d'afficher les lignes encore chiffrées avec celles affichant ce que l'on sait déchiffrer,
pour nous aider à obtenir de nouvelles correspondances.

```python
#!/usr/bin/env python3

import sys

if __name__ == '__main__':
    # What we know (to be completed)
    encrypted = 'abcdefghijklmnopqrstuvwxyz'
    decrypted = 't-g---o-----l--anmeh------'

    # Put that in a dict for ease of use: the key is the encrypted letter and value is the decrypted one
    decr_map = {e:d for e,d in zip(encrypted, decrypted)}
    # Also use the same table for upper case letters
    for e,d in tuple(decr_map.items()):
        decr_map[e.upper()] = d.upper()

    # Take our input from stdin
    for line in sys.stdin:
        line = line.strip()
        print(line)
        if not line:  # Don't print empty line twice
            continue
        # Decrypt, if known, and print back
        print(''.join(decr_map.get(c, c) for c in line))
```

En lançant ce script sur le fichier leaké, on obtient (extrait) :

```
{Dtpqqsm: #aspr-ysdgqendngqpegysf | fsciqep 09:15}
{-hannel: #team--e-on----ona-o-e- | -eg-n-a 09:15}

{FnwasyMj bgnqse ats dtpqqsm}
{---te-L- -o-ne- the -hannel}

[pernq]jJDpyo4mtg78Jj: Gmp xsffgpm!
[a-m-n]---a--4lho78--: Ola -e--oal!
E4a4Rnqsy_XA: Gmp.
-4t4M-ne-_-T: Ola.
FnwasyMj: Gmp!
---te-L-: Ola!
[pernq]jJDpyo4mtg78Jj: Asrgf ir qgog tgbs.
[a-m-n]---a--4lho78--: Temo- -m no-o ho-e.
E4a4Rnqsy_XA: Usr-onqeg FnwasyMj!
-4t4M-ne-_-T: -em---n-o ---te-L-!
[pernq]jJDpyo4mtg78Jj: Usr-onqeg p shinxp.
[a-m-n]---a--4lho78--: -em---n-o a e----a.
FnwasyMj: Rinag guyncpeg.
---te-L-: M--to o---ga-o.
```

La conversation apparaît et semble logique.


### Indices supplémentaires

On peut trouver des indices pour réaliser d'autres déductions.


#### `dtpqqsm` -> `-hannel`

Ce mot semble être `channel`, même s'il est en anglais (préfixé de `the` dans le log).
Avec la correspondance `d` -> `c`, on complète les traductions suivantes, qui appuient ce choix :

```
[pernq]jJDpyo4mtg78Jj: Oprgf dgrsdpy dgr dpmrp.
[a-m-n]---a--4lho78--: -amo- comeca- com calma.
[...]
[pernq]jJDpyo4mtg78Jj: Dsyag, dgqanqisr.
[a-m-n]---a--4lho78--: Ce-to, cont-n-em.
[...]
[pernq]jJDpyo4mtg78Jj: Sjdsmsqas, guyncpeg!
[a-m-n]---a--4lho78--: E-celente, o---ga-o!
```

On en déduit la correspondance `d` -> `c`.
Avec la correction automatique (Google, Word, ChatGPT),
on peut déduire que les mots « certo » et « excellente » existent en portugais et on déduit les déchiffrements `y` -> `r` et `j` -> `x`.
Avec l'hypothèse que `n` déchiffre en `i`, `dgqanqisr` devient `contin-em`.
Le `i` chiffré correspondrait au `u`, à confirmer.

Avancement :

```
    chiffré: abcdefghijklmnopqrstuvwxyz
  déchiffré: t-gc--o--x--l--anmeh----r-
à confirmer: -----r--u----i----------s-
```

Dans les lettres à confirmer, le `y` correspond maintenant à un `r` avec une plus grande certitude ;
on peut donc faire l'hypothèse que `f` correspond plutôt à `s` qu'à `r`.

```
    chiffré: abcdefghijklmnopqrstuvwxyz
  déchiffré: t-gc--o--x--l--anmeh----r-
à confirmer: -----s--u----i------------
```


#### `Gmp xsffgpm!`

Avec l'hypothèse `f` donne `s`, `xsffgpm` devient `-essoal` et `Olá pessoal` se dit pour « bonjour tout le monde ».
La première phrase de la discussion est une forme de bonjour pour le groupe, ce qui est cohérent.

On en déduit `f` -> `s` et `x` -> `p` :

```
    chiffré: abcdefghijklmnopqrstuvwxyz
  déchiffré: t-gc-so--x--l--anmeh---pr-
à confirmer: --------u----i------------
```


#### Jours de la semaine

L'avancement actuel nous donne :

```
  chiffré: fsciqep asydp hipyap
déchiffré: -eg-n-a terca --arta
```

`terca` est `mardi`, et `--arta` pourrait être le jour qui nous intéresse, comme déjà supposé précédemment : mercredi (`quarta`).
Dans cette logique, `seg-n-a` est `segunda` (le seul jour qui correspond avec les lettres trouvées).
La présence du `u` dans `segunda` et `quarta` appuie ces hypothèses.

On en déduit `h` -> `q`, `i` -> `u` (confirmation de l'hypothèse) et `e` -> `d` :

```
    chiffré: abcdefghijklmnopqrstuvwxyz
  déchiffré: t-gcdsohux--l--anmeh---pr-
à confirmer: -------------i------------
```

Le pseudonyme qui nous intéresse le plus, `[pernq]jJDpyo4mtg78Jj`, est presque traduit : `[adm-n]xXCar-4lho78Xx`.
Il faut donc confirmer l'hypothèse `n` -> `i` et déchiffrer `o` en priorité.


#### `Fnr` et `guyncpeg`

Traduit par `S-m`, ce mot apparaît souvent et pourrait correspondre à `Sim` d'après l'hypothèse (« Oui » en portugais).

```
FnwasyMj: Fnr, sjxmndpypr-rs ir xgidg.
S--terLx: S-m, expl-caram-me um pouco.
[...]
FnwasyMj: Fnr, guyncpeg.
S--terLx: S-m, o-r-gado.
[... apparaît une seconde fois]
FnwasyMj: Fnr, guyncpeg.
S--terLx: S-m, o-r-gado.
```

On confirme l'hypothèse `n` -> `i` par `sjxmndpypr` -> `explicaram` (et par le fait qu'il ne reste plus d'autre voyelle :shrug:).

`guyncpeg` correspond maintenant à `o-rigado`, qu'on devine être `obrigado` dans la formule `Sim, obrigado` (« Oui, merci »).

Avancement :

```
    chiffré: abcdefghijklmnopqrstuvwxyz
  déchiffré: t-gcdsohux--li-anmehb--pr-
à confirmer: --------------------------
```


#### Le `o` chiffré

Avec les extraits suivants, on déduit que le `o` chiffré est le `v` déchiffré :

```
E4a4Rnqsy_XA: Usr-onqeg FnwasyMj!
D4t4Miner_PT: Bem--indo Si-terLx!
[pernq]jJDpyo4mtg78Jj: Usr-onqeg p shinxp.
[admin]xXCar-4lho78Xx: Bem--indo a ehuipa.
[...]
[pernq]jJDpyo4mtg78Jj: Popqdp g aypupmtg?
[adm-n]xXCar-4lho78Xx: A-anca o trabalho?
MspzWgycs: Fnr, popqdp.
Lea--orge: Sim, a-anca.
```

`Bem-vindo à ehuipa` (« Bienvenue dans l'équipe ») et `Avanca` (« ça avance »).

Avancement :

```
    chiffré: abcdefghijklmnopqrstuvwxyz
  déchiffré: t-gcdsohuxw-livanmehb--pr-
à confirmer: --------------------------
```

Nous avons donc assez de lettres pour comprendre le texte et faire l'hypothèse que la preuve recherchée est `[admin]xXCarv4lho78Xx`.
Finir le déchiffrement nous permettra de confirmer cette hypothèse.


### Finir le déchiffrement

Les éléments suivants nous permettent de finir le déchiffrement :

- `Xsywsnag` : du contexte on déduit `Perfeito` et `w` -> `f`, confirmé par `wndtsnyg` qui devient `ficheiro`,
- `a-udar o D4t4Miner_PT no seu pro-eto` -> `ajudar` et `projeto`, donc `b` -> `j`,
- `zzzzzzzz` : les rires en portugais en chat peuvent s'écrire `kkkkkkkkk`, donc `z` -> `k` (confirmé par `Recebemos leaks de toda a parte`),
- `Tem soft-are proprio para fa-er parsing dos dados?` : la partie `software` donne `k` -> `w`,
  et `fa-er` est `fazer` (confirmé par `ve-es` qui est `vezes`), ce qui donne `l` -> `z`,
- enfin `Usamos P-thon.` donne `v` -> `y`.

On en déduit la correspondance complète :

```
    chiffré: abcdefghijklmnopqrstuvwxyz
  déchiffré: tjgcdsohuxwzlivanmehbyfprk
```


## Étape 3 : Déchiffrement

On complète dans le script la table de déchiffrement.


### Résultat

On utilise le script pour obtenir le texte complet déchiffré en portugais.

```
{Channel: #team-recondicionadores | segunda 09:15}

{SifterLx joined the channel}

[admin]xXCarv4lho78Xx: Olá pessoal!
D4t4Miner_PT: Olá.
SifterLx: Olá!
[admin]xXCarv4lho78Xx: Temos um novo hoje.
D4t4Miner_PT: Bem-vindo SifterLx!
[admin]xXCarv4lho78Xx: Bem-vindo à equipa.
SifterLx: Muito obrigado.
[admin]xXCarv4lho78Xx: Vens de Lisboa, não?
SifterLx: Sim, exatamente.
D4t4Miner_PT: Fixe, precisávamos de reforço.
[admin]xXCarv4lho78Xx: Fazemos tratamento de dados aqui.
SifterLx: Sim, explicaram-me um pouco.
[admin]xXCarv4lho78Xx: O LeakForge devia estar aqui para te explicar, mas ainda não chegou, kkkkk!
D4t4Miner_PT: Sempre atrasado este gajo, kkkkkkkk!
SifterLx: Sem problema.
[admin]xXCarv4lho78Xx: É o nosso perito técnico.
D4t4Miner_PT: O melhor para fazer parsing.
[admin]xXCarv4lho78Xx: Vamos começar com calma.
[admin]xXCarv4lho78Xx: Próxima etapa, ajudar o D4t4Miner_PT no seu projeto.
SifterLx: Perfeito, obrigado!

{Channel: #team-recondicionadores | segunda 11:42}

[admin]xXCarv4lho78Xx: Olá.
D4t4Miner_PT: Olá!
SifterLx: Olá.
[admin]xXCarv4lho78Xx: Quem pode tratar do ficheiro?
D4t4Miner_PT: Eu não, estou noutra coisa.
SifterLx: Posso tentar, mas ainda não conheço bem.
[admin]xXCarv4lho78Xx: O LeakForge, onde está?
D4t4Miner_PT: Ainda não está.
[admin]xXCarv4lho78Xx: A sério, isto já começa a chatear!
SifterLx: Ele chega em breve, não?
[admin]xXCarv4lho78Xx: Espero...

{Channel: #team-recondicionadores | segunda 14:20}

[admin]xXCarv4lho78Xx: Bom, vou explicar um pouco o nosso trabalho.
[admin]xXCarv4lho78Xx: O LeakForge já conhece esta parte.
SifterLx: Sim, obrigado.
D4t4Miner_PT: Vai ajudar.
[admin]xXCarv4lho78Xx: Recebemos leaks de toda a parte.
[admin]xXCarv4lho78Xx: O nosso trabalho é filtrar e limpar tudo isso.
D4t4Miner_PT: Extrair o que é útil.
[admin]xXCarv4lho78Xx: Exatamente.
[admin]xXCarv4lho78Xx: Eu sou o contacto principal dos Fantasmas.
[admin]xXCarv4lho78Xx: Passam por mim para tudo.
[admin]xXCarv4lho78Xx: É importante, perceberam bem?
SifterLx: Percebido.
D4t4Miner_PT: Percebido. Pela milésima vez, kkkkk!
[admin]xXCarv4lho78Xx: Mais vale duas vezes que uma.
[admin]xXCarv4lho78Xx: Aliás, a nossa aplicação cifra as mensagens,
[admin]xXCarv4lho78Xx: portanto nada de copiar-colar texto,
[admin]xXCarv4lho78Xx: nem screenshots de nada!
[admin]xXCarv4lho78Xx: Percebido?
SifterLx: Percebido.
D4t4Miner_PT: Percebido.
[admin]xXCarv4lho78Xx: Também é preciso passar sempre por TOR e .onion para falar connosco.
[admin]xXCarv4lho78Xx: Mas isso já deves saber.
SifterLx: Sim.
[admin]xXCarv4lho78Xx: Perguntas?
SifterLx: No que estamos a trabalhar agora?
[admin]xXCarv4lho78Xx: Vários projetos.
[admin]xXCarv4lho78Xx: Bases de dados principalmente.
D4t4Miner_PT: Muitos dados.
SifterLx: Certo, obrigado.
[admin]xXCarv4lho78Xx: Temos de filtrar, algumas não têm nada de interessante.
[admin]xXCarv4lho78Xx: Outra pergunta?
SifterLx: Têm software próprio para fazer parsing dos dados?
[admin]xXCarv4lho78Xx: Usamos Python.
[admin]xXCarv4lho78Xx: Mas muitas vezes temos de ir ver manualmente com SQL.
[admin]xXCarv4lho78Xx: "SELECT" é o teu amigo ;)
SifterLx: Obrigado.
[admin]xXCarv4lho78Xx: Nunca IA, não queremos que nos descubram!
SifterLx: Percebido.
[admin]xXCarv4lho78Xx: Para informação, às vezes também temos JSON.
[admin]xXCarv4lho78Xx: Outra pergunta?
SifterLx: Não, por agora está bem.
[admin]xXCarv4lho78Xx: Perfeito. Obrigado pela atenção.
[admin]xXCarv4lho78Xx: Fazemos o ponto amanhã. Boa tarde.
SifterLx: Obrigado, até amanhã.
D4t4Miner_PT: Até amanhã, chefe.

{Channel: #team-recondicionadores | terça 09:47}

{LeakForge joined the channel}

LeakForge: Olá pessoal!
[admin]xXCarv4lho78Xx: Olá.
D4t4Miner_PT: Olá!
SifterLx: Olá.
[admin]xXCarv4lho78Xx: Olá. Finalmente, aqui estás!
D4t4Miner_PT: Já era tempo, kkkkkk!
LeakForge: Desculpa, tive problemas.
LeakForge: Não tinha permissões para entrar.
LeakForge: Enfim, olá a todos.
[admin]xXCarv4lho78Xx: Bem-vindo na mesma.
LeakForge: Ah, o novo, olá SifterLx, bem-vindo!
SifterLx: Olá LeakForge, vou precisar dos teus talentos!
[admin]xXCarv4lho78Xx: Estávamos à tua espera para ajudar o SifterLx.
[admin]xXCarv4lho78Xx: Agora vamos poder trabalhar.
LeakForge: Estou aqui agora.
[admin]xXCarv4lho78Xx: Ainda bem.
LeakForge: Qual é o programa? O que posso fazer por vocês?
SifterLx: Preciso que me apresentes as vossas ferramentas habituais.
SifterLx: Sobretudo sem IA não é fácil.
SifterLx: Estou em apuros ^^"
LeakForge: Vamos ver isso. O que é exatamente?
[admin]xXCarv4lho78Xx: Temos de encontrar informações sobre os utilizadores nas bases de dados.
[admin]xXCarv4lho78Xx: Identidade, telefones, moradas, scans de documentos oficiais, ...
SifterLx: Tenho a base de dados.
SifterLx: Os primeiros scripts não dão nada.
LeakForge: Perfeito, vou ver isso.
LeakForge: Tenho alguns scripts por aí.
LeakForge: Vou-te enviar isso.
SifterLx: Ótimo, obrigado!
[admin]xXCarv4lho78Xx: Fazemos o ponto no fim da tarde então.
[admin]xXCarv4lho78Xx: Até logo.
SifterLx: Até logo.
D4t4Miner_PT: Até logo.
SifterLx: Até logo.

{Channel: #team-recondicionadores | terça 18:30}

[admin]xXCarv4lho78Xx: Olá.
LeakForge: Olá!
D4t4Miner_PT: Olá.
SifterLx: Olá!
[admin]xXCarv4lho78Xx: Avança o trabalho?
LeakForge: Sim, avança.
D4t4Miner_PT: Progredimos bem.
[admin]xXCarv4lho78Xx: Onde estão exatamente?
SifterLx: Quase acabámos o primeiro ficheiro.
[admin]xXCarv4lho78Xx: Certo, obrigado.
[admin]xXCarv4lho78Xx: Quantos faltam?
LeakForge: Ainda três ficheiros.
[admin]xXCarv4lho78Xx: Certo, continuem.
D4t4Miner_PT: Sim, chefe!
[admin]xXCarv4lho78Xx: Falamos amanhã.
SifterLx: Até amanhã.
D4t4Miner_PT: Até amanhã!
SifterLx: Até amanhã.

{Channel: #team-recondicionadores | quarta 08:59}

[admin]xXCarv4lho78Xx: Olá.
LeakForge: Olá!
D4t4Miner_PT: Olá.
SifterLx: Olá!
[admin]xXCarv4lho78Xx: Então, as novidades?
D4t4Miner_PT: Terminámos a primeira parte.
LeakForge: Está tudo limpo agora.
[admin]xXCarv4lho78Xx: Perfeito, bom trabalho!
SifterLx: Obrigado.
D4t4Miner_PT: Obrigado!
LeakForge: Obrigado.
[admin]xXCarv4lho78Xx: Corre melhor SifterLx?
SifterLx: Sim, obrigado.
[admin]xXCarv4lho78Xx: Ótimo.
[admin]xXCarv4lho78Xx: Conseguem acabar hoje à noite?
D4t4Miner_PT: Sim, acabamos isso.
SifterLx: Sim, estará feito.
[admin]xXCarv4lho78Xx: Excelente, obrigado!
LeakForge: De nada.
```

On peut alors traduire (ou comprendre le sens global grâce aux mots proches du français)
et déduire que le chef du groupe, celui qui donne les ordres et organise le travail,
et donc la personne qui nous intéresse, est `[admin]xXCarv4lho78Xx`.
`D4t4Miner_PT` mentionne directement que l'admin est le chef à deux reprises (`Até amanhã, chefe` et `Sim, chefe!`).


✅ **Preuve :** `[admin]xXCarv4lho78Xx`
