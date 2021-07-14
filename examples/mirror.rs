use proc_caesar::mirror;

mirror![
{
    res
    ;(ys)extend.res
    ;(xs)extend.res

    {
        ({
            ny
            ;(nx)Some = x
            ;()next.ys = y
        } else {
            nx
            ;(ny) Some = y
            ;() next.xs = x
        } ny => nx if)push.res
    } (y, x) = ((ny)Some, (nx)Some) let while

    ;(() next.ys, () next.xs) = (y mut, x mut) let
    ;(() into_iter.ys, () into_iter.xs) = (ys mut, xs mut) let
    ;(()len.ys + ()len.xs)with_capacity::Vec = res mut let
} <T>Vec <- (<T>Vec :ys, <T>Vec :xs)<Ord :T>merge fn
];

fn main() {
    merge(vec![1, 3, 5], vec![2, 3, 4]);
}

