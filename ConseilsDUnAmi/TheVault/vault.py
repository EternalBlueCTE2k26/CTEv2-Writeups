#!/usr/bin/env python3
"""
Vault script for LeVault challenge.
"""
import getpass
import sys

# Hardcoded secrets
SECRETS = (
    {"id": 1, "name": "Nuclear Launch Code", "value": "1234-5678-9012"},
    {"id": 2, "name": "Swiss Bank Account", "value": "CH93 0000 0000 0000 0000 0"},
    {"id": 3, "name": "Area 51 Gate Key", "value": "A51-KEY-999"},
)

# Encryption Constants
KEY = b"Sup3rS3cr3tK3y"
SALT = b"S4ltY_V4lu3"

# Encrypted Data (Double XOR with Salt)
ENC_PASSWORD = bytes([48, 117, 44, 112, 25, 60, 85, 99])
ENC_BANNER = bytes(
    [
        10,
        32,
        105,
        51,
        67,
        99,
        23,
        119,
        36,
        102,
        20,
        121,
        106,
        124,
        85,
        12,
        123,
        36,
        10,
        122,
        4,
        90,
        68,
        106,
        121,
        86,
        6,
        6,
        102,
        46,
        114,
        50,
        32,
        99,
        115,
        47,
        60,
        74,
        120,
        124,
        106,
        124,
        84,
        104,
        81,
        102,
        114,
        66,
        25,
        124,
        84,
        117,
        119,
        74,
        111,
        68,
        73,
        116,
        97,
        96,
    ]
)


def xor_string(data: bytes, key: bytes) -> bytes:
    """XORs data with a key."""
    return bytes([b ^ key[i % len(key)] for i, b in enumerate(data)])


def decrypt(data: bytes, key: bytes, salt: bytes) -> str:
    """Decrypts data using double XOR (reverse order of encryption)."""
    # Encryption was: (Data ^ Key) ^ Salt
    # Decryption is: (Encrypted ^ Salt) ^ Key
    step1 = xor_string(data, salt)
    decrypted = xor_string(step1, key)
    return decrypted.decode("utf-8")


def main():
    """Main entry point for the vault."""
    print("Welcome to LeVault Secure Storage.")

    try:
        password_input = getpass.getpass("Enter password: ")
    except KeyboardInterrupt:
        print("\nOperation cancelled.")
        sys.exit(1)

    # Decrypt the expected password to compare
    real_password = decrypt(ENC_PASSWORD, KEY, SALT)

    if password_input == real_password:
        # Decrypt and show banner
        banner = decrypt(ENC_BANNER, KEY, SALT)
        print(banner)
        print("-" * 40)
        print("ACCESS GRANTED. DECRYPTING DATA...")
        print("-" * 40)

        for secret in SECRETS:
            print(f"ID: {secret['id']}")
            print(f"Name: {secret['name']}")
            print(f"Value: {secret['value']}")
            print("-" * 20)
    else:
        print("Access Denied.")


if __name__ == "__main__":
    main()
