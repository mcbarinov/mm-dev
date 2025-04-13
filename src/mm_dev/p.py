import os
import tomllib
from dataclasses import dataclass
from pathlib import Path
from typing import Optional

import pydash
import typer
from mm_std import fatal, print_table, run_command
from packaging.requirements import Requirement
from typer import Argument

from mm_dev._common import create_app

app = create_app(multi_command=True)


@dataclass
class OutdatedPackage:
    package: str
    installed_version: str
    new_version: str
    pyproject_version: Requirement | None = None


@dataclass
class ProjectDependencies:
    dependencies: list[Requirement]
    dev_dependencies: list[Requirement]

    def get_outdated_pyproject_packages(self, all_outdated_packages: list[OutdatedPackage], dev: bool) -> list[OutdatedPackage]:
        result = []
        deps = self.dev_dependencies if dev else self.dependencies
        for package in all_outdated_packages:
            dep = pydash.find(deps, lambda p: p.name == package.package)  # noqa: B023
            if dep:
                package.pyproject_version = dep
                result.append(package)

        return result


def parse_outdated_packages(value: str) -> list[OutdatedPackage]:
    result = []
    for line in value.splitlines():
        if not line.strip():
            continue
        parts = line.split()
        if len(parts) < 3:
            continue
        package = parts[0]
        old_version = parts[1]
        new_version = parts[2]
        if package.startswith("--") or package == "Package":
            continue
        result.append(OutdatedPackage(package=package, installed_version=old_version, new_version=new_version))
    return result


def parse_pyproject_packages(pyproject_file: Path) -> ProjectDependencies:
    with pyproject_file.open("rb") as f:
        data = tomllib.load(f)

    dependencies = [Requirement(d) for d in pydash.get(data, "project.dependencies", [])]
    dev_dependencies = [Requirement(d) for d in pydash.get(data, "tool.uv.dev-dependencies", [])]
    return ProjectDependencies(dependencies=dependencies, dev_dependencies=dev_dependencies)


@app.command(name="o", help="uv pip list --outdated")
def pip_list_outdated() -> None:
    res = run_command("uv pip list --outdated")
    typer.echo(res.stdout)

    pyproject_file = Path("pyproject.toml")
    if not pyproject_file.exists():
        return

    all_outdated_packages = parse_outdated_packages(res.stdout)
    pyproject_deps = parse_pyproject_packages(pyproject_file)

    outdated_pyproject_deps = pyproject_deps.get_outdated_pyproject_packages(all_outdated_packages, dev=False)
    outdated_pyproject_dev_deps = pyproject_deps.get_outdated_pyproject_packages(all_outdated_packages, dev=True)

    if outdated_pyproject_deps:
        rows = [[d.pyproject_version, d.installed_version, d.new_version] for d in outdated_pyproject_deps]
        print_table("pyproject.toml, deps", ["pyproject", "installed", "new"], rows)

    if outdated_pyproject_dev_deps:
        rows = [[d.pyproject_version, d.installed_version, d.new_version] for d in outdated_pyproject_dev_deps]
        print_table("pyproject.toml, dev deps", ["pyproject", "installed", "new"], rows)


@app.command(name="l", help="uv pip list")
def pip_list() -> None:
    run_command("uv pip list", capture_output=False)


@app.command(name="i", help="install packages or requirements.txt")
def install(packages: Optional[str] = Argument(None)) -> None:  # noqa: UP007
    if not os.getenv("VIRTUAL_ENV"):
        fatal("venv is not activated")
    if packages:
        run_command(f"uv pip install {packages}", capture_output=False)
        return
    run_command("uv pip install -Ur requirements.txt", capture_output=False)


@app.command(name="v", help="create .venv")
def venv() -> None:
    if os.getenv("VIRTUAL_ENV"):
        fatal("venv is activated already")

    if Path(".venv").exists():
        fatal(".venv exists")
    run_command("uv venv", capture_output=False)


@app.command(name="d", help="uninstall all packages(+editable) from venv")
def uninstall() -> None:
    if not os.getenv("VIRTUAL_ENV"):
        fatal("venv is not activated")

    run_command("uv pip list --format freeze -e | xargs uv pip uninstall", capture_output=False)
    run_command("uv pip freeze | xargs uv pip uninstall", capture_output=False)


@app.command(name="c", help="uv cache clean {package}")
def clean_cache(package: str) -> None:
    run_command(f"uv cache clean {package}", capture_output=False)


if __name__ == "__main__":
    app()
