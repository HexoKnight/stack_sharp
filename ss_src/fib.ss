//dep: stdlib

//fibanacci :)
[fib|fib_recursive /* index -- fibonacci number */ // index >= 0
    dup 1 gt (
        dup 1 - fib
        swap 2 - fib
        +
    )
]
[fastfib|fib_fast /* index -- fibonacci number */ // index >= 0
    dup 1 gt (
        0 1
        { brot dup 1 ne (
            1 - rot
            dup lswap +
        })
        drop swap drop
    )
]