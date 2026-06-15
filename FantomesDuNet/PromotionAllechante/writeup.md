# Challenge : Promotion alléchante

## Informations du challenge

| Catégorie | Difficulté | Points | Auteur |
|-----------|------------|--------|--------|
| Osint | Facile | 100 | B3cha |

**Preuve :** variante toutes les 4 heures

---

## Résumé

Ce challenge est un `Canary Token`, les équipes qui sollicitent l'adresse mail **les-influenceurs@jeucinequiz.online**
reçoivent une réponse dynamique qui change toutes les 4 heures.

## Résolution du challenge

Lors du challenge `Affiliation`, les joueurs trouvent dans les archives `WaybackmMchine` une mention à l'adresse mail
de contact des influenceurs pour participer au jeu.

![](images/adresse-de-contacte.png)

En envoyant un mail avec une adresse de vos sockpuppets, un répondeur automatique répond avec un code Promotion
différent :

![](images/mail-repondeur.png)

L'analyse des horodatages des mails reçus ainsi que le moment de soumission du flag permettent de voir si deux teams
utilisent le même code promotion.

Une équipe qui valide le challenge sans avoir envoyé de mail est également suspectée.

Les codes promotions sont sélectionnés avec de faibles distances de Hamming permettant de confondre `l` Lima minuscule
avec `1` unité, ou encore `I` India majuscule.

Des **CONTROLES VITESSE** sont effectués auprès des équipes pour analyser leur méthodes de résolution.

En cas d'opération active, il est impératif d'utiliser un messagerie jetable et bien protéger son identité 💪🏽. 

La Liste des codes valides est la suivante :

```shell
74IFCE4B7A
74lFCE4B7A
74lFCE4b7A
74IFCE4b7A
EF4711B139
EF47llB139
EF4711B13g
EF47llB13g
0D76256733
OD76256733
0D76257633
OD76257633
EF4711I39
EF4711l39
EE4711I39
EE4711l39
6A52DOOC5E
6A52DO0C5E
6A52D0OC5E
6A25DOOC5E
94E451329E
94E45l329E
94E45I329E
94E45I328E
C3I3I0934F
C3I3l0934F
C3l3I0934F
C3I3I0934F
36D2A4C157
36D2A4CI57
36D2A4Cl57
33D2A4Cl57
```

---

## Résultat

La solution de notre challenge est le code promotion dynamique retourné par le répondeur de mail des influenceurs.
 
✅ **Preuve :** variante toutes les 4 heures.
