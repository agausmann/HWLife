from argparse import ArgumentParser
from typing import NamedTuple


class Rule(NamedTuple):
    birth: set[int]
    survival: set[int]


def parse_mirek(s: str) -> Rule:
    x, y = s.split("/")
    return Rule(
        birth=set(int(c) for c in y),
        survival=set(int(c) for c in x),
    )


def parse_golly(s: str) -> Rule:
    y, x = s.upper().split("/")
    y = y.removeprefix("B")
    x = x.removeprefix("S")
    return Rule(
        birth=set(int(c) for c in y),
        survival=set(int(c) for c in x),
    )


def parse_unknown(s: str) -> Rule:
    x, y = s.upper().split("/")
    # If Golly prefixes are detected, swap:
    if x.startswith("B") and y.startswith("S"):
        y, x = x.removeprefix("B"), y.removeprefix("S")

    return Rule(
        birth=set(int(c) for c in y),
        survival=set(int(c) for c in x),
    )


def main():
    ap = ArgumentParser(
        description="Generate an EEPROM image for any Life-like automaton."
    )

    notation = ap.add_mutually_exclusive_group()
    # TODO
    # notation.add_argument(
    #     "-w",
    #     "--wolfram",
    #     action="store_true",
    #     help="Hint that the rule is provided as a Wolfram code.",
    # )
    notation.add_argument(
        "-m",
        "--mirek",
        action="store_true",
        help="Hint that the rule is provided in Mirek/MCell notation (e.g. 23/3).",
    )
    notation.add_argument(
        "-g",
        "--golly",
        action="store_true",
        help="Hint that the rule is provided in Golly/RLE notation (B3/S23)",
    )

    ap.add_argument(
        "rule",
        help="The rule to generate. The format will be auto-detected by default, but can be specified using -w, -m, or -g.",
    )
    ap.add_argument("outfile", help="The file to write the image to.")

    args = ap.parse_args()

    try:
        if args.mirek:
            rule = parse_mirek(args.rule)
        elif args.golly:
            rule = parse_golly(args.rule)
        else:
            rule = parse_unknown(args.rule)
    except Exception:
        print(f"error: cannot parse rule {args.rule!r}")
        raise

    with open(args.outfile, "wb+") as fout:
        print(f"Generating rule {rule!r} ...")
        for byte_index in range(64):
            byte = 0
            for bit_index in range(8):
                full_index = (byte_index << 3) | bit_index
                neighbors = full_index & 0xFF
                current_self = full_index >> 8

                # Each 1-bit in neighbors is an alive neighbor
                num_alive = sum(((neighbors >> i) & 1) for i in range(8))

                if current_self:
                    alive_set = rule.survival
                else:
                    alive_set = rule.birth

                if num_alive in alive_set:
                    byte |= 1 << bit_index

            fout.write(bytes([byte]))


main()
