import os
import re
import shutil
import subprocess
import sys
from contextlib import contextmanager
from pathlib import Path
from tempfile import TemporaryDirectory

sys.path.append(".")

from codegen import versions
from codegen.gen import gen_svd


ROOT = Path(__file__).resolve().parents[1]
SVD_DIR = ROOT / "svd"
SRC_DIR = ROOT / "src"


def generate(name, peripherals):
    svd_file = generate_svd(name, peripherals)
    generate_rust(name, svd_file)


def generate_svd(name, peripherals):
    svd = gen_svd(name, peripherals)

    SVD_DIR.mkdir(exist_ok=True)
    svd_file = SVD_DIR / (name + ".svd")
    svd_file.write_text(svd)

    return svd_file


def generate_rust(name, svd_file):
    with inside_tempdir():
        subprocess.run(
            ["svd2rust", "-g", "-i", str(svd_file), "--target", "none"],
            check=True,
        )

        shutil.copyfile("generic.rs", SRC_DIR / "generic.rs")

        src = Path("lib.rs")
        dst = SRC_DIR / (name + ".rs")
        clean_and_copy_module(src, dst)


def clean_and_copy_module(src, dst):
    """
    Copy the Rust file src to dst. Remove all module-level attributes
    that are not `allow`s in the process.
    """

    attr = re.compile(r"^# ! \[ (?P<attr>\w+) .*\]\n$")

    with src.open("rt") as fsrc, dst.open("wt") as fdst:
        for line in fsrc:
            m = attr.match(line)
            if m and m.group("attr") != "allow":
                continue

            fdst.write(line)

            if not m:
                break

        fdst.write(fsrc.read())


@contextmanager
def inside_tempdir():
    with TemporaryDirectory() as tmpdir:
        previous = Path.cwd()
        os.chdir(tmpdir)
        try:
            yield
        finally:
            os.chdir(previous)


def rustfmt():
    subprocess.run(["cargo", "fmt"], check=True)


def main():
    generate("f302", versions.f302.make())
    generate("f303", versions.f303.make())
    generate("f303e", versions.f303e.make())
    generate("f333", versions.f333.make())
    generate("f373", versions.f373.make())

    rustfmt()


if __name__ == "__main__":
    main()