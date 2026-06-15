#!/usr/bin/env python3

"""
Script that prepares the binary blob file that will be compiled and embedded in the rust binary.
It is prefixed and postfixed by random data.
It contains the FLAG, which is set in this script.

You have to rerun this script when any of the parameters change
 then recompile the app (cargo build --release).
"""

from hashlib import sha256
from itertools import count
import random


# The final flag is the URL to be found
# It is embedded in the clear text that will be encrypted and embedded in the app to be reversed
FLAG = b'https://t.me/+k1JCrVoDHN5lYTBk'

# This secret is used to derive the encryption/decryption key
# It is obfuscated in the app to be reversed
# A little harder to change:
# - generate a new one with `echo -n 'n3w_s3cR37' | ./xor.py C0 -s` (you can change the key C0),
# - copy the generated Rust code and paste it in src/main.rs (where SECRET is currently defined),
# - adjust the test cases:
#   - last two cases of tests_pwd_validator::examples,
#   - tests_pwd_validator::direct_secret,
#   - tests_pwd_validator::a_good_password,
#   - tests_validation::a_good_password,
#   - tests that decrypts the payload (not done yet).
RUST_SECRET = b'paste!s d3 na74 3 b4c4lh4u'

# The length of the random garbage before the text
PREFIX_LEN = 1024

# The text which contains PLACEHOLDER_FLAG and that we will encrypt+embed in the app
SOURCE_FNAME = './text_bugreport'

# The output of this script
DEST_FNAME = './encrypted_bugreport'


# Kind of CTR mode
def iter_otp(iv):
    for i in count(0):
        yield from sha256(iv+i.to_bytes(4, 'big')).digest()


if __name__ == '__main__':
    # Read the file, place the flag
    with open(SOURCE_FNAME, 'rb') as f:
        payload = f.read()
        assert b'PLACEHOLDER_FLAG' in payload
        payload = payload.replace(b'PLACEHOLDER_FLAG', FLAG)

    # Encode the size of the clear text as an uint16 before the text,
    #  so that the decoding process can remove the padding
    payload = len(payload).to_bytes(2, 'little') + payload

    # Prefix with 1k random, then pad up to next 512
    # We don't really care about the quality of the randomness,
    #  it is just to dissuade people from bruteforcing the RUST_SECRET
    #  and make it a little harder if they try to not use the app to display the message.
    payload = random.randbytes(PREFIX_LEN) + payload
    target_len = (len(payload)//512+1)*512  # (we still pad with 512 bytes if len%512 == 0)
    payload = payload + random.randbytes(target_len-len(payload))

    # Encrypt
    # 1. create our stream of XOR (OTPad)
    # 2. xor
    encrypted = bytearray()
    for c,k in zip(payload, iter_otp(RUST_SECRET)):
        encrypted.append(c^k)

    # Save
    with open(DEST_FNAME, 'wb') as f:
        f.write(encrypted)
    print('saved to', DEST_FNAME)

    # Also save the OTPad alone for manual verification purposes
    #  (you should not save the OTPad xD)
    with open('otp', 'wb') as f:
        f.write(bytes(k for k,_ in zip(iter_otp(RUST_SECRET), range(2048))))
    print('saved otpad')
