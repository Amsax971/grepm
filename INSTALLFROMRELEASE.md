# install

# from release

installing from release is the **most recommended** installation method because it provide you a stable version of the code

here, you just have to go to the `[binary.zip]` file in the release and and execute (only on linux) :
```sh
make install
```
and to uninstall, execute:
```sh
make uninstall
```

# from github code rn

|Os|command|
|-|-|
|ubuntu/debian based os|`sudo apt install cargo`|
|Arch linux based os|`pacman -S rustup`|

on others os (stil on linux), use this command : `curl https://sh.rustup.rs -sSf | sh`

and after you installed the compiler execute this (still on linux)

```sh
git clone https://github.com/Amsax971/lsm.git ./lsm
cd ./lsm
make install
```
