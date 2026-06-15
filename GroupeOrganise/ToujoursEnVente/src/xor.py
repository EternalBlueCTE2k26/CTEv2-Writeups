#!/usr/bin/env python3

"""
Small utility that xor-es what it finds on stdin with the key given by parameters,
and outputs in various formats.
"""

import argparse
from itertools import cycle
import sys


def escape_buf(buf):
    """Manually escape buf because repr(buf) gives unpredictable results with changing quotes"""
    def _repr(c):
        if c == 0x08:
            return r'\b'
        elif c == 0x09:
            return r'\t'
        elif c == 0x0a:
            return r'\n'
        elif c == 0x0d:
            return r'\r'
        elif c == 0x22:
            return r'\"'
        elif c == 0x5c:
            return r'\\'
        elif 0x20 <= c < 0x80:
            return chr(c)
        else:
            return rf'\x{c:02x}'
    return ''.join(map(_repr, buf))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='xor stdin to stdout')
    parser.add_argument('key', help='key given as an hex string')
    grp = parser.add_mutually_exclusive_group()
    grp.add_argument('--repr', '-r', action='store_true', help='use repr(bytes) instead of raw binary')
    grp.add_argument('--hex', '-x', action='store_true', help='use bytes.hex() instead of raw binary')
    grp.add_argument('--rust', '-u', action='store_true', help='format for XoredLiteral')
    grp.add_argument('--rust-secret', '-s', action='store_true', help='format for XoredLiteral, but add index to key')
    args = parser.parse_args()

    key = bytes.fromhex(args.key)  # Asserts key format before reading anything
    msg = sys.stdin.buffer.read()  # Read all as bytes

    if args.rust_secret:
        assert len(key) == 1
        # In case of our special secret, we want to XOR with (key+i) where i is the index of the message
        # The simplest is to prepare the key for that and keep the rest of the code
        key = bytes(key[0]+i for i in range(len(msg)))
    # cycle(key) makes it easier to handle any length key
    buf = bytes(c^k for c,k in zip(msg, cycle(key)))

    if args.repr:
        print(repr(buf))
    elif args.hex:
        print(buf.hex())
    elif args.rust:
        assert len(key) == 1
        print(f'// "{msg.decode()}"')
        print(f'const ERR_/*TODO*/: XoredLiteral = XoredLiteral::from_xored(0x{key[0]:02x}, b"{escape_buf(buf)}");')
    if args.rust_secret:
        assert len(key) == len(msg)
        print(f'// "{msg.decode()}" but XORed with (key+i) for pwd[i]')
        print(f'const SECRET: XoredLiteral = XoredLiteral::from_xored(0x{key[0]:02x}, b"{escape_buf(buf)}");')
    else:
        sys.stdout.buffer.write(buf)
