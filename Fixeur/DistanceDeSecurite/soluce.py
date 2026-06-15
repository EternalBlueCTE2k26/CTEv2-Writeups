import torch
import torch.nn.functional as F
import cma
import time
import requests
import numpy as np
import math
import matplotlib
matplotlib.use('Agg') 
import matplotlib.pyplot as plt

# =====================================================
# CONFIGURATION
# =====================================================
URL = "http://localhost:1985"
token = "ZGVidWc6MTc2ODY4NDc2Nw==.znllPo1hb8qCQMf3I90WQp0MYcrT6unzLANLSo678SE="
DIM = 192
TARGET_COUNT = int(DIM * 1.5) 
REQ_INTERVAL = 0.065 # Légère marge de sécurité
MAX_STREAK = 5 

valid_vectors = []
valid_scores = []
last_request = 0.0

def rate_limited_post(payload):
    global last_request
    now = time.time()
    delta = now - last_request
    if delta < REQ_INTERVAL:
        time.sleep(REQ_INTERVAL - delta)
    last_request = time.time()
    return requests.post(f"{URL}/embedding/", json=payload, headers={"X-Flag": token})

def get_full_data(vec: torch.Tensor):
    """Version robuste : gère les types de retour, les erreurs 400 et 429."""
    v = F.normalize(vec.detach().view(-1), p=2, dim=0)
    try:
        r = rate_limited_post({"embedding": v.tolist()})
        if r.status_code == 429:
            time.sleep(1.0)
            return get_full_data(vec)
        
        data = r.json()
        # On récupère les infos étendues de la nouvelle route
        zone = data.get("zone")
        precision = data.get("precision") # cs_raw
        streak = data.get("streak", 0)
        flag = data.get("flag")
        
        return zone, precision, streak, flag
    except Exception as e:
        return 10, None, 0, None

# =====================================================
# 1. CMA-ES + COLLECTE
# =====================================================
print(f"[*] Initialisation CMA-ES (Dim: {DIM})...")
es = cma.CMAEvolutionStrategy(torch.randn(DIM).tolist(), 0.5)
gen = 0
streak_attained = 0 

while len(valid_vectors) < TARGET_COUNT:
    gen += 1
    solutions = es.ask()
    scores_cma = []
    
    for s in solutions:
        s_t = torch.tensor(s, dtype=torch.float32)
        zone, precision, streak, flag = get_full_data(s_t)
        
        # Le score CMA-ES doit être minimisé (Zone 0 = 0.0)
        val_cma = float(zone) if (zone is not None and zone >= 0) else 10.0
        scores_cma.append(val_cma)
        
        # Collecte pour triangulation : On veut des points en Zone 0
        if zone == 0 and precision is not None:
            v_norm = F.normalize(s_t, p=2, dim=0)
            valid_vectors.append(v_norm.detach())
            valid_scores.append(float(precision))
            
        if flag:
            print(f"🏆 Flag obtenu par chance en exploration : {flag}")
            exit(0)
    
    es.tell(solutions, scores_cma)
    
    # Mise à jour du streak actuel pour le log
    current_best_zone = es.result.fbest
    print(f"Gen {gen} | Best Zone: {current_best_zone} | Collected: {len(valid_vectors)}/{TARGET_COUNT}")

# =====================================================
# 2. ANALYSE LINALG / LSTSQ
# =====================================================
print("[*] Résolution du système linéaire pour trouver le centre exact...")
T = torch.stack(valid_vectors)
S = torch.tensor(valid_scores).unsqueeze(1)
res = torch.linalg.lstsq(T, S)
zero_zone_vec = F.normalize(res.solution.squeeze()[:DIM], p=2, dim=0)

# =====================================================
# 3. VERIFICATION DU CENTRE
# =====================================================
final_zone, final_precision, _, _ = get_full_data(zero_zone_vec)
print(f"[*] Centre calculé - Zone: {final_zone} | Cosine Raw: {final_precision}")

# =====================================================
# 4. FINAL PUSH: Stratégie Haute-Dimension (Générateur)
# =====================================================
print("\n[!] Lancement du Final Push (Générateur dynamique)")

def generate_flag_points(target, num_points=1000, angle_to_target_deg=1.8):
    """
    Générateur de points dans le cône de la Zone 0.
    Utilise la haute dimension pour garantir l'espacement.
    """
    dim = target.shape[0]
    alpha = math.radians(angle_to_target_deg)
    
    history = [] # Pour la vérification de colinéarité locale

    for _ in range(num_points):
        # 1. Direction aléatoire (Exploitation de la dimension 192)
        noise = torch.randn(dim)
        # 2. Orthogonalisation (Gram-Schmidt)
        ortho = F.normalize(noise - torch.dot(noise, target) * target, dim=0)
        # 3. Mélange pour l'angle cible (Zone 0)
        p = math.cos(alpha) * target + math.sin(alpha) * ortho
        p_norm = F.normalize(p, p=2, dim=0)

        # 4. Vérification de sécurité locale avant de yield
        if history:
            prev_matrix = torch.stack(history)
            # Cosinus avec tous les points déjà générés
            cos_sims = torch.abs(torch.mv(prev_matrix, p_norm))
            if torch.any(cos_sims >= 0.99939): # Seuil 2.0°
                continue # On rejette et on régénère si trop proche
        
        history.append(p_norm)
        yield p_norm

# Paramètres
num_hits_target = 1000
target_vec = zero_zone_vec.view(-1).detach()

# Consommation du générateur
point_generator = generate_flag_points(target_vec, num_points=num_hits_target, angle_to_target_deg=1.9)



for i, p in enumerate(point_generator):
    try:
        # Envoi et récupération des scores
        zone, precision, streak, flag = get_full_data(p)
        
        print(f"Hit {i+1}/{num_hits_target} | Streak: {streak} | Zone: {zone} | Cos: {precision:.6f}")
        
        if flag:
            print(f"\n🏆 FLAG TROUVÉ : {flag}")
            break
            
        # Sécurité : si le streak reset à 0 alors qu'on est en Zone 0, 
        # c'est qu'il reste des résidus du streak précédent en mémoire serveur.
        if i > 0 and streak == 1 and zone == 0:
            print("[!] Streak reset détecté par le serveur. Nettoyage de l'historique local...")
            # On pourrait envisager un sleep ou un changement de token ici
            
    except Exception as e:
        print(f"[!] Erreur lors du hit {i+1}: {e}")
        time.sleep(1) # Petite pause en cas d'erreur réseau