use core::time::Duration;

use criterion::Criterion;
use criterion::PlotConfiguration;
use criterion::AxisScale;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;

use slate::element::DrawFn;
use slate::scaffold::Scaffold;
use slate::element::tests::ElementTestImpl;

//---
fn bench_scaffold_composition(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaffold_composition");
    
    group.sample_size(100);
    group.measurement_time(Duration::from_secs(10));
    group.warm_up_time(Duration::from_secs(2));
    group.nresamples(20000);
    group.significance_level(0.01);
    group.noise_threshold(0.02);
    group.confidence_level(0.95);
    group.plot_config({
        PlotConfiguration::default()
            .summary_scale(AxisScale::Logarithmic)
    });
    
    group.bench_function("scaffold_composition::baseline", |b| b.iter(|| baseline(black_box(1000))));
    group.bench_function("scaffold_composition::twofer", |b| b.iter(|| twofer(black_box(1000))));
    group.bench_function("scaffold_composition::heavy_styles", |b| b.iter(|| heavy_styles(black_box(1000))));
    group.bench_function("scaffold_composition::deep_nesting", |b| b.iter(|| deep_nesting(black_box(1000))));
}

/// TODO
fn baseline(n: u64) {
    #[cfg(feature = "bump")]
    let mut arena = slate::arena::Bump::new();
    
    let draw_fn: DrawFn = chizel::uix! {
        <ElementTestImpl name="" />
    };
    
    for _ in 0..n {
        Scaffold::try_from_draw_fn(#[cfg(feature = "bump")] &arena, draw_fn)
            .expect("scaffold");
    
        #[cfg(feature = "bump")]
        arena.reset();
    }
}

//---
/// TODO
fn twofer(n: u64) {
    #[cfg(feature = "bump")]
    let mut arena = slate::arena::Bump::new();
    
    let draw_fn: DrawFn = chizel::uix! {
        <ElementTestImpl name="" />
        <ElementTestImpl name="" />
    };
    
    for _ in 0..n {
        Scaffold::try_from_draw_fn(#[cfg(feature = "bump")] &arena, draw_fn)
            .expect("scaffold");
    
        #[cfg(feature = "bump")]
        arena.reset();
    }
}

/// TODO
fn heavy_styles(n: u64) {
    #[cfg(feature = "bump")]
    let mut arena = slate::arena::Bump::new();
    
    let draw_fn: DrawFn = chizel::uix! {
        #[style(BackgroundColor::hex("#ff0000"))]
        #[style(Margin::all(0., 0., 0., 0.))]
        #[style(Padding::all(0., 0., 0., 0.))]
        #[style(BoxSize::xy(100., 100.))]
        #[style(BorderWeight::all(1., 1., 1., 1.))]
        <ElementTestImpl name="" />
    };
    
    for _ in 0..n {
        Scaffold::try_from_draw_fn(#[cfg(feature = "bump")] &arena, draw_fn)
            .expect("completed scaffold");
        
        #[cfg(feature = "bump")]
        arena.reset();
    }
}

/// TODO
fn deep_nesting(n: u64) {
    #[cfg(feature = "bump")]
    let mut arena = slate::arena::Bump::new();
    
    let draw_fn: DrawFn = chizel::uix! {
        // #[on(Click, on_click_fn)]
        <ElementTestImpl name="" number=0usize>
            <ElementTestImpl name="" number=3>
                <ElementTestImpl name="" number=0usize>
                    <ElementTestImpl name="" number=3>
                        <ElementTestImpl name="" number=3>
                            <ElementTestImpl name="" number=3>
                                <ElementTestImpl name="" number=3>
                                    <ElementTestImpl name="" number=3 />
                                    <ElementTestImpl name="" number=3 />
                                    <ElementTestImpl name="" number=3 />
                                </ElementTestImpl>
                            </ElementTestImpl>
                        </ElementTestImpl>
                    </ElementTestImpl>
                </ElementTestImpl>
            </ElementTestImpl>
        </ElementTestImpl>
    };
    
    for _ in 0..n {
        Scaffold::try_from_draw_fn(#[cfg(feature = "bump")] &arena, draw_fn)
            .expect("completed scaffold");
        
        #[cfg(feature = "bump")]
        arena.reset();
    }
}

criterion_group!(
    scaffold_composition,
    bench_scaffold_composition,
);

criterion_main!(
    scaffold_composition,
);
