## ruby-parser

`ruby-parser` is a Ruby parser for Node.js written in Rust.

## Supported nodes

- [x] Alias
- [x] And
- [x] AndAsgn
- [x] Arg
- [x] Args
- [x] Array
- [x] ArrayPattern
- [x] ArrayPatternWithTail
- [x] BackRef
- [x] Begin
- [x] Block
- [x] Blockarg
- [x] BlockPass
- [x] Break
- [x] Case
- [x] CaseMatch
- [x] Casgn
- [x] Cbase
- [x] Class
- [x] Complex
- [x] Const
- [x] ConstPattern
- [x] CSend
- [x] Cvar
- [x] Cvasgn
- [x] Def
- [x] Defined
- [x] Defs
- [x] Dstr
- [x] Dsym
- [x] EFlipFlop
- [x] EmptyElse
- [x] Encoding
- [x] Ensure
- [x] Erange
- [x] False
- [x] File
- [x] FindPattern
- [x] Float
- [x] For
- [x] ForwardArg
- [x] ForwardedArgs
- [x] Gvar
- [x] Gvasgn
- [x] Hash
- [x] HashPattern
- [x] Heredoc
- [x] If
- [x] IfGuard
- [x] IFlipFlop
- [x] IfMod
- [x] IfTernary
- [x] Index
- [x] IndexAsgn
- [x] InPattern
- [x] Int
- [x] Irange
- [x] Ivar
- [x] Ivasgn
- [x] Kwarg
- [x] Kwargs
- [x] KwBegin
- [x] Kwnilarg
- [x] Kwoptarg
- [x] Kwrestarg
- [x] Kwsplat
- [x] Lambda
- [x] Line
- [x] Lvar
- [x] Lvasgn
- [x] Masgn
- [x] MatchAlt
- [x] MatchAs
- [x] MatchCurrentLine
- [x] MatchNilPattern
- [x] MatchPattern
- [x] MatchPatternP
- [x] MatchRest
- [x] MatchVar
- [x] MatchWithLvasgn
- [x] Mlhs
- [x] Module
- [x] Next
- [x] Nil
- [x] NthRef
- [x] Numblock
- [x] OpAsgn
- [x] Optarg
- [x] Or
- [x] OrAsgn
- [x] Pair
- [x] Pin
- [x] Postexe
- [x] Preexe
- [ ] Procarg0
- [ ] Rational
- [ ] Redo
- [ ] Regexp
- [ ] RegOpt
- [x] Rescue
- [ ] RescueBody
- [x] Restarg
- [ ] Retry
- [x] Return
- [x] SClass
- [x] Self\_
- [x] Send
- [ ] Shadowarg
- [ ] Splat
- [x] Str
- [x] Super
- [x] Sym
- [x] True
- [x] Undef
- [ ] UnlessGuard
- [ ] Until
- [ ] UntilPost
- [x] When
- [x] While
- [ ] WhilePost
- [ ] XHeredoc
- [ ] Xstr
- [x] Yield
- [ ] ZSuper

## Development

```bash
yarn
yarn build
```

To release a package:

```bash
npm version [major/minor/patch]
git push --follow-tags
```
