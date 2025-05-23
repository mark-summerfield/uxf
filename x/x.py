#!/usr/bin/env python3

B = -1 # list begin
E = -2 # list end


def main():
    tokens = [B, 1, 2, B, 30, 40, B, 500, B, E, E, 70, B, 800, 900, E,
              1000, E, 1100, E]
    assert tokens.count(B) == tokens.count(E)
    expected = [1, 2, [30, 40, [500, []], 70, [800, 900], 1000], 1100]
    actual = parse(tokens)
    print('Actual  ', actual)
    print('Expected', expected)
    assert actual == expected


# My algorithm (20 LOC)
def parse(tokens):
    value = None
    stack = []
    for token in tokens:
        if token == B:
            lst = []
            if value is None:
                value = lst
                stack.append(value)
            else:
                top = stack[-1]
                if isinstance(top, list):
                    top.append(lst) # add new list to current list
                stack.append(lst) # make new list the current list
        elif token == E:
            stack.pop()
        else:
            top = stack[-1]
            top.append(token)
    return value


if __name__ == '__main__':
    main()
