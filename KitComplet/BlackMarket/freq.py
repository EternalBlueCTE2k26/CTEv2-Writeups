#!/usr/bin/env python3

from collections import Counter
import sys

if __name__ == '__main__':
    freqs = Counter()
    for line in sys.stdin:
        # chat will be the right hand side of the ':' on each line (or empty ottherwise)
        #  and contain the chat text which we want to use to count letters
        *_, chat = line.strip().partition(':')
        for c in chat:
            freqs[c] += 1
    # Results
    for c,f in freqs.most_common(10):
        print(f'{c} appeared {f} times')
