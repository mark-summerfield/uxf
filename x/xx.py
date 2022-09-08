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


# rust forum's 2e71828's algorithm (19 LOC)
def parse(tokens):
    value = None
    stack = []
    for token in tokens:
        if value is not None:
            element = value
            value = None
            if stack:
                top = stack[-1]
                if isinstance(top, list):
                    top.append(element)
        if token == B:
            stack.append([])
            value = None
        elif token == E:
            value = stack.pop()
        else:
            value = token
    return value


if __name__ == '__main__':
    main()
