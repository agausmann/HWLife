# Images

EEPROM image generators and precomputed images for various automata.

The firmware requires that the EEPROM is flashed with one of these images, and
which image you flash will determine the behavior of the cell.

You can also create your own images based on whatever 2-state automata you want,
including automata with non-symmetric behavior, as long as the cell's next state
is a function of its Moore neighborhood (its own state and the state of the 8
cells surrounding it).

## Prebuilt Images

- `life.bin` - Classic Game of Life (B3/S23)

## Format

The `.bin` files contain a raw image of the EEPROM contents, with the first byte
stored at the EEPROM start address and successive bytes at successive addresses.


The EEPROM contains a table that is 64 bytes (512 bits) long. In each
generation, the cell's next state is determined by selecting one of the bits in
this table.

Each bit in the table is addressed with a 9-bit index. The upper 6 bits are the
address of the byte containing the bit, where 0 (`000000`) is the first byte of
the table, and 63 (`111111`) is the last byte of the table. The lower 3 bits are
the index of the bit within that byte, where 0 (`000`) is the least-significant
bit, and 7 (`111`) is the most-significant bit.

For example, to look up bit 237 (`011101101`), you would read byte 29 (`011101`)
and read bit 5 (`101`) in that byte.

The indexes are computed from each of the 8 neighbor states plus the current
state of the cell itself. Each of the 9 states corresponds to a single bit in
the index: if the cell is alive, the bit is `1`, and if the cell is dead, the
bit is `0`.

Here are the bit-positions of each state (neighbor and self) arranged in their physical positions:

```
+-----+-----+-----+
| NW  |  N  | NE  |
|  7  |  6  |  5  |
+-----+-----+-----+
|  W  |     |  E  |
|  0  |  8  |  4  |
+-----+-----+-----+
| SW  |  S  | SE  |
|  1  |  2  |  3  |
+-----+-----+-----+
```

The most-significant bit of the index is set to the self state, and then bits
7-0 wind clockwise around the neighbors, starting from the top-left.