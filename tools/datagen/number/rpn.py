#!/bin/bash
import argparse
from functools import partial

from utils import is_num


ops = ["+", "-", "*", "/", "%", "<<", ">>", "&", "|", "^", "~"]


def argparser():
    parser = argparse.ArgumentParser()
    parser.add_argument("equation", type=str, nargs="*")

    args = parser.parse_args()
    return args


def operation(op, stack):
    if op != "~":
        a, b = stack.pop(), stack.pop()
    else:
        a = stack.pop()
        b = 0

    match op:
        case '+':
            return a + b
        case '-':
            return a - b
        case '*':
            return a * b
        case '/':
            return a / b
        case '%':
            return a % b
        case '<<':
            return a << b
        case '>>':
            return a >> b
        case '&':
            return a & b
        case '|':
            return a | b
        case '^':
            return a ^ b
        case '~':
            return -a


def rpn(equation):
    stack = []
    partial_results = []

    for token in equation:
        if is_num(token):
            stack.append(int(token, 0))
        elif token in ops:
            a, b = stack[0], stack[1]
            r = operation(token, stack)
            partial_results.append((a, token, b, r))
            stack.append(r)
    
    if len(stack) > 1:
        raise Exception("Invalid equation")
    else:
        return stack.pop(), partial_results


def main():
    args = argparser()
    if len(args.equation):
        print(rpn(args.equation))
    else:
        eq = input("Enter equation: ")
        result = rpn(eq.split(" "))
        print(f"{eq} is {result}")

if __name__ == "__main__":
    main()