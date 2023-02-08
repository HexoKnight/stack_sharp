[;set|.> /* value pointer -- */ . > ]
[;get|.< /* pointer -- value */ . < ]
[;)!( )! -1 ( ] [;}) } ) ]

//: comparison
[;gtz +@ ] [;ltz|-@ ~ +@ ]
[;gez ltz ! ] [;lez gtz ! ]
[;ez @ ] [;nez @ ! ]
//[@ dup +@ swap -@ | ! ]
[;eq - @ ] [;ne|neq eq ! ]
[;gt - gtz ] [;lt - ltz ]
[;ge lt ! ] [;le gt ! ]
//;

//: stack manipulation
[;nswap /* a(n) .. a1 a0 n -- a0 .. a1 a(n) */ <> ]
[;swap /* a b -- b a */ 0 <> ]
[;lswap|longswap /* a b c -- c b a */ 1 <> ]
[;llswap|longlongswap /* a b c -- c b a */ 2 <> ]
[;rot|frot /* a b c -- c a b */ lswap swap ]
[;brot /* a b c -- b c a */ swap lswap ]

[;drop /* a -- */ -1 >> ]
[;ndrop /* a(n+1) a(n) .. a2 a1 n -- a(n+1) */ ~ >> ]
[;clr /* ... a2 a1 -- (nada)*/ 0 >> ]
[;dup /* a -- a a */ 1 >> ] // 0 dupfrom
[;ldup /* a b -- a b a */ swap dup rot ] // 1 dupfrom
[;ddup|2dup|doubledup /* a b -- a b a b */ dup lswap dup llswap ]
[dupfrom /* a(n) .. a1 a0 n -- a(n) .. a1 a0 a(n) */ // n >= 0
    dup ez (
        drop dup
    )!(
        dup rot nswap // a0 .. a1 n a(n)
        dup brot nswap  // a(n) .. a1 a(n) a0
        swap // a(n) .. a1 a0 a(n)
    )
]
//;

