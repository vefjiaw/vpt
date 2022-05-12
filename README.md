# Elements
Elements is a package manager with the focus on speed and ease of use.

## Installing Elements

### Dependencies
In order to use Elements, the following packages are needed `python 3.10`(may work with 3.9 but it is untested), `pip` `colorama` `requests`

### Precompiled binary
This is actually the method that Elements itself uses
```
wget https://raw.githubusercontent.com/NitrogenLinux/elements/stable/lmt
chmod a+x lmt
sudo mv lmt /usr/bin/
git clone https://github.com/tekq/elements-search.git
sudo mkdir /etc/elements
sudo elements-search/* /etc/elements
rm -rf elements-search
sudo chmod a+x /etc/elements/{search,search-repo}
```

## Contributing to Elements
Everyone is welcome to contributing to Elements
