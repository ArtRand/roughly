import os
from setuptools import setup
from setuptools_rust import Binding, RustExtension

if os.environ.get("RELEASE"):
    debug = False
else:
    debug = True

setup(
    name="roughly",
    version="0.1.0",
    rust_extensions=[
        RustExtension("roughly_rs", binding=Binding.PyO3, debug=debug)
    ],
    # packages=find_namespace_packages(include=["pyext.*"]),
    packages=[
        "roughly",
    ],
    zip_safe=False,
)
