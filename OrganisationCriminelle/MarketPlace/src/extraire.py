import cv2
import numpy as np
import pytesseract
from PIL import Image

# Si tesseract n'est pas dans le PATH (Windows)
# pytesseract.pytesseract.tesseract_cmd = r"C:\Program Files\Tesseract-OCR\tesseract.exe"

IMAGE_PATH = "image.png"

# 1. Charger l'image
img = cv2.imread(IMAGE_PATH)

# 2. Convertir en espace HSV (plus fiable pour la couleur)
hsv = cv2.cvtColor(img, cv2.COLOR_BGR2HSV)

# 3. Définir une plage de gris
# Le gris a une saturation faible
lower_gray = np.array([0, 0, 50])
upper_gray = np.array([180, 50, 220])

mask = cv2.inRange(hsv, lower_gray, upper_gray)

# 4. Nettoyage du masque
kernel = np.ones((2, 2), np.uint8)
mask = cv2.morphologyEx(mask, cv2.MORPH_OPEN, kernel)

# 5. Créer une image ne contenant que le texte gris
gray_text_img = cv2.bitwise_and(img, img, mask=mask)

# 6. Conversion pour OCR
gray = cv2.cvtColor(gray_text_img, cv2.COLOR_BGR2GRAY)
gray = cv2.threshold(gray, 150, 255, cv2.THRESH_BINARY)[1]

# 7. OCR ligne par ligne
custom_config = r'--oem 3 --psm 6'
text = pytesseract.image_to_string(gray, config=custom_config, lang="eng")

# 8. Mise sur une seule ligne de tous les caractères

one_line_text = "".join(
    char for char in text
    if char.isalnum() or char in ".&"
)

# 9. Correction du résultat en remplaçan & par g
one_line_text = one_line_text.replace("&", "g")

# 10. Remplacement du caractère coupé en bord par y
one_line_text = one_line_text.replace("n", "?n", 1)

# 11. Suppression de l'allucination de l'algo d l'allucination
one_line_text = one_line_text.replace("ad", "", 1)

# 12. Conversion en miniscule de tous les caractères
one_line_text = "".join(c for c in one_line_text if not c.isupper())

print(one_line_text)
