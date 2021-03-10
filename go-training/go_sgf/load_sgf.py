import pprint

import tatsu
from tatsu import parse

# antlr SGF grammar: https://github.com/antlr/grammars-v4/tree/master/sgf
# translate with: https://tatsu.readthedocs.io/en/stable/antlr.html

if __name__ == '__main__':
    # sgf_grammar = g2e.translate(filename='../resources/sgf.g4', name='SGF')
    # with open('../resources/sgf.ebnf', 'w') as f:
    #     f.write(sgf_grammar)
    with open('../resources/sgf.ebnf') as _:
        sgf_grammar = _.read()

    with open('8.sgf') as _:
        ast = parse(sgf_grammar, _.read())

    tatsu.to_python_sourcecode(sgf_grammar, name=None, filename='toto.py')

    pprint.pprint(ast, indent=2, width=20)
    print()
