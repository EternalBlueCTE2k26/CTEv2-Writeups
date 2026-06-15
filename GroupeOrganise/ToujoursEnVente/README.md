# Challenge : Toujours en vente

## Informations du challenge

| Catégorie | Difficulté | Points | Auteur |
|-----------|------------|--------|--------|
| Reverse   | Moyen      | 150    | Miaou  |

**Preuve :** `https://t.me/+k1JCrVoDHN5lYTBk` (sensible à la casse)

---

## Résumé

Ce challenge présente une application qu'il faut rétro-ingénierer afin de découvrir l'adresse d'un des groupes de discussion des malfaiteurs.
Le but est de passer l'étape de vérification du mot de passe afin que l'application affiche un message contenant ladite adresse.

1. **Reconnaissance** - analyse du comportement de l'application
2. **Identification** - trouver la fonction à analyser et déduire comment fournir le mot de passe
3. **Contraintes** - découverte et analyse des contraintes sur le mot de passe
4. **Déduction** - déduction d'un mot de passe valide et affichage de la réponse

---

## Étape 1 : Reconnaissance

### Pré-analyse

Le fichier est un binaire Linux sans dépendance ni protection particulière :

- `file broker_tool` : `ELF 64-bit LSB pie executable, x86-64`,
- `ldd broker_tool` : pas de dépendance dynamique,
- `checksec broker_tool` (de `pwntools`) : portable exécutable sans canary.

### Découverte

Le logiciel semble être un outil en ligne de commande pour faire remonter des informations aux data brokers.
Il y a 4 menus :

- 3 menus demandent des informations, puis les valident et terminent en erreur,
- le menu `Bug Report` requiert un mot de passe pour se débloquer,
- des choix invalides affichent `invalid menu`, mais peuvent être une piste à explorer pour créer un buffer overflow par exemple.

Nous verrons lesquels des menus sont intéressants, mais de prime abord le menu `Bug Report` attire l'œil avec sa demande de mot de passe.

### Décompilation

J'utilise radare2 dans ce write-up pour son affichage textuel facile à copier/coller dans ce document, mais j'ai également utilisé Ghidra, dont j'ai plus l'habitude du décompilateur, et d'autres outils donneront des résultats similaires.

Une première analyse du fichier donne environ 650 fonctions, ce qui est énorme pour un challenge CTF :

```
$ r2 -e bin.relocs.apply=true broker_tool
[0x00017c10]> aaa
[...]
[0x00017c10]> aflc
648
```

Étant donné le contexte d'un challenge CTF, ce grand nombre de fonctions n'est pas dû à la complexité de l'exécutable.
Ceci indique plutôt que la compilation a tiré beaucoup de dépendances, et est un indice que le langage utilisé a une bibliothèque standard fournie dont beaucoup de types sont utilisés.

Avec la commande r2 `afl`, on peut regarder quelques noms de fonctions :

```
[0x00017c10]> afl
sym._ZN4core3ptr238drop_in_place_LT_alloc..boxed..convert.._LT_impl_u20_core..convert..From_LT_alloc..string..String_GT__u20_for_u
sym.core::fmt::Write::write_char::h3f2a830029454542
sym.core::num::__impl_u64_::from_ascii_radix::h72ad40eebc575d6a
```

