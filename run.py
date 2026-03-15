from subprocess import run
from shutil import rmtree

tests_dir: str = "tests"
project_name: str = "proj"

run(["cargo", "build", "--release"])

try: rmtree(project_name)
except: pass
run([f"./target/release/setup", "new", f"{project_name}"])

run(["ls", f"{project_name}"])