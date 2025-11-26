/// Basic Bloom Filter Usage Example
///
/// Demonstrates:
/// - Creating a Bloom filter
/// - Inserting items
/// - Querying membership
/// - Understanding false positives
use sketches::filters::bloom::BloomFilter;
use sketches::filters::traits::ApproximateMembershipQuery;
use sketches::hashing::AHasher;

fn main() {
    println!("=== Basic Bloom Filter Example ===\n");

    // Create a Bloom filter for 1000 items with 1% false positive rate
    let capacity = 1000;
    let false_positive_rate = 0.01;
    let mut filter = BloomFilter::<_, AHasher>::new(capacity, false_positive_rate);

    println!("Created Bloom filter:");
    println!("  Capacity: {}", filter.capacity());
    println!("  Target FPR: {:.2}%", filter.false_positive_rate() * 100.0);
    println!();

    // Insert some programming languages
    let languages = vec![
        "Rust",
        "Python",
        "JavaScript",
        "Go",
        "TypeScript",
        "Java",
        "C++",
        "C",
        "Ruby",
        "Swift",
    ];

    println!("Inserting {} programming languages...", languages.len());
    for lang in &languages {
        filter.insert(lang);
    }
    println!("Filter now contains {} items\n", filter.len());

    // Test membership for inserted items (no false negatives)
    println!("Testing inserted items (should all be found):");
    for lang in &languages {
        let found = filter.contains(lang);
        println!(
            "  '{}': {}",
            lang,
            if found { "✓ found" } else { "✗ NOT found" }
        );
        assert!(found, "False negative! This should never happen.");
    }
    println!();

    // Test membership for non-inserted items (might have false positives)
    let non_inserted = vec![
        "Haskell", "OCaml", "Erlang", "Elixir", "Clojure", "Scala", "Kotlin", "Dart", "Lua", "Perl",
    ];

    println!("Testing non-inserted items (might have false positives):");
    let mut false_positives = 0;
    for lang in &non_inserted {
        let found = filter.contains(lang);
        if found {
            println!("  '{}': ⚠ false positive", lang);
            false_positives += 1;
        } else {
            println!("  '{}': ✓ correctly identified as absent", lang);
        }
    }

    let empirical_fpr = false_positives as f64 / non_inserted.len() as f64;
    println!();
    println!("False positive analysis:");
    println!("  Expected FPR: {:.2}%", false_positive_rate * 100.0);
    println!(
        "  Empirical FPR: {:.2}% ({}/{})",
        empirical_fpr * 100.0,
        false_positives,
        non_inserted.len()
    );
}