[;while { ] [;do ) ] [;end }) ]
[;repeat|rep|repeat{(|rep{( { 1 - dup ltz ( drop )!( ] // repeats a set of instructions but requires them to leave the top of the stack alone
[;list_for|list_for{( /* list_var {-} list_element */ // loops over the elements of a list, placing them on top of the stack but requires the top of the stack to be left alone
    1 +
    { 1 - .< dup 1 + swap ez ( drop )!(
]

//: input/output
:stdio;0
[;in stdio .< ] [;out stdio .> ]
[inln -1 stdio . { < dup '\n' ne ( }) drop ]
[inln_echo -1 stdio . { < 2 >> '\n' ne ( > }) drop ]
[print stdio . { dup gez ( > }) drop ]
[println ;print '\n' out ]
[printerr|print_err|err '!? ' stdio . > > > ;println !? ]
//;

//: maths
[divrem /* dividend divisor -- quotient remainder */
    2dup / rot %
]
//;

[;1+ 1 + ] [;1- 1 - ]
[++ /* pointer -- (pointer value incremented)*/ dup .< 1 + swap .> ]

//: array
:type_array;2
[array_create_raw /* length -- pointer */ dup 1 + #+ dup rot .> ]
[array_destroy /* array_pointer -- pointer */ dup .< 1 + swap #- ]
[array_index /* index array_pointer -- index_pointer */ 1 + + ]
[array_len /* array_pointer -- length */ .< ]

[array_print /* array_pointer -- (characters in array outputted)*/
    2 >> array_len + swap // array_end array_pointer
    { 1 + 2dup ge (
        dup .< out
    })
    2 ndrop
]
[array_println /* array_pointer -- (output array followed by newline)*/
    ;array_print '\n' out
]
//;

//: linked list
:type_list;3
[list_destroy /* list_var -- */
    dup .<
    { dup nez ( dup .< 2 brot #- })
    drop
    . 0 > // remove reference to list stored in variable
]
[list_len /* list_var -- length */
    0
    { swap .< swap ldup nez (
        1 +
    })
    swap drop
]
[list_index /* index list_var -- index_pointer */
    { .< swap 1 - swap ldup gez ( dup .< ez ( !? ) }) // traverse through list until either index becomes < 0 or list ends
    swap drop // remove index
    1 + // get the pointer to the data in the item
]
[list_append /* value list_var -- */
    2 #+ 2 >> // allocate memory
    . 0 > // store the terminating pointer
    brot
    { dup .< nez ( .< }) // find the end of the list
    .> // store the pointer to new item in the previous end
    1 + .> // store the data in the item
]
[list_prepend /* value list_var -- */
    2 #+ // allocate memory
    ldup .< // get pointer to first item
    ldup .> // move pointer to first item to current item's pointer
    dup brot .> // move pointer to current item to list pointer
    1 + .> // store data in the item
]
[list_insert /* value index list_var -- */ // inserts just before index (maybe don't need prepend and append)
    { swap 1 - swap ldup gez ( dup .< nez ( .< }) ) // traverse through list until either index becomes < 0 or list ends
    swap drop // remove index
    list_prepend // prepend before current item in list
]

[list_inln /* list_var -- (input in list)*/
    { in dup '\n' ne ( ldup list_append })
    2 ndrop
]
[list_inln_echo /* list_var -- (input in list)*/
    { in dup '\n' ne ( 2dup rot list_append out })
    2 ndrop
]
[list_print /* list_var -- (output list contents)*/
    { .< dup nez (
        dup 1 + .< out
    })
    drop
]
[list_println /* list_var -- (output list contents followed by newline)*/
    ;list_print '\n' out
]
//;

//: list-array conversions
[array_copy_to_list /* array_pointer -- list_pointer */
    :temp_list
    0 temp_list .>
    2 >> array_len +
    { ldup ldup ne (
        dup .< // get value
        temp_list list_prepend
        1 -
    })
    -2 >>
    temp_list .< // return list_pointer
]
[list_copy_to_array /* list_var -- array_pointer */
    dup list_len dup array_create_raw // list_var list_len array_pointer
    2 >> 2 <> + swap 2 <> lswap // array_pointer list_var array_end array_pointer(to be index_pointer)
    { 1 + 2dup ge (
        lswap .< dup 1 + .< swap // array_pointer index_pointer array_end list_element_value next_list_pointer
        llswap // array_pointer next_list_pointer array_end list_element_value index_pointer
        dup . swap > // store list value at array index // array_pointer next_list_pointer array_end index_pointer
    })
    3 ndrop
]
//;

//: string manipulation
[num_to_char /* num -- char */ // num > 0
    dup -1 eq (
        drop 45 // negative sign
    )!(
        dup 9 gt (
            87 // to lowercase letter (55 for uppercase)
        )!(
            48 // to normal digit
        )
        +
    )
]
[baseN_to_str /* num base list_var -- (num_str stored in list)*/
    swap 1 2 nswap // 1(-> neg) list_var base num
    { dup nez (
        dup ltz (
            ~ 2 nswap ~ 2 nswap // -neg(-> neg) list_var base -num(-> num)
        )!(
            ldup divrem num_to_char 3 dupfrom list_prepend // neg list_var base num/10(-> num)
        )
    })
    2 ndrop swap
    ltz ( 45/* neagtive sign */ swap list_prepend )!( drop )
]
[num_to_str /* num list_var -- (num_str stored in list)*/ 10 swap baseN_to_str ]

[char_to_num /* char -- num */ // char is alphanumeric
    dup 45 eq ( // negative sign
        drop -1
    )!(
        dup 96 gt ( // lowercase letter
            87
        )! dup 64 gt ( // uppercase letter
            55
        )!( // normal digit
            48
        )
        -
    )
]
[str_to_baseN /* base list_var -- num */ // base > 0
    1 rot 0 swap // neg base 0(-> sum) list_var
    list_for{(
        dup .< char_to_num // neg base sum list_element num
        dup ltz (
            drop llswap ~ llswap // -neg(-> neg) base sum list_element
        )!(
            brot 3 dupfrom * + // neg base list_element num+sum*base(-> sum)
            swap // neg base sum list_element
        )
    })
    swap drop *
]
[str_to_num /* list_var -- num */ 10 swap str_to_baseN ]
[print_num_baseN /* num base -- (outputted)*/
    :temp_list
    temp_list baseN_to_str
    temp_list list_print
    temp_list list_destroy
]
[print_num /* num -- (outputted)*/ 10 print_num_baseN ]
//;

//: typed object
:type_obj;0
:type_num;1
[new_obj|new_obj|#obj+ /* -- *obj */
    2 #+ 2 >> // dup twice
    . 0 > // store null terminator in pointer
    1 + . -1 > // store -1 (undefined) in data type
] //TODO: maybe implement some sort of flag for type (eg. 0001 for indexable, 0010 for insertable, etc.)
[assign_obj|obj.> /* *obj(to be stored) *obj -- */
    dup rot .> // store obj pointer in pointer
    1 + . type_obj > // store list type in data type
]

//: lists
[new_list|#list+ /* -- *obj */
    2 #+ 2 >> // dup twice
    . 0 > // store null terminator in pointer
    1 + . type_list > // store list type in data type
]
[assign_list|list.> /* *list *obj -- */
    dup rot .> // store list pointer in pointer
    1 + . type_list > // store list type in data type
]
//;
//: arrays
[new_array|#array+ /* length -- *obj */
    2 #+ 2 >> // dup twice
    2 nswap array_create_raw
    swap .> // store array pointer in pointer
    1 + . type_array > // store array type in data type
]
[assign_array|array.> /* *array *obj -- */
    dup rot .> // store array pointer in pointer
    1 + . type_array > // store array type in data type
]
//;

//: general methods
[len /* *obj -- length */
    dup 1 + .<
    dup type_obj eq ( // obj
        drop .< len
    )! dup type_num eq ( // number
        "cannot get length of integer" err // !?
    )! dup type_array eq ( // array
        drop .< array_len
    )! dup type_list eq ( // list
        drop list_len
    )!( // undefined
        "cannot get length of undefined object" err // !?
    )
]
[index /* index *obj -- element_at_index_pointer */
    dup 1 + .<
    dup type_obj eq ( // obj
        drop .< index
    )! dup type_num eq ( // number
        "cannot index an integer" err // !?
    )! dup type_array eq ( // array
        drop .< array_index
    )! dup type_list eq ( // list
        drop list_index
    )!( // undefined
        "cannot index an undefined object" err // !?
    )
]
[obj_print /* *obj -- (obj printed)*/
    dup 1 + .<
    dup type_obj eq ( // obj
        drop .< obj_print
    )! dup type_num eq ( // number
        drop .< print_num
    )! dup type_array eq ( // array
        drop .< array_print
    )! dup type_list eq ( // list
        drop list_print
    )!( // undefined
        "cannot print an undefined object" err // !?
    )
]
[obj_println /* *obj -- (obj printed with trailing newline)*/
    ;obj_print '\n' out
]

[;;destroy_value_only /* *obj -- */
    dup 1 + .<
    dup type_obj eq ( // obj
        drop .< destroy
    )! dup type_num eq ( // number
        2 ndrop
    )! dup type_array eq ( // array
        drop .< array_destroy
    )! dup type_list eq ( // list
        drop list_destroy
    )!(
        2 ndrop "empty object stored" err // !?
    )
]
[;destroy_reference /* *obj -- */ // UNSAFE - might lead to unfreed memory, use only if another reference exists to that data
    2 swap #-
]
[destroy /* *obj -- */
    dup ;destroy_value_only
    destroy_reference
]
[destroy_value_only /* *obj -- */ ;destroy_value_only ] // UNSAFE - might lead to incorrectly typed objects, generally just don't use
[destroy_value /* *obj -- */
    2 >> ;destroy_value_only
    1 + . -1 > // store -1 (undefined) in type
    . 0 > // store null-terminator in pointer
]
//;
//;

//: USES:
[add
    "Enter two numbers:" println
    :list
    list list_inln_echo '\n' out list str_to_num list list_destroy
    list list_inln_echo '\n' out list str_to_num list list_destroy
    + print_num
]
//;

//: testing:
// [test
//     { (
//         !? "hello bro"
//     })
    
//     hi guys
// ]

// [testheap
//     1 +
//     { 1 - dup gez (
//         dup 500 + dup .>
//     })
//     drop
// ]
// 20 testheap
//;