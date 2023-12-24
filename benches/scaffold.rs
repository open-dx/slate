use std::time::Duration;

use criterion::Criterion;
use criterion::PlotConfiguration;
use criterion::AxisScale;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;

use slate::scaffold::Scaffold;
use slate::element::tests::ElementTestImpl;

//---
fn bench_main(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaffold_composition");
    {
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
    }
    
    group.bench_function("baseline", |b| b.iter(|| baseline(black_box(1000))));
    group.bench_function("twofer", |b| b.iter(|| twofer(black_box(1000))));
    group.bench_function("heavy_styles", |b| b.iter(|| heavy_styles(black_box(1000))));
    group.bench_function("deep_nesting", |b| b.iter(|| deep_nesting(black_box(1000))));
}

/// TODO
fn baseline(n: u64) {
    for _ in 0..n {
        let _scaffold = Scaffold::try_from_draw_fn({
            chizel::uix! {
                <ElementTestImpl name="" />
            }
        });
    }
}

//---
/// TODO
fn twofer(n: u64) {
    for _ in 0..n {
        let _scaffold = Scaffold::try_from_draw_fn({
            chizel::uix! {
                <ElementTestImpl name="" />
                <ElementTestImpl name="" />
            }
        });
    }
}

/// TODO
fn heavy_styles(n: u64) {
    for _ in 0..n {
        let _scaffold = Scaffold::try_from_draw_fn({
            chizel::uix! {
                #[style(BackgroundColor::hex("#ff0000"))]
                #[style(Margin::all(0., 0., 0., 0.))]
                #[style(Padding::all(0., 0., 0., 0.))]
                #[style(BoxSize::xy(100., 100.))]
                #[style(BorderWeight::all(1., 1., 1., 1.))]
                // #[class(some_class_name)]
                <ElementTestImpl name="" />
            }
        });
    }
}

/// TODO
fn deep_nesting(n: u64) {
    for _ in 0..n {
        let _scaffold = Scaffold::try_from_draw_fn({
            chizel::uix! {
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
            }
        });
    }
}

//---
criterion_group!(benches, bench_main);
criterion_main!(benches);
