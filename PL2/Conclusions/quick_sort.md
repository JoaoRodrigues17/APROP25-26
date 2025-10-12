# Parallel Quick Sort comparison


| Version                    | Size 50.000.000 | Size 5.000.000 | Size 500.000 |
|----------------------------|-----------------|----------------|--------------|
| Sequential                 | 14.796          |   1.407        |   0.136      |
| Parallel                   | 15.108          |   1.342        |   0.100      |

So, for greater array sizes, the parallel version proves less efficient, while smaller array sizes have a slight improvement