<h1 align="center">
  <a href="https://github.com/ShaharyarAhmed-bot/cell"><img src="icons/cell.png" alt="Cell" width="200" height="200"/></a><br>
  
  <a href="https://github.com/ShaharyarAhmed-bot/cell">Cell</a>
  
</h1>


<h4 align="center">A minimal command line text editor written in Rust</h4>

<center><a href="https://www.rust-lang.org/"><img alt="Rust" src="https://img.shields.io/badge/-Rust-C93A42?style=flat-square&logo=rust&logoColor=black%22"></a> </center>



***Development status:*** *The cell editor project is something I made in my free
time and is not something I activly work on. I would love to work on it more but
I dont have enough time. I will happily accept bug fixes, updates and improvements.
I have many things planned for cell and hope I can get around then. I want to make 
cell into a modern text editor which includes everything a developer would need
to write code faster and more efficently.*   
— Editor

Goals include:

* ***Incredibly high performance***. *The editor should be lightning fast and complete
  all the drawing operations really quick. It shouldnt make you wait for anything.*              

* ***Beauty***. *The editor should look stunning and fit well on all modern desktops,
  It should preform all text operations which the best tecnology avaliable
  and support Unicode fully.*

* ***Reliability***. *Crashing, hanging, or losing work should never happen.*

* ***Plugins***. *Cell should support the installation, removal of plugins.*

* ***Custom Scripting Language***. *There should be a custom scriptintg language for
the editor which will help interact with the editor on a basic level.*

* ***Developer friendliness***. *Along the scripting language, users who do not know 
the scripting language should be able to edit and customise cell according 
to their needs without any hassle.*

* ***Windows compatibility***. *Cell should also be avaliable for windows.*

Screenshot:

![cell screenshot](/images/screenshots/cell-screenshot.png?raw=true)


## Getting started

Cell is avaliable for only Linux currently. You can install it using the <a href="https://wiki.archlinux.org/title/pacman">pacman</a> package manager 
which is avaliable for Arch Linux or if you are on any other Linux distrubution you can use <a href="https://snapcraft.io/">snap</a>.

It is avaliable as a snap package beacuse snap packages allow it to be packaged for a varierty of
Linux distrubutions without any hassle.

MacOS and Windows is coming soon! As soon as I learn how to package it for <a href="https://brew.sh/">brew</a> and <a href ="https://docs.microsoft.com/en-us/windows/package-manager/winget/">winget</a> 😂

## Installation
### Linux

***NOTE***: *I have created a new revision for cell on <a href="https://snapcraft.io">Snapcraft</a> which is currently being reviewed therfore it cannot be installed right now. However you can still install cell on Arch Linux and build the project on your own. When the reviewing process is done I will update this section.*


To install cell on Linux install <a href="https://snapcraft.io/docs/installing-snapd">snap</a>
on your Linux distrubution.

If you are running Arch Linux go and read the Arch Linux section down below....

After installing snap excecute the following:

`sudo snap install cellionor`



### Arch Linux

You can install cell on Arch Linux using the pacman package manager or any <a href="https://wiki.archlinux.org/title/AUR_helpers">AUR</a> helper:

`pacman -S cell-bin`

I have used the <a href="https://github.com/Jguer/yay">yay</a> AUR helper as an example here:

`yay -S cell-bin`


### Windows
The terminal library that I have used to make cell doesnt work on Windows, I will look into that issue sometime in the future
[Windows is coming soon!]


### MacOS
[MacOS is coming soon]


### Building the project

Cell targets 'recent stable Rust'. I recommend installing via [rustup](https://www.rustup.rs).

To build the project:

```
> cargo build
```

Then you can run the project:

```
> cargo run
```

🥳Viola! The editor starts running


## License

This project is licensed under the GNU GENERAL PUBLIC LICENSE V3 [license](LICENSE).


## Contributions

I will gladly accept contributions via GitHub pull requests. I will
soon add a CONTRIBUTING.md for more details.
