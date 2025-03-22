# #!/usr/bin/env python3

import argparse
from importlib.metadata import entry_points

import openspeleo_core


def main():
    registered_commands = entry_points(group="openspeleo_core.actions")

    parser = argparse.ArgumentParser(prog="openspeleo_core")
    parser.add_argument(
        "-v",
        "--version",
        action="version",
        version=f"%(prog)s version: {openspeleo_core.__version__}",
    )
    parser.add_argument(
        "command",
        choices=registered_commands.names,
    )
    parser.add_argument(
        "args",
        help=argparse.SUPPRESS,
        nargs=argparse.REMAINDER,
    )

    args = argparse.Namespace()
    parser.parse_args(namespace=args)

    main_fn = registered_commands[args.command].load()
    return main_fn(args.args)
