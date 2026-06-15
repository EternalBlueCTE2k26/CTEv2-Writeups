# Challenge : Jeu de dupes

## Informations du challenge

| Catégorie | Difficulté | Points | Auteur |
|-----------|------------|--------|--------|
| Osint | Moyen | 200 | B3cha |

**Preuve :** `Eternal-Sunshine_Tom-Cruise_Rome`

---

## Résumé

Ce challenge nécessite de retrouver le site de jeu ciné quiz, puis d'utiliser les informations de naissance de Mélanie trouvées lors du challenge `Mot de passe faible`.
En participant au jeu, on retrouve les informations attendues par le flag.

### Identifier le site du jeu

Le premier indice se trouve dans le nom du challenge `Jeu de dupes` : ceci donne comme indication que nous recherchons une nouvelle technique d'escroquerie pour récolter des données personnelles via un **JEU**.
Il faut donc découvrir ce jeu.
En rapport avec notre enquête, il y a nécessairement un personnage de l'enquête qui s'est fait avoir par ce jeu.
En recherchant sur les réseaux sociaux de **Mélanie**, sur son compte `BlueSky`, une réponse faite par Mélanie attire l'attention :
https://bsky.app/profile/lefevremelanie.bsky.social/post/3mjvuhchxjk2d

![](images/reponse-bluesky.png)

Sur ce post, Mélanie fait état d'un jeu accessible à l'url suivante : `https://www.jeucinequiz.online/`.
Ce jeu propose d'essayer de deviner :
1. Quel film est sorti l'année de votre naissance ?
2. Quelle célébrité du cinéma est née le même mois que vous ?
3. Quelle capitale a le même numéro que votre jour de naissance ?

Les escrocs utilisent ce type de jeux ludiques qu'ils postent sur les réseaux sociaux : entre deux visionnages de vidéos, votre vigilance baisse et vous saisissez sans en être vraiment conscient votre date de naissance complète. Dans le cas de Mélanie, c'est aussi son mot de passe (à ne surtout pas faire).

**Restez vigilant et ne livrez jamais vos informations personnelles par la ruse (même un jeu d'apparence inoffensive).**

Pour avoir un maximum de victimes, le jeu doit être simple, ludique et ne doit pas dépasser 3 clics.
Le visuel du site de jeu est le suivant :

![](images/jeu-cine-quizz.png)

## Renseigner les informations de Mélanie

À partir de la date de naissance de Mélanie, le 05 juillet 2004, trouvée lors du challenge `Mot de passe faible`, retournez les cartes qui correspondent aux informations personnelles de Mélanie.

Pour l'année :

![](images/annee.png)

Pour le mois :

![](images/m.png)

Enfin pour le jour :

![](images/jour.png)

En fin de jeu, la page principale récapitule vos informations ; elle vous incite à envoyer vos données pour pouvoir être inscrit sur un pseudo tirage au sort. Bien évidemment, c'est très tentant, mais il n'y a aucun tirage au sort et il n'y a que des perdants.

![](images/Résultats.png)

Cette photo récapitule les réponses attendues de notre flag.

### Point de confirmation

En avançant dans l'enquête, nous trouvons sur le groupe Facebook de `Fantasmas-de-redes` un post de Miguel faisant la promotion de ce jeu sympa.

![](images/post-miguel-FdR.png)

---

## Résultat

La solution de notre challenge est constituée des trois réponses suivantes, séparées par un underscore _ :
1. Le film associé à l'année de naissance de Mélanie : **Eternal-Sunshine**
2. Le nom de l'acteur né le même mois que Mélanie : **Tom-Cruise**
3. Le numéro de la ville associée au jour d'anniversaire de Mélanie : **Rome**

✅ **Preuve :** `Eternal-Sunshine_Tom-Cruise_Rome`
