Amateur Radio Numeric Callsign Encoding (README)
================================================

This is the repository for the [ham-address specification
(HAM-64)](n6drc-arnce.md) ([HTML](n6drc-arnce.html) |
[TXT](n6drc-arnce.txt) | [XML](n6drc-arnce.xml)), which specifies a
way for efficiently and reversably encoding radio callsigns into
numberic identifiers, which may then be used as addresses for packet
radio or other purposes.

Click on [this link](https://rawgit.com/arngll/arnce-spec/main/n6drc-arnce.html)
to view the most recent HTML version of the specification.

The original source of the specification is in the file [`n6drc-arnce.md`](n6drc-arnce.md),
which is formatted in [mmark syntax](https://github.com/miekg/mmark/wiki/Syntax) (vaguely
similar to [GitHub-flavored Markdown](https://help.github.com/articles/basic-writing-and-formatting-syntax/)).
The included makefile uses [paulej's rfctools](https://github.com/paulej/rfctools) to
convert this base format into text, HTML, and XML formats.

This project is hosted on GitHub as [arngll/arnce-spec](https://github.com/arngll/arnce-spec).

## Reference Implementations ##

This repository also contains reference implementations in various
languages. Currently, reference implementations in the following
languages are included:

 * Standard unix shell sript (`/bin/sh`)

## See also ##

 * [Amateur Radio Next Generation Link Layer (ARNGLL)](https://github.com/arngll/arngll-spec)
 * [RFC Tools](https://github.com/paulej/rfctools)
 * [Mmark](https://github.com/miekg/mmark)
