# Context

In the mCRL2 202507.0 release of the mCRL2 toolset, see the
[repository](https://github.com/mCRL2org/mCRL2) or [website](https://mcrl2.org),
we have adapted the parser to conform with the priorities written in the
corresponding book.

# Tool

This is a tool that can be used to find parsing differences

# Building

This tool contains two different versions of the mCRL2 toolset before and after
the changes to the parser. Linking these versions together has some nasty
consequences due to how linking works for C and C++ projects. In particular, the
`dparser` C library will have a lot of name clashes of the two different parsers
that will silently be resolved.