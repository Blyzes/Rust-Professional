/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
where
    T: Ord + Copy,
{
    //TODO
    if array.len() <= 1 {
        return;
    }

    let flag = array[0];
    let mut low = 0;
    let mut high = array.len() - 1;
    while low < high {
        while low < high && array[high] >= flag {   // first move high, because if move low first, you don't know place the val where.
            high -= 1;
        }
        array[low] = array[high];
        
        while low < high && array[low] < flag {
            low += 1;
        }
        array[high] = array[low];
    }
    array[low] = flag;
    sort(&mut array[0..low]);
    sort(&mut array[low+1..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
