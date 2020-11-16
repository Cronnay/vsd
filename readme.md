# VSD

Simple CLI to calculate velocity, distance or speed from the arguments

## Get started

`git clone https://github.com/Cronnay/vsd.git`  
`cd vsd/`  
`cargo build`

To have it in your $PATH - run `cargo install --path .`

## Examples

`$ vsd --help`  
```
USAGE:
    vsd [OPTIONS]

FLAGS: 
    -h, --help      Prints help information
    -V, --version   Prints version information

OPTIONS:
    -d, --distance <Distance in meters>
    -t, --time <Time in seconds>
    -v, --velocity <Velocity in m/s>
```


`$ vsd -d 300 -t 25`   
`> Calulating velocity: 12m/s`


`$ vsd -v 10 -t 25`   
`> Calulating velocity: 12m/s`
