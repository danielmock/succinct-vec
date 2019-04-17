#[derive(Debug, Clone)]
pub struct BcdmsArray<T> {
    index: Vec<Vec<T>>, // the index pointing to the data blocks,

    // TODO: maybe compute ad hoc, could be quite expensive (high constant)
    n: usize, // number of elements
    d: usize, // the number of non-empty data blocks

    // TODO Maybe replace with bool
    s: usize, // number of suberblocks

    // TODO: maybe replace with call to index[d] or index.len
    has_empty_data: bool, // the number of empty data blocks (there are atmost two)

    // TODO: replace it with vector calls do index[d-1]
    len_last_data: usize, // occupancy of last data block
    cap_last_data: usize, // size of last data block
    
    // Definitely needed IMO
    len_last_super: usize, // length of super block (amount of data blocks)
    cap_last_super: usize, // capacity of super block (amount of data blocks)
}

impl<T> BcdmsArray<T> {
    pub fn new() -> BcdmsArray<T> {
        BcdmsArray {
            index: vec![Vec::with_capacity(1)],
            n: 0,
            s: 1,
            d: 1,
            has_empty_data: false,
            len_last_data: 0,
            cap_last_data: 1,
            len_last_super: 1,
            cap_last_super: 1,
        }
    }

    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.index.into_iter()
            .flat_map(|inner| inner.into_iter())
    }

    pub fn push(&mut self, value: T) {
        self.grow();
        self.index[self.d-1].push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.shrink()
    }

    pub fn read(&self, i: usize) -> Option<&T> {
        if i >= self.n {
            return None
        } 
        let (a,b) = BcdmsArray::<T>::locate(i);
        Some(&self.index[a][b])
    }

    fn grow(&mut self) {
        // 1. If the last nonempty data block DB[d-1] is full
        if self.cap_last_data == self.len_last_data {
            // a If the last superblock SB[s-1] is full
            if self.len_last_super == self.cap_last_super {
                self.s += 1;
                if self.s % 2 == 1 {
                    self.cap_last_super *= 2;
                } else {
                    self.cap_last_data *= 2;
                }
                self.len_last_super = 0;
            }

            if !self.has_empty_data {
                // i manual resizing is unnecessary
                self.index.push(Vec::with_capacity(self.cap_last_data));
            } else {
                self.has_empty_data = false;
            }

            self.d += 1;
            self.len_last_super += 1;
            self.len_last_data = 0;
        }

        self.n += 1;
        self.len_last_data += 1;
    }
    

    fn shrink(&mut self) -> Option<T> {
        if self.n == 0 {
            return None
        }

        self.n -= 1;
        self.len_last_data -= 1;
        let ret = self.index[self.d-1].pop();

        // If DB[d-1] is empty
        if  self.len_last_data == 0 {
            // If there is another empty data block, it has to be at last position (DB[d]), remove it with pop
            if self.has_empty_data {
                let temp = self.index.pop();
                assert!(
                    match temp {
                        None => true,
                        Some(i) => i.is_empty(),
                    });
            }

            // 2 b TODO reallocate index when quarter full???

            self.d -= 1;
            self.len_last_super -= 1;

            if self.len_last_super == 0 {
                self.s -= 1;
                
                if self.s % 2 == 0 {
                    self.cap_last_super /= 2;
                } else {
                    self.cap_last_data /= 2;
                }

                self.len_last_super = self.cap_last_super;
            }

            self.len_last_data = self.cap_last_data;

        }
        
        return ret
    }

    fn locate(index: usize) -> (usize, usize) {
        let index = index + 1;

        let k = std::mem::size_of::<usize>()*8 - index.leading_zeros() as usize - 1; // size of index - 1

        let l = (k + 1) / 2; // ceil(k/2)

        let b = (index & !(1 << k)) >> l; // get the first floor(k/2) bits of index after the leading One
        // remember that there are k bits after the leading 1 and that we have to cut off the last ceil(k/2) bits

        let e = index & ((1 << l) - 1); // get the last ceil(k/2) bits of index

        // There is an error in the paper. The number of data blocks in super blocks prior to SB[k] is not 2^k - 1, since an SB[i] has 2^floor(i/2) data blocks, not 2^i
        let p = 2*((1 << l)-1)
            - (k % 2) * (
                (1 << l)/2
            );

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
