## Comprehensive Benchmark Suite for Hash-based Sketches

### Motivation

Understand trade-offs between different sketch implementations for real-world workloads.

### Objective

Build a comprehensive benchmark harness that compares hash-based sketches across multiple dimensions:
- Memory usage (bytes per element)
- Query latency (lookup, insertion, deletion)
- False positive rates vs. theoretical bounds
- Throughput under concurrent workloads
- Cache efficiency

### Core Components

1. **Membership estimat**
    - Bloom filters vs. Blocked Bloom filters
    - Cuckoo filters vs. Quotient filters
    - XOR filters comparison
    - Test with various load factors and fingerprint sizes

2. **Sketch Comparison Suite**
    - Count-Min Sketch vs. Count Sketch
    - Conservative update variants
    - Different hash function families

3. **Cardinality Estimators**
    - HyperLogLog vs. HyperLogLog++
    - Different precision parameters
    - Bias correction impact

4. **Workload Scenarios**
    - Uniform random inserts
    - Zipfian distributions (realistic workloads)
    - Time-series patterns
    - Adversarial patterns

### Expected Outcomes

- Deep understanding of space-accuracy trade-offs
- Performance profiling and optimization in Rust
- Statistical analysis of probabilistic guarantees
- Memory layout optimization and cache awareness
- Benchmark design methodology

### Key References

**Books:**
  - **Medjedovic & Tahirovic (2022).** "Algorithms and Data Structures for Massive Datasets" - Manning Publications
  - **Gakhov (2019).** "Probabilistic Data Structures and Algorithms for Big Data Applications" - Wiley

**Papers:**
  - "Xor Filters: Faster and Smaller Than Bloom and Cuckoo Filters" (Arxiv 2019) - https://arxiv.org/pdf/1912.08258
  - "Cuckoo Filter: Practically Better Than Bloom" (CoNEXT 2014) - https://www.cs.cmu.edu/~dga/papers/cuckoo-conext2014.pdf
  - "Morton Filters: Faster, Space-Efficient Cuckoo Filters" (VLDB 2018)
  - **Bloom, B. H. (1970).** "Space/time trade-offs in hash coding with allowable errors" - Communications of the ACM
  - **Putze et al. (2007).** "Cache-, Hash- and Space-Efficient Bloom Filters" - Journal of Experimental Algorithmics
  - **Cormode, G. & Muthukrishnan, S. (2005).** "An Improved Data Stream Summary: The Count-Min Sketch and its Applications" - Journal of Algorithms
  - **Flajolet, P. et al. (2007).** "HyperLogLog: the analysis of a near-optimal cardinality estimation algorithm" - AOFA Conference
  - **Apache Parquet** - Split Block Bloom Filter specification

**Existing Implementations:**
  - DataSketches project benchmarking methodology
  - Rust crates: `probabilistic-collections`, `streaming_algorithms`, `bloomfilter`

**Online Resources:**
  - [Quint Documentation](https://quint-lang.org/docs/) - Formal specification language
  - [Apache DataSketches](https://datasketches.apache.org/) - Reference implementations
  - [Count-Min Sketch Explained](https://florian.github.io/count-min-sketch/) - Interactive visualization
  - [HyperLogLog in Practice](https://research.google/pubs/pub40671/) - Google's practical analysis