Les noms comme `_ZN4core3ptr238drop` peuvent indiquer du [name mangling](https://en.wikipedia.org/wiki/Name_mangling), technique utilisée par exemple en C++ pour donner un nom unique dans le binaire à des fonctions ou méthodes surchargées.
On distingue quelle variante de la fonction ou méthode doit être appelée en ajoutant à la compilation ces indications (ici entre autres `_ZN4`).

Un dernier indice est la présence de noms en `rust` :

```
[0x00017c10]> afl~rust~?
60
[0x00017c10]> afl~rust
0x0001abf0    1      5 sym.std::sys::backtrace::__rust_begin_short_backtrace::ha9a4ced036ee7fbf
0x0001f030    5     93 sym._RNvCskdKJRKLKjqM_7___rustc11___rdl_alloc
[...]
```

**On peut conclure qu'il s'agit d'un programme écrit en Rust.**

## Étape 2 : Identification

### Point d'entrée

`r2` ou Ghidra nous mènent au point d'entrée du programme, qui lui-même appelle une fonction `main` :

```
[0x00017c10]> pdf
            ;-- section..text:
            ;-- segment.LOAD1:
            ;-- _start:
            ;-- rip:
┌ 33: entry0 (int64_t arg3);
│           ; arg int64_t arg3 @ rdx
│           0x00017c10      31ed           xor ebp, ebp                ; [15] -r-x section size 295792 named .text
│           0x00017c12      4989d1         mov r9, rdx                 ; arg3
│           0x00017c15      5e             pop rsi
│           0x00017c16      4889e2         mov rdx, rsp
│           0x00017c19      4883e4f0       and rsp, 0xfffffffffffffff0
│           0x00017c1d      50             push rax
│           0x00017c1e      54             push rsp
│           0x00017c1f      4531c0         xor r8d, r8d
│           0x00017c22      31c9           xor ecx, ecx
│           0x00017c24      488d3d852f..   lea rdi, [main]             ; 0x1abb0 ; "PH\x89\xf1Hc\xd7H\x8d\x052\xf2\xff\xffH\x89\x04$H\x8d5\xcfh\x04"
└           0x00017c2b      ff15bfc40400   call qword [reloc.__libc_start_main] ; [0x640f0:8]=0
```

On voit le nom du symbole `main` à l'avant-dernière ligne.
La dernière ligne appelle la fonction de la bibliothèque standard du C qui passera l'exécution à notre `main`.
Dans cette fonction `main`, on trouve un appel à une autre fonction :

```
[0x00017c10]> s main
[0x0001abb0]> pdf
            ; DATA XREF from entry0 @ 0x17c24(r)
┌ 39: int main (int argc, char **argv);
│           ; arg int argc @ rdi
│           ; arg char **argv @ rsi
│           0x0001abb0      50             push rax
│           0x0001abb1      4889f1         mov rcx, rsi                ; argv
│           0x0001abb4      4863d7         movsxd rdx, edi             ; argc
│           0x0001abb7      488d0532f2..   lea rax, [sym.ToujoursEnVente::main::h8160fd29997b4e84] ; 0x19df0 ; "UAWAVAUATSH\x83\xecHI\xbe"
│           0x0001abbe      48890424       mov qword [rsp], rax
│           0x0001abc2      488d35cf68..   lea rsi, obj.anon.c0634bec73b0d960eab5b8b643e01a77.2.llvm.13903541842468237913 ; 0x61498
│           0x0001abc9      4889e7         mov rdi, rsp
│           0x0001abcc      4531c0         xor r8d, r8d
│           0x0001abcf      ff153b960400   call qword [0x00064210]     ; [0x64210:8]=0
│           0x0001abd5      59             pop rcx
└           0x0001abd6      c3             ret
```

Sans rentrer dans les détails, on se doute que la fonction `sym.ToujoursEnVente::main::h8160fd29997b4e84` sera appelée :

```
[0x0001abb0]> s sym.ToujoursEnVente::main::h8160fd29997b4e84
[0x00019df0]> pdf
ERROR: Linear size differs too much from the bbsum, please use pdr instead
```

Ici, `r2` s'étouffe sur cette fonction.
On peut tenter une décompilation, mais on est face à une grande fonction (293 lignes de pseudo-code) :

```
[0x00019df0]> pdc~?
293
```

Et `r2` ne trouve pas les sous-fonctions appelées par notre fonction dans ce cas (commande `axf`).

Côté Ghidra, on n'est pas beaucoup mieux, avec un pseudo-code illisible pour cette fonction et des variables locales en triples pointeurs.
En revanche, Ghidra trouve les sous-fonctions.

### Fonctions du challenge

On prend un peu de recul.
Il y a plusieurs difficultés introduites par le Rust :

1. beaucoup de fonctions, dont 99,8% ne font pas partie du challenge,
2. l'inlining et la réutilisation des variables rendent plus confuse la décompilation,
3. certaines habitudes du compilateur n'aident pas `r2`.

En revanche, certaines aides sont restées, comme les noms des fonctions et méthodes.

Pour le point 1, on remarque avec le `main` que certaines fonctions ont un préfixe `ToujoursEnVente` :

```
[...]> afl~Toujours
0x00018150   20    670 sym.ToujoursEnVente::bug_report::ha164bc6b6a20cb56
0x00018ec0   30    714 sym.ToujoursEnVente::validate_password::h4bce2367aede5e60
0x0001a120   23    569 sym.ToujoursEnVente::decrypt::hce575bdba5c87435
0x00018490   27   1030 sym.ToujoursEnVente::validation::h5fee5d39219e2b44
0x000188d0    6    715 sym.ToujoursEnVente::submit_mails::h7a1c27fdddfe3653
0x0001a380   24    532 sym.ToujoursEnVente::read_line::hc6fc04344a13d2e3
0x00018bc0    7    768 sym.ToujoursEnVente::submit_leaked_doc::hc1f09e778aa6dce9
0x0001a8c0   26    614 sym.__ToujoursEnVente::XoredLiteral_as_core::fmt::Display_::fmt::h07c9e71e0006887c
0x00019b10    7    725 sym.ToujoursEnVente::submit_leaked_photo::hcfe760dfe01ec068
0x00019df0   46    799 sym.ToujoursEnVente::main::h8160fd29997b4e84
```

L'organisation du code en Rust pousse à avoir une organisation en "crate", ici `ToujoursEnVente`.
Tout le code réalisé pour le challenge est donc dans ces 10 fonctions.

D'après les noms des fonctions et le menu, on peut supposer que `main` appelle `submit_mails`, `submit_leaked_photo`, `submit_leaked_doc`, et `bug_report`.
La fonction `read_line` est la fonction qui attend que l'utilisateur appuie sur `Entrée` et renvoie le contenu de la ligne.

### Analyse dynamique

Pour s'aider dans l'analyse, on peut appuyer l'analyse statique avec une analyse dynamique.
On peut utiliser le debugger de radare2 ou un autre debugger comme `gdb` :

```
$ gdb broker_tool
```

Avec le debugger, on peut confirmer que les fonctions `submit_*` correspondent aux 3 premiers menus et la fonction `bug_report` au dernier :

```
db-peda$ b ToujoursEnVente::submit_mails::h7a1c27fdddfe3653 
Breakpoint 1 at 0x188d0
gdb-peda$ r

...
Breakpoint 1, 0x000055555556c8d0 in ToujoursEnVente::submit_mails::h7a1c27fdddfe3653 ()
gdb-peda$ bt
#0  0x000055555556c8d0 in ToujoursEnVente::submit_mails::h7a1c27fdddfe3653 ()
#1  0x000055555556dfd8 in ToujoursEnVente::main::h8160fd29997b4e84 ()
```

`r` lance le programme et, en jouant avec les menus, on arrive au breakpoint.

On constate que les 3 menus `submit_` appellent `validation` qui, semble-t-il, ne fait rien.
Elle n'appelle que `_print` et `sleep`.

Il reste donc 4 fonctions à analyser :

- `bug_report` qui est associée au menu 4,
- `validate_password` qui est appelée par `bug_report`,
- `decrypt` qui est appelée par `bug_report`,
- `XoredLiteral::fmt` appelée par `validate_password`
  (cette fonction est la conséquence d'un appel Rust à la méthode `to_string` sur le type `XoredLiteral`).

Comme ces 4 fonctions sont liées à `bug_report`, commençons par cette fonction.

### Fonctionnement de `bug_report`

En utilisant `r2` :

```
[0x____]> s sym.ToujoursEnVente::bug_report::ha164bc6b6a20cb56
[0x____]> pdf
[...]
|           0x00018159      4881ecb000..   sub rsp, 0xb0
│           0x00018160      488d353917..   lea rsi, str.ADMIN_PWDlibrary_std_src_.._.._backtrace_src_symbolize_gimli_elf.rs ; 0x98a0 ; "ADMIN_PWDlibrary/std/src/../../backtrace/src/symbolize/gimli/elf.rs"
│           0x00018167      488d7c2460     lea rdi, [var_60h]
│           0x0001816c      ba09000000     mov edx, 9
│           0x00018171      ff1509c00400   call qword [sym.std::env::_var::hedc9fcdb7326c51f] ; [0x64180:8]=0x2c870 case.0x5998d.4
[...]
```

La première étape est un appel à `std::env::_var` dont on peut deviner, en utilisant la [doc Rust de `std::env`](https://doc.rust-lang.org/std/env/index.html), qu'il s'agit de récupérer la valeur d'une variable d'environnement.
Le texte le plus proche indique `ADMIN_PWD` suivi d'autre texte.
On peut ignorer le reste du texte car `mov edx, 9` (avant le `call`) indique de n'utiliser que les 9 premiers caractères de cette chaîne, soit `ADMIN_PWD`.

On voit ici que le compilateur Rust concatène toutes les chaînes de texte, contrairement à d'autres langages comme le C où les chaînes de texte sont généralement suivies de `\0` pour les délimiter, ce qui facilite l'analyse statique du binaire.
En Rust, en simplifiant grandement, le type `str` en mémoire est un couple (pointeur vers la donnée, nombre de caractères).
Lors de la compilation des chaînes de texte constantes (écrites telles quelles dans le code source), le compilateur rassemble les chaînes ensemble et génère du code en dur pour gérer séparément le pointeur vers le contenu et leur taille (ici `mov edx, 9`).
Ceci rend plus difficiles les analyses du binaire, et on ne peut pas toujours remonter aux utilisations de ces chaînes, comme on va le voir assez rapidement.

En relançant l'exécutable avec la variable d'environnement en question, le message change :

```
$ ADMIN_PWD= ./broker_tool
[...]
Choose: 4

the given password is too short
invalid password: cannot decrypt message
```

On voit apparaître une première contrainte sur le mot de passe.
La chaîne de texte `the given password is too short`, bien que présente dans le binaire, n'est pas analysée par Ghidra comme une string et il est difficile de trouver d'autres messages similaires.

L'analyse statique du code de `bug_report` montre que si on passe la fonction `validate_password`, alors la fonction `decrypt` est appelée.

### Aperçu de `decrypt`

Avant d'aller plus loin dans `validate_password`, voyons si on peut prendre un raccourci et directement comprendre `decrypt` pour éviter d'avoir à trouver un mot de passe valide.

Le compilateur Rust fait ici office d'obfuscateur en ayant inliné certaines sous-fonctions dans cette région du code.
On se retrouve avec deux fonctions imposantes et difficiles à reverse.

On remarque des appels à la bibliothèque `sha2::sha256` impliquant un hash.
On remarque également que le code génère des kilo-octets de hash (au moins 0x400 = 1024) comme on peut le voir en début de fonction :

```
[...]
        r15 = 0xfffffffffffffc00
[...]
    loc_0x0001a1b0:
        // CODE XREF from ToujoursEnVente::decrypt::hce575bdba5c87435 @ 0x1a1c4(x)
        rdi = r14     // int64_t arg1
        sym.__core::iter::adapters::zip::Zip_A_B__as_core::iter::adapters::zip::ZipImpl_A_B__::next::h5117f180eb4bc210  () // _<core::iter::adapters::zip::Zip<A,B> as core::iter::adapters::zip::ZipImpl<A,B>>::next::h5117f180eb4bc210 // sym.__core::iter::adapters::zip::Zip_A_B__as_core::iter::adapters::zip::ZipImpl_A_B__::next::h5117f180eb4bc210(0x0, 0x0)
        var = rax & rax
        if  (!var) goto loc_0x1a323 // likely
        goto loc_0x0001a1c1;
        goto loc_0x0001a1c1;
        return rax;
    loc_0x0001a1c1:
        r15+
        if  (var) goto loc_0x1a1b0 // likely
```

On voit ici qu'on incrémente `r15` et appelle `next::h5117f180eb4bc210` jusqu'à obtenir `r15==0`, soit 0x400 fois, soit 1024 fois.
On peut supposer qu'on est sur un [One-Time Pad](https://en.wikipedia.org/wiki/One-time_pad) : on génère un OTP à partir du mot de passe, puis on XOR chaque octet obtenu avec un payload chiffré contenu dans l'exécutable.

La clef XOR est donc aussi longue que le contenu chiffré, et on ne peut pas la deviner.
Elle n'est pas contenue dans le binaire mais générée à la volée à partir d'un argument de la fonction et de sha256 successifs.
Il faut donc obtenir le mot de passe utilisé pour générer l'OTP de déchiffrement.
On peut tenter un bruteforce sur le mot de passe, mais la fonction `validate_password` devrait nous aiguiller sur le bon chemin...

## Étape 3 : Contraintes

### Premières contraintes

Une première analyse des chaînes de texte ne donne pas beaucoup plus d'indices.
Une analyse statique avec la décompilation donne également des difficultés, car la fonction a été une nouvelle fois mise à plat par la compilation Rust.

On peut cependant progresser manuellement en tentant de nouveaux mots de passe et déduire, par les erreurs affichées, ce qu'on doit faire, puis confirmer par la décompilation les étapes à franchir :

- longueur au minimum 16,
- longueur multiple de 3 et de 4, donc multiple de 12,
- la somme des valeurs des lettres est un multiple de 5
  (en tests manuels, il suffira de remplacer la dernière lettre par sa suivante dans l'alphabet jusqu'à passer la contrainte),
- il faut au moins une lettre en minuscule et une en majuscule
  (deuxième contrainte qu'on obtient avec `strings broker_tool | grep letter`).

À partir d'ici, on ne retrouve plus les messages d'erreur dans le binaire, mais ils sont affichés dans la console via `XoredLiteral::fmt`.
On retrouve également le code qui calcule ces conditions dans le reverse de `validate_password` (0x01952a dans les adresses présentées par `r2` jusque-là) :

- il faut au moins un caractère spécial (dont le code hexadécimal est entre `0x21` et `0x7e` et qui n'est pas une lettre ou un chiffre),
- il faut mettre un `!` au milieu du mot de passe,
- il faut au moins 9 lettres minuscules et 5 majuscules,
- il doit y avoir au moins 7 couples de caractères égaux séparés par un autre caractère (exemple `X.X`),
- il doit y avoir au moins 6 couples de caractères dont le deuxième est le suivant du premier (`ABC` compte pour deux).

En passant toutes ces étapes, il ne reste qu'un message d'erreur : `invalid password: cannot decrypt message`.
Il reste donc (au moins) une dernière étape de contrainte silencieuse qui ne précise pas en message d'erreur ce qu'elle attend.

### Dernière contrainte

La dernière contrainte est la plus difficile, car elle n'est pas annoncée en cas d'échec.
Il faut donc comprendre un minimum comment la fonction se comporte :

- une structure allouée sur la stack gère l'avancement de l'analyse,
- la boucle principale itère sur le mot de passe et modifie cette structure (0x019530),
- une série de tests réagit aux différents flags levés pendant l'analyse (0x0196b0).

On constate, en dernier test dans la boucle qui itère sur le mot de passe, la condition : `password[i] == target[k] ^ (0xC0 + k)`.
`target` pointe vers une zone mémoire, et `k` avance de 1 lorsque l'on réussit cette condition.
Le test est passé tant que `k < 0x1a`, c'est donc la taille du secret.

On peut récupérer la donnée en mémoire et appliquer le XOR manuellement :

```
  mem b0 a0 b1 b7 a1 e4 b5 e7 ac fa ea a5 ad fa fa ef e3 f1 b0 e7 b7 e1 ba bf ec ac
  XOR c0 c1 c2 c3 c4 c5 c6 c7 c8 c9 ca cb cc cd ce cf d0 d1 d2 d3 d4 d5 d6 d7 d8 d9
    = 70 61 73 74 65 21 73 20 64 33 20 6e 61 37 34 20 33 20 62 34 63 34 6c 68 34 75
ASCII  p  a  s  t  e  !  s     d  3     n  a  7  4     3     b  4  c  4  l  h  4  u
```

Le secret à inclure dans le mot de passe est donc `paste!s d3 na74 3 b4c4lh4u`.

Ces lettres doivent apparaître dans l'ordre dans le mot de passe, mais pas nécessairement les unes à la suite des autres.

## Étape 4 : Déductions

De toutes les contraintes, on déduit un mot de passe valide, comme par exemple `ABCDEFGHIJKL!p!a!s!t!e!s! !d!3! na74 3 b4c4lh4u9`.
On obtient le texte déchiffré par l'application :

```
Hey! So you pretend you are an admin and found a bug?

This is a serious software developed by serious roxxors from the 1337 team of brokers from Fantasmas De Redes!!
After all, we have found so much value in what others thought were useless!
We will track data from closed companies (maybe you worked there?) and extract value using their client listings.
Completed with dark web data, like people that wouldn't pay ransoms to recover their encrypted data
(all of this because they could not prevent 1337 hackers from entering their poorly protected systems).
And we did all this by our selves.

So.

Do you REALLY, *REALLY* think that you found a bug?
Prepare everything because we will want details and proofs that their IS a bug...
Be able to reproduce the bug, know what YOU did or what is wrong with YOUR machine (poor CPU architecture? not enough RAM? slow connection?).

WE WANT TO SEE IT.
If you dare showing it to us, just come to https://t.me/+k1JCrVoDHN5lYTBk and we will see.

Adeus!
```

## Résultat

La solution de notre challenge est l'url du groupe Telegram des `FakeBrokers`.

✅ **Preuve :** `https://t.me/+k1JCrVoDHN5lYTBk`
