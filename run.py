from subprocess import run
from os import mkdir
from shutil import rmtree

tests_dir: str = "tests"

run(["cargo", "build", "--release"])

try: 
    rmtree(tests_dir)
except:
    pass
mkdir(tests_dir)
run(["cp", "target/release/setup", tests_dir])
run([f"./{tests_dir}/setup", "new", "proj"])
run(["ls", f"{tests_dir}"])