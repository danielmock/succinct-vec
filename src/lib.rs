use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct BcdmsArray<T> {
    index: Vec<Vec<T>>, // the index pointing to the data blocks,

    // TODO: maybe compute ad hoc, could be quite expensive (high constant)
    n: usize, // number of elements
    d: usize, // the number of non-empty data blocks

    // true iff number of superblocks is odd
    s_odd: bool,

    // TODO: replace it with vector calls do index[d-1]
    len_last_data: usize, // occupancy of last data block
    cap_last_data: usize, // size of last data block

    // Definitely needed IMO
    len_last_super: usize, // length of super block (amount of data blocks)
    cap_last_super: usize, // capacity of super block (amount of data blocks)
}

impl<T> BcdmsArray<T> {
    pub fn push(&mut self, value: T) {
        self.grow();
        self.index[self.d - 1].push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.shrink()
    }

    pub fn read(&self, i: usize) -> Option<&T> {
        if i >= self.n {
            return None;
        }
        let (a, b) = BcdmsArray::<T>::locate(i);
        Some(&self.index[a][b])
    }

    fn grow(&mut self) {
        // 1. If the last nonempty data block DB[d-1] is full
        if self.cap_last_data == self.len_last_data {
            // a If the last superblock SB[s-1] is full
            if self.len_last_super == self.cap_last_super {
                self.s_odd ^= true;
                if self.s_odd {
                    self.cap_last_super *= 2;
                } else {
                    self.cap_last_data *= 2;
                }
                self.len_last_super = 0;
            }

            self.index.push(Vec::with_capacity(self.cap_last_data));

            self.d += 1;
            self.len_last_super += 1;
            self.len_last_data = 0;
        }

        self.n += 1;
        self.len_last_data += 1;
    }

    fn shrink(&mut self) -> Option<T> {
        if self.n == 0 {
            return None;
        }

        self.n -= 1;
        self.len_last_data -= 1;
        let ret = self.index[self.d - 1].pop();

        // If DB[d-1] is empty
        if self.len_last_data == 0 {
            // 2 b TODO reallocate index when quarter full???

            self.d -= 1;
            self.len_last_super -= 1;

            if self.len_last_super == 0 {
                self.s_odd ^= true;

                if self.s_odd {
                    self.cap_last_data /= 2;
                } else {
                    self.cap_last_super /= 2;
                }

                self.len_last_super = self.cap_last_super;
            }

            self.len_last_data = self.cap_last_data;
        }

        return ret;
    }

    fn locate(index: usize) -> (usize, usize) {
        let index = index + 1;

        let k = std::mem::size_of::<usize>() * 8 - index.leading_zeros() as usize - 1; // size of index - 1

        let l = (k + 1) / 2; // ceil(k/2)

        let b = (index & !(1 << k)) >> l; // get the first floor(k/2) bits of index after the leading One
                                          // remember that there are k bits after the leading 1 and that we have to cut off the last ceil(k/2) bits

        let e = index & ((1 << l) - 1); // get the last ceil(k/2) bits of index

        // There is an error in the paper. The number of data blocks in super blocks prior to SB[k] is not 2^k - 1, since an SB[i] has 2^floor(i/2) data blocks, not 2^i
        let p = 2 * ((1 << l) - 1) - (k % 2) * ((1 << l) / 2);

        // return e-th element of DB[p+b]
        (p + b, e)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.index.iter().flat_map(|x| x.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.index.iter_mut().flat_map(|x| x.iter_mut())
    }
}

impl<T> IntoIterator for BcdmsArray<T> {
    type Item = T;
    type IntoIter = std::iter::FlatMap<
        std::vec::IntoIter<Vec<T>>,
        std::vec::IntoIter<T>,
        fn(Vec<T>) -> <Vec<T> as IntoIterator>::IntoIter,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.index
            .into_iter()
            .flat_map(std::iter::IntoIterator::into_iter)
    }
}

impl<T> Index<usize> for BcdmsArray<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        let (a, b) = BcdmsArray::<T>::locate(i);
        &self.index[a][b]
    }
}

impl<T> IndexMut<usize> for BcdmsArray<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        let (a, b) = BcdmsArray::<T>::locate(i);
        &mut self.index[a][b]
    }
}

impl<T> Default for BcdmsArray<T> {
    fn default() -> BcdmsArray<T> {
        BcdmsArray {
            index: vec![Vec::with_capacity(1)],
            n: 0,
            s_odd: true,
            d: 1,
            len_last_data: 0,
            cap_last_data: 1,
            len_last_super: 1,
            cap_last_super: 1,
        }
    }
}
