#![allow(unused)]

use std::time::Duration;
use criterion::{Criterion, PlotConfiguration, AxisScale, black_box, criterion_group, criterion_main};
use slab::Slab;
use bumpalo::Bump;
use std::boxed::Box;

// TreeNode for Box-based allocation (heap)
struct BoxTreeNode {
    value: usize,
    style: String,
    event_callback: fn(),  // Simulated function pointer for event callback
    left: Option<Box<BoxTreeNode>>,
    right: Option<Box<BoxTreeNode>>,
}

// TreeNode for Slab-based allocation (slab)
struct SlabTreeNode {
    value: usize,
    style: String,
    event_callback: fn(),
    left: Option<usize>,   // References child nodes by their index in the Slab
    right: Option<usize>,
}

// TreeNode for Bumpalo-based allocation (bump)
struct BumpTreeNode<'a> {
    value: usize,
    style: &'a str,
    event_callback: fn(),
    left: Option<&'a BumpTreeNode<'a>>,
    right: Option<&'a BumpTreeNode<'a>>,
}

// A simple placeholder callback function
fn sample_callback() {}

// Helper function to recursively build a balanced binary tree using Box
fn build_tree_box(depth: usize, value: usize) -> Option<Box<BoxTreeNode>> {
    if depth == 0 {
        return None;
    }

    Some(Box::new(BoxTreeNode {
        value,
        style: "default-style".to_string(),  // Allocating a String on the heap
        event_callback: sample_callback,
        left: build_tree_box(depth - 1, value * 2),
        right: build_tree_box(depth - 1, value * 2 + 1),
    }))
}

// Helper function to recursively build a balanced binary tree using Slab
fn build_tree_slab(slab: &mut Slab<SlabTreeNode>, depth: usize, value: usize) -> Option<usize> {
    if depth == 0 {
        return None;
    }

    let node = slab.insert(SlabTreeNode {
        value,
        style: "default-style".to_string(),  // Allocating a String in Slab
        event_callback: sample_callback,
        left: None,
        right: None,
    });

    slab[node].left = build_tree_slab(slab, depth - 1, value * 2);
    slab[node].right = build_tree_slab(slab, depth - 1, value * 2 + 1);

    Some(node)
}

// Helper function to recursively build a balanced binary tree using Bumpalo
fn build_tree_bump<'a>(bump: &'a Bump, depth: usize, value: usize) -> Option<&'a BumpTreeNode<'a>> {
    if depth == 0 {
        return None;
    }

    Some(bump.alloc(BumpTreeNode {
        value,
        style: "default-style",  // Bumpalo allocates a &str (avoiding heap allocation)
        event_callback: sample_callback,
        left: build_tree_bump(bump, depth - 1, value * 2),
        right: build_tree_bump(bump, depth - 1, value * 2 + 1),
    }))
}

// Benchmark function for different allocation strategies
fn bench_allocation_strategy(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("allocation_strategy");
    const TREE_DEPTH: usize = 16;  // Depth of 16 for realistic complex structures
    const NUM_PASSES: usize = 5;   // Running multiple passes to observe the behavior across all allocators
    
    group.sample_size(50);  // Reduced sample size for better benchmarking speed
    group.measurement_time(Duration::from_secs(15));  // Increased measurement time for complex trees
    group.warm_up_time(Duration::from_secs(2));
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    // Benchmark standard Box-based allocation with multiple passes
    group.bench_function("allocation::box", |bench| {
        bench.iter(|| {
            for _ in 0..NUM_PASSES {
                let tree = build_tree_box(TREE_DEPTH, black_box(1));
                black_box(tree);
            }
        });
    });

    // Benchmark Slab-based allocation with multiple passes
    group.bench_function("allocation::slab", |bench| {
        bench.iter(|| {
            for _ in 0..NUM_PASSES {
                let mut slab = Slab::new();
                let tree = build_tree_slab(&mut slab, TREE_DEPTH, black_box(1));
                black_box(tree);
                slab.clear();  // Clear slab between passes
            }
        });
    });

    // Benchmark Bumpalo-based allocation with multiple passes
    group.bench_function("allocation::bumpalo", |bench| {
        bench.iter(|| {
            let mut bump = Bump::new();
            for _ in 0..NUM_PASSES {  // Run multiple passes to observe behavior after resetting
                let tree = build_tree_bump(&bump, TREE_DEPTH, black_box(1));
                black_box(tree);
                bump.reset();  // Reset the bump allocator for the next pass
            }
        });
    });
}

criterion_group!(allocation_strategy, bench_allocation_strategy);
criterion_main!(allocation_strategy);
