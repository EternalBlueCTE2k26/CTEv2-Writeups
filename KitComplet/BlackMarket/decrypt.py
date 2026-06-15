#!/usr/bin/env python3

import sys

if __name__ == '__main__':
    # What we know
    encrypted = 'abcdefghijklmnopqrstuvwxyz'
    decrypted = 'tjgcdsohuxwzlivanmehb$fprk'
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
