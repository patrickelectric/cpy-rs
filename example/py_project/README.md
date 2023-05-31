# How to use

From the ./py_project/ 
```shell
python -m venv venv
source venv/bin/activate
pip install --upgrade pip
pip install maturin
pip install ../.
python main.py
```

Output:
```shell
Tire pressure: 47.49
Tire material: Material.Rubber
Tire size: 7.04 width, 15.23 height
The tire have an aspect ratio of 46.23
Yep, no returns
```

Alternativelly, to install, you can use:
```shell
maturin develop
```
You can deploy and install manually with:
```shell
maturin build --release
pip install target/wheels/example-0.1.0-cp311-cp311-linux_x86_64.whl
```