Amateur Radio Numeric Callsign Encoding (README)
================================================

The Amateur Radio Numeric Callsign Encoding (ARNCE) is a mechanism
for any radio callsign of up to 12 characters to be encoded into 8 bytes(or
less). These numeric "ham addresses" can also be translated to EUI-48 and EUI-64
addresses, which can be useful when operating consumer Wi-Fi equipment
under Amateur Radio rules. It can also be useful for encoding short
text identifiers into MAC addresses in non-radio contexts.

Examples:

| Callsign      | HamAddr               | EUI-48              | EUI-64                    |
|---------------|-----------------------|---------------------|---------------------------|
| N6DRC         | `5CAC-70F8`           | `02:5C:AC:70:F8:00` | `02:5C:AC:FF:FE:70:F8:00` |
| KJ6QOH/P      | `4671-6CA0-E9C0`      | `C2:46:71:6C:A0:E9` | `C2:46:71:FF:FE:6C:A0:E9` |
| AA0XXX/MOBI-4 | `0683-99D8-F5E7`      | `EA:06:83:99:D8:F5` | `EA:06:83:FF:FE:99:D8:F5` |
| VI2BMARC50    | `8B05-0E89-7118-A8C0` | N/A                 | `C2:8B:05:0E:89:71:18:A8` |

The mechanism for reversibly translating callsigns into numeric identifiers
is outlined in the [ARNCE technical specification (HAM-64)](n6drc-arnce.md#introduction),
included in this repository. Also included in this repository are shell scripts
which can be used to convert between all of these address formats.

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

A implementation in Rust is available [here](https://github.com/arngll/arngll-rust/tree/main/hamaddr).

## See also ##

 * [Amateur Radio Next Generation Link Layer (ARNGLL)](https://github.com/arngll/arngll-spec)
 * [RFC Tools](https://github.com/paulej/rfctools)
 * [Mmark](https://github.com/miekg/mmark)
