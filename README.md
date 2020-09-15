# take_ref

`take_ref` provides `TakeRef`, `TakeSlice`, and `TakeString` traits which enable treating references/slices and values interchangeably.
They can be treated as references/slices.
And you can take ownership of the value within with a clone performed only as needed.

## An Example

The example `take_slice` can take in both `&[i64]` and `Vec<i64>` instances and refer to them multiple times
before finally deciding that it needs to take ownership of a `Vec<i64>`.
At that point if it is a `&[i64]` a `Vec<i64>` is constructed from the `&[i64]`.
On the other hand, if it is a `Vec<i64>` ownership is transferred directly without overhead.

    use take_ref::TakeSlice;
    
    fn take_slice(value: impl TakeSlice<i64>) {
        value.as_slice();
        value.as_slice();
        value.take();
    }
