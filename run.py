from subprocess import run
from shutil import rmtree


def soft_rm(name: str):
    try: rmtree(name)
    except: pass


project_name: str = "Project"
mod_name: str = "Mod"

run(["cargo", "build", "--release"])

soft_rm(project_name)
run([f"./target/release/setup", "new", f"{project_name}"])

soft_rm("Source")
run(["ls", f"{project_name}"])


