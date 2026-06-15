#!/usr/bin/env python3

"""
Encryption script for the black market challenge.
The goal is to swap letters so that players have to guess contextually with the conversation.
Reads on stdin, output on stdout.

Usage:
    ./encrypt.py < source > dest 2> >(tee log >&2)
or:
    cat source | ./encrypt.py > dest 2> >(tee log >&2)
2> redirects to >(tee log >&2) which prints to the log and redirects its stdout to our stderr
(the log contains the decryption keys)

The swap should be deterministic, at least on some similar python/linux,
hence multiple calls to this script should produce the same result.
"""

import string
import sys
import random


# Print to stderr
eprint = lambda *args, **kwargs: print(*args, **kwargs, file=sys.stderr)


if __name__ == "__main__":
    # First step, choose a permutation
    # The swap is done letter by letter, hence a dict is good enough

    # Make a reproducible random swap
    #  (at least on the same machine, but should be ok with the same python major version)
    random.seed(b'Vive EB!!!')

    # Shuffle letters and see if we obtained something good enough
    letters = list(string.ascii_lowercase)
    random.shuffle(letters)
    swaps = {}
    for c,e in zip(string.ascii_lowercase, letters):
        assert c != e, f'not correctly shuffled, change the seed\n input: {string.ascii_lowercase}\noutput: {"".join(letters)}'
        swaps[c] = e
        swaps[c.upper()] = e.upper()

    # Second step, swap letters from the text, so read the text
    #with open('conversation_pt') as f:
    #    text = f.read()
    text = sys.stdin.read()

    # Remove accentuations (don't install a lib for this, we only have a few of them)
    for c,d in ('àa', 'áa', 'ãa', 'çc', 'ÉE', 'ée', 'êe', 'íi', 'ÓO', 'óo', 'õo', 'úu'):
        text = text.replace(c,d)

    # Show remaining non-ASCII chars (accents)
    for c in text:
        if ord(c) >= 128:
            eprint('remaining non-ASCII', c)
    assert text.encode('ascii')  # Don't encrypt if there are still such chars

    # And encrypt
    # We can't just swap with .replace like before, as encrypted letters will be mixed with not-yet encrypted ones
    # So we just replace on the fly, and .get ensures that we keep unswapped chars (hopefully symbols only)
    text = ''.join(swaps.get(c, c) for c in text)

    print(text)
    eprint('encryption ok')

    # Print solution to log to be able to check the solution
    deswap = {e:c for c,e in swaps.items()}
    eprint('to decrypt, swap:', string.ascii_lowercase)
    eprint('            with:', ''.join(deswap[e] for e in string.ascii_lowercase))
