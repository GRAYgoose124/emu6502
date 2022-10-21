#!/bin/python3
import argparse
import columnize

from num_tables import *
from ops_data import *


def parser():
    parser = argparse.ArgumentParser()
    parser.add_argument("op", type=str, nargs="?")

    args = parser.parse_args()
    if args.op is None:
        print("No op given")
        exit()

    return args


def main():
    args = parser()

    if args.op.isalpha():
        codes = []
        for n, v in valid_names:
            if args.op.upper() == n:
                codes.append(h(v))
                
        print(args.op, " ".join(codes))





if __name__ == "__main__":
    main()
