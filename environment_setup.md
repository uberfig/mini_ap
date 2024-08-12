## install openssl

### debian/ubuntu

```shell
sudo apt install openssl
```
```shell
sudo apt install libssl-dev
```

### nixos

you just need to run `nix-shell` before building

## intsall postgres

### debian/ubuntu

```shell
sudo apt install postgresql
```
```shell
sudo apt install libpq-dev
```

### nixos

basically paste this into your config and rebuild. I was hoping to put it in the default.nix but alas that seems like a hastle and wouldn't make a lot of sense for it in release. I'm still very new to nix so please let me know in an issue or reach out to me if you have any pointers

https://nixos.wiki/wiki/PostgreSQL

```nix
  services.postgresql = {
    enable = true;
    ensureDatabases = [ "mydatabase" ];
    enableTCPIP = true;
    authentication = pkgs.lib.mkOverride 10 ''
      #type database  DBuser  auth-method
      local all       all     trust
      #type database DBuser origin-address auth-method
      # ipv4
      host  all      all     127.0.0.1/32   trust
      # ipv6
      host all       all     ::1/128        trust
    '';
  };
```

don't forget to run vs code from the terminal and build after running `nix-shell` so that the default.nix is active. You can omit that for vs code if you get the Nix Environment Selector plugin or something similar

## create your user 
make sure to substitute the username and password for something more secure if you're going to run this in the wild

```shell
sudo -u postgres createuser -s -i -d -r -l -w ivy
```
```shell
sudo -u postgres psql -c "ALTER ROLE ivy WITH PASSWORD 'password';"
```

## droping and creating the database

```shell
dropdb mini_ap
```
```shell
createdb mini_ap
```
